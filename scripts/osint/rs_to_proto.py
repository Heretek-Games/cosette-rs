#!/usr/bin/env python3
"""Reverse-engineer .proto files from cosette-protocol/include/takasho/schema/.

Walks the generated Rust tree, parses `#[prost(TYPE, ..., tag = "N")]`
attributes + `pub struct Foo { pub field: Type }` lines, emits .proto files
mirroring the directory structure under
cosette-protocol/third_party/protos/takasho/schema/.

Field tags are extracted from the `tag = "N"` attribute — the existing
generated Rust preserves them, so the emitted .proto reproduces the
original wire format (modulo type-mapping edge cases).

Usage:
    python3 scripts/osint/rs_to_proto.py [--src PATH] [--out PATH]

Defaults: --src=cosette-protocol/include/takasho/schema
          --out=cosette-protocol/third_party/protos/takasho/schema
"""

from __future__ import annotations

import argparse
import logging
import re
import sys
from pathlib import Path


# Map prost Rust types to proto2/proto3 types.
PROST_TYPE_MAP = {
    "i32": "int32",
    "i64": "int64",
    "u32": "uint32",
    "u64": "uint64",
    "f32": "float",
    "f64": "double",
    "bool": "bool",
    "String": "string",
    "Vec<u8>": "bytes",
}

# A repeated Vec<T> is "repeated T".
RE_VEC_TYPE = re.compile(r"^Vec<(.+)>$")
# An Option<T> is optional T.
RE_OPTION_TYPE = re.compile(r"^Option<(.+)>$")

# Attribute shape: #[prost(TYPE, [modifier, ...] tag = "N")]
# Examples: #[prost(string, tag = "1")]
#           #[prost(message, optional, tag = "2")]
#           #[prost(message, repeated, tag = "10")]
ATTR_RE = re.compile(
    r'#\[prost\((?P<type>[^,)]+)(?P<modifiers>[^)]*)\s+tag\s*=\s*"(?P<tag>\d+)"\s*\)\]'
)
FIELD_RE = re.compile(r"^\s*pub\s+(?P<name>\w+)\s*:\s*(?P<type>.+?)(?:,\s*)?$")
STRUCT_RE = re.compile(
    r"^\s*pub\s+struct\s+(?P<name>\w+)\s*(?P<body>\{.*\})\s*$", re.DOTALL
)
MOD_RE = re.compile(r"^\s*pub\s+mod\s+(?P<name>\w+)\s*\{")
PACKAGE_PATH_RE = re.compile(r"takasho/schema/(.+)\.rs$")

logging.basicConfig(level=logging.INFO, format="%(levelname)s %(message)s")
log = logging.getLogger("rs_to_proto")


def map_type(rust_type: str, modifiers: list[str]) -> tuple[str, str]:
    """Map a Rust prost type + modifiers to a proto field line fragment.

    Returns (proto_type, label) where label is e.g. "repeated " or "".
    """
    rust_type = rust_type.strip()

    # Vec<T> => repeated T
    m = RE_VEC_TYPE.match(rust_type)
    if m:
        inner = m.group(1).strip()
        inner_proto = PROST_TYPE_MAP.get(inner, inner)
        return inner_proto, "repeated "

    # Option<T> => optional T (label = "" because proto3 default is optional)
    m = RE_OPTION_TYPE.match(rust_type)
    if m:
        inner = m.group(1).strip()
        inner_proto = PROST_TYPE_MAP.get(inner, inner)
        return inner_proto, ""

    # Plain type
    if rust_type in PROST_TYPE_MAP:
        return PROST_TYPE_MAP[rust_type], ""

    # Unknown — likely a user-defined message type (e.g.,
    # `player_api_file_option::GoServer`). These are already in proto form
    # (Rust path syntax) — emit as-is without spamming warnings.
    if "::" in rust_type or (
        rust_type and rust_type[0].isupper() and rust_type not in PROST_TYPE_MAP
    ):
        return rust_type, ""
    log.warning(f"unknown type: {rust_type!r} (emitting unchanged)")
    return rust_type, ""


