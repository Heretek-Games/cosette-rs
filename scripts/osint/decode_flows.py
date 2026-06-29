#!/usr/bin/env python3
"""Decode captured takasho flows against vendored protos.

Reads a mitmproxy .flow dump, classifies each request by URL path,
attempts to decode request/response bodies with the matching .proto
descriptor, and emits one JSON file per flow.

Decode strategy:
  1. Match URL path against known proto prefixes in cosette-protocol.
  2. If a candidate proto is found, decode with protoc --decode_raw
     (we don't have a compiled descriptor at runtime; this gives
     a best-effort raw parse).
  3. If decode fails, mark decode_status="missing-proto" so the
     inventory in docs/osint/endpoints.md can flag it.

This script is intentionally read-only and stateless. It writes
JSON samples; humans decide whether to add missing protos.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import uuid
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
PROTO_ROOT = REPO_ROOT / "cosette-protocol" / "third_party" / "protos"
OUT_DEFAULT = REPO_ROOT / "docs" / "osint" / "decoded"

# Heuristic: takasho paths look like
#   /takasho.schema.<service>.<resource>/<Method>
# Map the leading proto package to a candidate .proto file under
# third_party/protos/takasho/schema/<service>/.
TAKASHO_PATH_RE = re.compile(r"^/takasho\.schema\.([^.]+)\.([^/]+)/")


def find_proto_candidate(path: str) -> Path | None:
    m = TAKASHO_PATH_RE.match(path)
    if not m:
        return None
    service, resource = m.group(1), m.group(2)
    candidate = PROTO_ROOT / "takasho" / "schema" / service / f"{resource}.proto"
    return candidate if candidate.exists() else None


def decode_raw(body: bytes, proto_path: Path) -> dict | None:
    """Best-effort raw decode using protoc --decode_raw.

    Returns parsed dict on success, None on failure.
    """
    if not body:
        return None
    try:
        proc = subprocess.run(
            ["protoc", "--decode_raw"],
            input=body,
            capture_output=True,
            timeout=5,
            check=False,
        )
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return None
    if proc.returncode != 0:
        return None
    return {"textproto": proc.stdout.decode("utf-8", errors="replace")}


def parse_flow_dump(flow_path: Path) -> list[dict]:
    """Read a mitmproxy .flow dump. Returns one dict per flow."""
    tmp_har = Path("/tmp") / f"decode_flows_{uuid.uuid4()}.har"
    try:
        subprocess.run(
            ["mitmdump", "-nr", str(flow_path), "--set", f"hardump={tmp_har}"],
            check=True,
            capture_output=True,
            timeout=60,
        )
        with tmp_har.open() as f:
            har = json.load(f)
        return har.get("log", {}).get("entries", [])
    finally:
        tmp_har.unlink(missing_ok=True)


def emit_one(entry: dict, out_dir: Path) -> dict:
    req = entry.get("request", {})
    resp = entry.get("response", {})
    url = req.get("url", "")
    path = req.get("path", "")
    if not path:
        from urllib.parse import urlparse

        path = urlparse(url).path

    candidate = find_proto_candidate(path)

    req_body_b64 = ""
    if req.get("postData"):
        req_body_b64 = "".join(
            p.get("text", "") for p in req["postData"].get("params", [])
        )
    resp_body_b64 = resp.get("content", {}).get("text", "")

    decoded_req = None
    decoded_resp = None
    decode_status = "no-proto-match"
    if candidate is not None:
        decode_status = "decode-attempted"
        try:
            import base64

            decoded_req = decode_raw(
                base64.b64decode(req_body_b64) if req_body_b64 else b"",
                candidate,
            )
            decoded_resp = decode_raw(
                base64.b64decode(resp_body_b64) if resp_body_b64 else b"",
                candidate,
            )
            if decoded_req is None and decoded_resp is None:
                decode_status = "decode-empty"
        except Exception:
            decode_status = "decode-error"

    flow_id = entry.get("serverIPAddress", uuid.uuid4().hex)
    sample = {
        "flow_id": flow_id,
        "url": url,
        "method": req.get("method", ""),
        "status": resp.get("status", 0),
        "proto_candidate": str(candidate.relative_to(REPO_ROOT)) if candidate else None,
        "decode_status": decode_status,
        "decoded_request": decoded_req,
        "decoded_response": decoded_resp,
    }

    out_path = out_dir / f"{uuid.uuid4().hex}.json"
    out_path.write_text(json.dumps(sample, indent=2))
    return sample


def main() -> int:
    p = argparse.ArgumentParser(description="Decode captured takasho flows.")
    p.add_argument("flow_file", type=Path)
    p.add_argument("--limit", type=int, default=50)
    p.add_argument("--out-dir", type=Path, default=OUT_DEFAULT)
    args = p.parse_args()

    if not args.flow_file.exists():
        print(f"decode_flows: file not found: {args.flow_file}", file=sys.stderr)
        return 1

    args.out_dir.mkdir(parents=True, exist_ok=True)

    entries = parse_flow_dump(args.flow_file)
    print(f"decode_flows: {len(entries)} flows in {args.flow_file}")

    summary = {"total": len(entries), "missing_proto": 0, "decoded": 0}
    for entry in entries[: args.limit]:
        sample = emit_one(entry, args.out_dir)
        if sample["proto_candidate"] is None:
            summary["missing_proto"] += 1
        else:
            summary["decoded"] += 1

    print(f"decode_flows: {summary}")
    print(f"decode_flows: samples in {args.out_dir}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
