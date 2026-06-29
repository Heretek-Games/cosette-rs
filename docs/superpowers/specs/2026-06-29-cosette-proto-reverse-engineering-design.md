# cosette-proto-reverse-engineering — Design

**Date:** 2026-06-29
**Status:** Draft for review
**Project:** cosette-rs (Rust reimplementation of `takt op.` takasho gRPC backend)
**Scope:** Recover `.proto` source files for the takasho backend by reverse-engineering them from the existing generated Rust types in `cosette-protocol/include/takasho/schema/`. This unblocks `cargo build -p cosette-protocol` to succeed offline.

---

## Context (why this design exists)

The prior OSINT toolchain spec (`docs/superpowers/specs/2026-06-29-cosette-osint-toolchain-design.md`) shipped successfully (Phases 1-2), but Phase 0 (vendor protos + env-overridable `build.rs`) was deferred because the takasho proto source at `/home/yohk4e/Downloads/proto/protos` is not accessible from this machine.

The subsequent proto-extraction spec (`docs/superpowers/specs/2026-06-29-cosette-proto-extraction-design.md`) attempted to recover protos by mirroring Yoshk4e's extraction pipeline (OxidizedRelay + NisDecryptor + Frida + Il2CppDumper). That effort found three blockers:

1. **OxidizedRelay** takes libpcap input (mitigated by `scripts/osint/flow_to_pcap.py` shipped earlier).
2. **NisDecryptor** handles only Windows .NET DLLs, not Android IL2CPP.
3. **`global-metadata.dat`** is encrypted v29+; Perfare/Il2CppDumper doesn't handle it. djkaty fork doesn't exist. IroniaTheMaster/Descrypt-global-metadata.dat is a comparison tool, not a decryptor. Frida hook approach was never tried due to likely anti-tamper on `takt op. v1.2.217`.

The findings doc (`docs/osint/extraction-findings/2026-06-29-il2cpp-pipeline-notes.md`) recommends: **reverse-engineer `.proto` files from the EXISTING generated Rust types in `cosette-protocol/include/takasho/schema/`**. This is mechanical, deterministic, and bypasses all the toolchain incompatibilities we hit.

This document specifies that reverse-engineering pipeline.

**Critical observation during planning:** the existing generated Rust types preserve field tags as prost attributes — `#[prost(string, tag = "1")]`, `#[prost(message, repeated, tag = "10")]`, etc. **This means the script can extract REAL field tags, not just sequential placeholders.** The output `.proto` files will reproduce the original wire format (modulo type-mapping edge cases), and `cargo build -p cosette-protocol` will regenerate `include/` to Rust that matches the existing committed source.

## Goals

1. Reverse-engineer `.proto` files from the existing generated Rust types.
2. Mirror the directory structure: `include/takasho/schema/score/player_api.rs` → `third_party/protos/takasho/schema/score/player_api.proto`.
3. Verify that `cargo build -p cosette-protocol` succeeds offline (both `check` and `build` stages).
4. Document the recovery procedure + provenance in `third_party/protos/README.md`.

## Non-goals (explicit)

- **Wire-format round-trip with real traffic.** No traffic capture in scope. The recovery verifies "Rust → .proto → Rust again" is self-consistent, not that decoded messages match real captures.
- **Re-running of the OSINT toolchain or extraction pipeline.** Those are separate specs.
- **Server-side implementation** of any takasho endpoint. Out of scope.
- **Catching up on game version drift.** v1.2.217 only. Future versions need new captures.
- **Auto-detection of `oneof`, `map`, or unsupported types.** Script emits a warning and skips them. If they appear and are critical, document in the findings doc.

---

## Architecture