def parse_attrs(attr_line: str) -> tuple[str, list[str], int] | None:
    """Parse a `#[prost(...)]` attribute line."""
    m = ATTR_RE.search(attr_line)
    if not m:
        return None
    type_str = m.group("type").strip()
    modifiers_str = m.group("modifiers").strip()
    modifiers = [m.strip() for m in modifiers_str.split(",") if m.strip()]
    tag = int(m.group("tag"))
    return type_str, modifiers, tag


def parse_struct_block(body: str) -> list[tuple[str, str, list[str], int]]:
    """Parse a struct body, returning list of (name, type, modifiers, tag)."""
    fields = []
    lines = body.splitlines()
    i = 0
    while i < len(lines):
        line = lines[i]
        # Look for the attribute line; field definition follows on the next non-empty line
        attr_match = ATTR_RE.search(line)
        if attr_match:
            # Look ahead for the field declaration
            j = i + 1
            while j < len(lines) and not lines[j].strip():
                j += 1
            if j < len(lines):
                field_line = lines[j]
                f_match = FIELD_RE.match(field_line)
                if f_match:
                    attr_type, modifiers, tag = parse_attrs(line)
                    rust_type = f_match.group("type").strip()
                    # Strip prost-generated absolute paths so PROST_TYPE_MAP
                    # lookups succeed for primitive Rust types. This must
                    # handle nested occurrences (e.g., inside Vec<T>).
                    # e.g. "::prost::alloc::string::String" -> "String"
                    # e.g. "::prost::alloc::vec::Vec<::prost::alloc::string::String>"
                    #   -> "Vec<String>"
                    rust_type = re.sub(r"::prost::alloc::\w+::", "", rust_type)
                    # Strip the fully-qualified Option wrapper path.
                    # e.g. "::core::option::Option<Foo>" -> "Option<Foo>"
                    rust_type = re.sub(r"^::core::option::", "", rust_type)
                    # When the rust_type is a generic wrapper like Vec<T> or
                    # Option<T> whose inner T is a primitive, use T directly
                    # so PROST_TYPE_MAP lookup succeeds below.
                    inner_match = re.match(r"^(?:Vec|Option)<(.+)>$", rust_type)
                    if inner_match:
                        candidate = inner_match.group(1).strip()
                        if candidate in PROST_TYPE_MAP:
                            rust_type = candidate
                    # When the attribute says "message" or "enumeration",
                    # the actual type is the user-defined Rust type from the
                    # field declaration (e.g., `player_api_file_option::GoServer`).
                    # For primitive attribute keywords (string, int32, ...),
                    # the canonical proto type comes from PROST_TYPE_MAP keyed
                    # on the Rust primitive (e.g., String, i64). When the
                    # rust_type resolves cleanly via PROST_TYPE_MAP, prefer it;
                    # otherwise fall back to the attribute keyword.
                    if attr_type in ("message", "enumeration"):
                        effective_type = rust_type
                    elif rust_type in PROST_TYPE_MAP:
                        effective_type = rust_type
                    else:
                        effective_type = attr_type
                    fields.append(
                        (
                            f_match.group("name"),
                            effective_type,
                            modifiers,
                            tag,
                        )
                    )
                i = j + 1
                continue
        i += 1
    return fields


def emit_message(name: str, fields: list[tuple[str, str, list[str], int]]) -> list[str]:
    """Emit proto lines for a single message."""
    lines = [f"message {name} {{"]
    for fname, ftype, fmods, ftag in fields:
        proto_type, label = map_type(ftype, fmods)
        # proto3: no `optional` label (default behavior)
        lines.append(f"  {label}{proto_type} {fname} = {ftag};")
    lines.append("}")
    return lines