```
┌───────────────────────────────────────────────────────────────┐
│  EXISTING (input)                                               │
│    cosette-protocol/include/takasho/schema/**/*.rs            │
│    (30+ generated message types from prior protoc-gen run,    │
│     with #[prost(TYPE, ..., tag = "N")] attributes preserving  │
│     field tags)                                                │
└──────────────────────────────┬────────────────────────────────┘
                               │ reads
┌──────────────────────────────▼────────────────────────────────┐
│  scripts/osint/rs_to_proto.py (NEW, this design)               │
│    • Glob include/takasho/**/*.rs                               │
│    • For each file:                                             │
│      - Parse `pub mod name { ... }` module decls               │
│      - For each `pub struct Foo { pub field: Type }`:           │
│        * Extract #[prost(TYPE, ..., tag = "N")] → type + tag  │
│          + modifier (optional, repeated)                        │
│        * Map prost type → proto type                           │
│        * Emit `TYPE field_name = N [modifier];`                │
│      - Recurse into nested `pub mod foo { ... }`                │
│      - Resolve cross-module refs as `import` statements         │
│    • Write to third_party/protos/takasho/schema/<path>.proto    │
│    • Print summary: N protos emitted, N fields total            │
└──────────────────────────────┬────────────────────────────────┘
                               │ writes
┌──────────────────────────────▼────────────────────────────────┐
│  OUTPUT (new)                                                    │
│    cosette-protocol/third_party/protos/takasho/schema/...     │
│      *.proto  (>=30 files)                                     │
│    + README.md                                                  │
└───────────────────────────────────────────────────────────────┘
                               │ used by smoke
┌──────────────────────────────▼────────────────────────────────┐
│  scripts/osint/smoke_rs_to_proto.sh                            │
│    1. Run rs_to_proto.py                                       │
│    2. Assert ≥30 .proto files produced                         │
│    3. cargo check -p cosette-protocol  (fast type-check)        │
│    4. cargo build -p cosette-protocol  (full compile)          │
│    5. PASS / FAIL                                                │
└───────────────────────────────────────────────────────────────┘
```

---

## Components

Five concrete additions, ordered so each produces a verifiable artifact.