def process_file(
    rs_path: Path, src_root: Path, out_root: Path
) -> tuple[int, list[str]]:
    """Process one .rs file; return (count of fields emitted, list of warnings)."""
    warnings = []
    rel = rs_path.relative_to(src_root)
    out_path = out_root / rel.with_suffix(".proto")

    text = rs_path.read_text(encoding="utf-8")

    # Find all top-level pub struct Foo { ... } blocks
    struct_blocks = []
    pos = 0
    while True:
        m = re.search(r"^pub\s+struct\s+(\w+)\s*\{", text[pos:], re.MULTILINE)
        if not m:
            break
        name = m.group(1)
        start = pos + m.end() - 1  # position of the '{'
        depth = 1
        i = start + 1
        while i < len(text) and depth > 0:
            if text[i] == "{":
                depth += 1
            elif text[i] == "}":
                depth -= 1
            i += 1
        if depth != 0:
            warnings.append(f"{rs_path}: unmatched braces in struct {name}")
            break
        body = text[start + 1 : i - 1]
        struct_blocks.append((name, body))
        pos = i

    if not struct_blocks:
        warnings.append(f"{rs_path}: no top-level pub struct found")
        return 0, warnings

    out_path.parent.mkdir(parents=True, exist_ok=True)

    # Compute proto package from path: takeasho/schema/<service>/<file>.rs
    # -> package takasho.schema.<service>;
    rel_str = str(rel.with_suffix(""))
    parts = rel_str.split("/")
    # parts = ["score", "player_api"] or ["takt", "db"] etc.
    # For top-level schema, parts = ["custom_option"] -> package takasho.schema.custom_option;
    package_parts = ["takasho", "schema"] + parts[:-1]
    package = ".".join(package_parts)

    field_count = 0
    out_lines = [
        'syntax = "proto3";',
        "",
        f"package {'.'.join(package_parts)};",
        "",
    ]

    # Add import for sibling schema files if the .rs references them
    # For now, emit imports lazily (when we see super:: references)
    imports_emitted = set()

    for sname, sbody in struct_blocks:
        fields = parse_struct_block(sbody)
        field_count += len(fields)
        # If the body references super::, emit an import
        if "super::" in sbody:
            # Heuristic: extract super::path prefix; emit as import
            for super_match in re.finditer(r"super::([\w:]+)", sbody):
                super_path = super_match.group(1).replace("::", "/")
                import_path = f"takasho/schema/{super_path}.proto"
                if import_path not in imports_emitted:
                    out_lines.insert(-1, "")
                    out_lines.insert(-1, f'import "{import_path}";')
                    imports_emitted.add(import_path)

        out_lines.extend(emit_message(sname, fields))
        out_lines.append("")

    out_path.write_text("\n".join(out_lines).rstrip() + "\n", encoding="utf-8")
    return field_count, warnings


def main() -> int:
    p = argparse.ArgumentParser(
        description="Reverse-engineer .proto from generated Rust"
    )
    p.add_argument(
        "--src", type=Path, default=Path("cosette-protocol/include/takasho/schema")
    )
    p.add_argument(
        "--out",
        type=Path,
        default=Path("cosette-protocol/third_party/protos/takasho/schema"),
    )
    args = p.parse_args()

    if not args.src.exists():
        log.error(f"source directory not found: {args.src}")
        return 1

    args.out.mkdir(parents=True, exist_ok=True)

    rs_files = sorted(args.src.rglob("*.rs"))
    if not rs_files:
        log.error(f"no .rs files under {args.src}")
        return 1

    total_files = 0
    total_fields = 0
    all_warnings = []

    for rs_path in rs_files:
        fields, warnings = process_file(rs_path, args.src, args.out)
        total_files += 1
        total_fields += fields
        all_warnings.extend(warnings)
        log.info(f"{rs_path.relative_to(args.src)} -> {fields} fields")

    log.info(f"emitted {total_files} .proto files with {total_fields} fields total")
    if all_warnings:
        log.warning(f"{len(all_warnings)} warnings:")
        for w in all_warnings:
            log.warning(f"  {w}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