| # | Component | What it is | Verifiable output |
|---|-----------|------------|-------------------|
| 1 | **`scripts/osint/rs_to_proto.py`** | Python 3 script. Walks `cosette-protocol/include/takasho/**/*.rs`, parses `#[prost(TYPE, ..., tag = "N")]` attributes + `pub struct Foo { pub field: Type }` lines, emits `.proto` files into `cosette-protocol/third_party/protos/takasho/schema/.../`. Handles nested modules + cross-file imports. | Running it produces ≥30 `.proto` files; `find ... -name '*.proto' \| wc -l >= 30`. |
| 2 | **Third-party protos tree** | Output directory `cosette-protocol/third_party/protos/` (doesn't exist yet) plus `takasho/schema/.../*.proto` files inside it. Mirrors the directory structure of `include/takasho/schema/`. | Tree exists after script run; `ls cosette-protocol/third_party/protos/takasho/schema/` shows the same top-level structure as `include/takasho/schema/`. |
| 3 | **`cosette-protocol/third_party/protos/README.md`** | Provenance + license note. Documents: source = reverse-engineered from `include/takasho/schema/`, generator script = `scripts/osint/rs_to_proto.py`, license = Apache-2.0 (matches `cosette-protocol/Cargo.toml`). | File exists with the documented structure. |
| 4 | **`scripts/osint/smoke_rs_to_proto.sh`** | Smoke test. Runs rs_to_proto.py; asserts ≥30 `.proto` files; runs `cargo check -p cosette-protocol` then `cargo build -p cosette-protocol`; asserts both exit 0. | Smoke exits 0 with all assertions passing. |
| 5 | **Reconciliation note** | Append a section to `docs/osint/extraction-findings/2026-06-29-il2cpp-pipeline-notes.md` documenting that proto recovery is complete via reverse-engineering + what limitations remain. | Note appended; committed. |

### Phased rollout

| Phase | Tasks | Output |
|-------|-------|--------|
| **Phase 1 — Script** | 1, 2, 3 | rs_to_proto.py writes .proto files; tree exists; README in place. |
| **Phase 2 — Verification** | 4 | Smoke verifies both file count and `cargo check` + `cargo build` exit. |
| **Phase 3 — Documentation** | 5 | Findings doc updated to reflect completion. |

### YAGNI guardrails

- **No enum support yet.** If generated Rust has `enum Foo { ... }`, the script skips it with a warning. If actually present (needs checking), add support in a follow-up.
- **No `oneof` or `map` support.** Skip with warning if encountered.
- **No cross-crate validation.** The smoke verifies "Rust → .proto → Rust again" is self-consistent. It does NOT verify the regenerated Rust matches the committed `include/` byte-for-byte (that would require diffing, which is brittle across prost version changes).
- **No regenerating `cosette-protocol/include/`.** That's `cargo build`'s job. The smoke verifies it succeeds; it doesn't diff the output.

---

## Data flow

End-to-end: one script run produces vendored protos; the smoke verifies via `cargo check` (fast) then `cargo build` (full).

```
┌────────────────────────────────────────────────────────────┐
│  EXISTING committed source                                   │
│    cosette-protocol/include/takasho/schema/**/*.rs          │
│    (30+ generated message types, with #[prost(...)]        │
│     attributes preserving field tags)                        │
└──────────────────────────────┬─────────────────────────────┘
                               │ reads
┌──────────────────────────────▼─────────────────────────────┐
│  scripts/osint/rs_to_proto.py                                │
│                                                              │
│  1. Glob include/takasho/**/*.rs                            │
│  2. For each file:                                           │
│     a. Parse module decl `pub mod name { ... }`             │
│     b. For each `pub struct Foo { pub field: Type }`:       │
│        - Extract #[prost(TYPE, ..., tag = "N")] → type      │
│          + tag + modifier (optional, repeated)               │
│        - Map prost type → proto type:                        │
│          i32/i64 → int32/int64, f32/f64 → float/double,     │
│          bool → bool, String → string,                       │
│          Vec<u8> → bytes, Vec<T> → repeated T,               │
│          Option<T> → T (optional),                           │
│          enum → enum block                                   │
│        - Emit `TYPE field_name = N [modifier];`             │
│     c. Recurse into nested `pub mod foo { ... }`             │
│     d. Resolve cross-module refs as `import` statements      │
│  3. Write to third_party/protos/takasho/schema/             │
│           <mirror path>/<file>.proto                        │
│  4. Print summary: N protos emitted, N fields total         │
└──────────────────────────────┬─────────────────────────────┘
                               │ writes
┌──────────────────────────────▼─────────────────────────────┐
│  OUTPUT                                                       │
│    cosette-protocol/third_party/protos/takasho/schema/...    │
│      *.proto  (>=30 files)                                   │
│    + README.md                                                │
└─────────────────────────────────────────────────────────────┘
                               │ used by smoke
┌──────────────────────────────▼─────────────────────────────┐
│  scripts/osint/smoke_rs_to_proto.sh                          │
│                                                              │
│  1. Run rs_to_proto.py                                       │
│  2. Assert >=30 .proto files produced                         │
│  3. cargo check -p cosette-protocol  (fast type-check)       │
│  4. cargo build -p cosette-protocol  (full compile)          │
│  5. If both exit 0: PASS                                     │
│     Else: FAIL with the relevant cargo error                │
└─────────────────────────────────────────────────────────────┘
```

### Two flow-control details worth pinning

1. **Module path mapping.** Mirrors the directory structure: `include/takasho/schema/score/player_api.rs` → `third_party/protos/takasho/schema/score/player_api.proto`. Cross-file refs (like `super::super::resource_cn::system::v1::SystemInfo`) become `import "takasho/schema/score/resource_cn/system/v1.proto";`.
2. **Idempotent re-runs.** Re-running overwrites existing files. No deletion of stale files (if a `.rs` is removed from `include/`, the `.proto` lingers; manual cleanup needed). Document this in the README.

---

## Error handling

| Failure mode | How we detect | How we respond |
|---|---|---|
| **`include/takasho/schema/` doesn't exist** | Glob returns empty | Smoke FAIL with clear message ("Source tree missing — cosette-protocol must be checked out"). |
| **No `.rs` files match** | `len(parsed_files) == 0` | Smoke FAIL ("0 Rust source files found — is `cosette-protocol` vendored?"). |
| **Unsupported field type** (e.g., `HashMap<String, Foo>`) | Regex parser sees an unknown Rust type | Skip the field with a stderr warning; emit the rest of the message. |
| **Nested module is malformed** (parse failure on inner struct) | Python raises on regex match | Print file + line; skip the whole file with a warning. Other files still emit. |
| **Generated `.proto` has unresolved `import`** | `cargo check` fails with "file not found" | Smoke FAIL with cargo error. Root cause: cross-file path mapping is off — fix the path translation. |
| **`cargo check -p cosette-protocol` fails** | non-zero exit | Smoke FAIL with stderr from cargo. Likely: invalid `.proto` syntax → tonic-build errors. Iterate. |
| **`cargo build -p cosette-protocol` fails** (but check passes) | non-zero exit | Smoke FAIL with stderr. Likely: missing system dep (protobuf compiler). Print remediation. |
| **Coverage < 30 .proto files** | `find third_party/protos -name '*.proto' \| wc -l < 30` | Smoke FAIL with count. Some Rust files weren't parsed (warnings tell which). |

### Logging

Script writes to stdout (human) AND `~/.cache/rs_to_proto.log` (machine, append-only). Smoke writes to stdout only.

### One failure this layer CAN'T catch

Generated Rust types from `include/` may use features the script doesn't recognize (e.g., `bytes::Bytes`, custom serializer attributes). If those features are critical to the wire format, the resulting `.proto` will decode incorrectly even though it builds. This is a real limitation; document in the README + findings doc.

---

## Testing

| # | Check | Script | Pass = |
|---|-------|--------|--------|
| 1 | **Generation + build smoke** | `scripts/osint/smoke_rs_to_proto.sh` runs rs_to_proto.py; asserts ≥30 `.proto` files; runs `cargo check` then `cargo build`; asserts both exit 0. | Smoke exits 0 with all assertions passing. |

### What we deliberately do NOT test

- **Wire-format round-trip.** No traffic to decode against. The smoke verifies "Rust source → .proto → Rust again" is self-consistent; it does not verify "→ real binary traffic" matches. That comes later when a real capture is available.
- **Specific field semantics.** A field that means "player ID" in the original might emit as `string player_id = 1` — semantically correct, structurally correct. No semantic verification.
- **Performance.** Generation runs in seconds for 30+ schemas. Not worth measuring.

### Smoke-check failure response

A failing smoke is a 30-second debug. Likely root causes:
- Parser missed a Rust pattern → extend the regex.
- cargo failed → check the error, fix the `.proto`.

---

## Open questions (defer until hit, don't pre-answer)

1. **Are there any `enum` types in `include/takasho/`?** If yes, the script must handle them. Discover during implementation.
2. **Does the regenerated `include/` match the committed `include/` byte-for-byte?** It should, given tags are preserved. But prost may introduce minor formatting differences (e.g., line wrapping). Discover during smoke run.

---

## Risks

- **Field type mapping edge cases.** A Rust type like `chrono::DateTime<Utc>` may appear as `string` in generated Rust, but the original `.proto` might have used `google.protobuf.Timestamp`. Script can't detect this — it'll emit `string` and lose the timestamp semantics. Acceptable for now; document as a known limitation.
- **Cross-file import paths.** The path translation logic is non-trivial. Cross-file refs in generated Rust use relative paths like `super::super::resource_cn::system::v1::SystemInfo`. Translating these to `.proto` `import` statements requires careful path resolution.
- **Generated `include/` is overwritten on next `cargo build`.** Anyone depending on the current committed `include/` will see drift after this spec ships. Document this in the README + changelog.
- **Protobuf compiler not installed.** `cargo build` invokes `protoc` (already on PATH per prior plan's Task 4). If missing, smoke FAILs with clear error.

---

## Success criteria for THIS design

This design is "done" when:

1. `scripts/osint/rs_to_proto.py` runs without error and produces ≥30 `.proto` files under `cosette-protocol/third_party/protos/takasho/schema/`.
2. `scripts/osint/smoke_rs_to_proto.sh` exits 0 (assertions pass + `cargo check` + `cargo build` both succeed).
3. `cargo build -p cosette-protocol` succeeds offline (no network access needed).
4. `cosette-protocol/third_party/protos/README.md` documents the provenance.
5. `docs/osint/extraction-findings/2026-06-29-il2cpp-pipeline-notes.md` is updated to reflect completion.

After Phase 3 ships, this spec is closed. The deferred OSINT Phase 0 (env-overridable `build.rs`) was already shipped in the prior spec — it's already in place to consume these vendored protos.