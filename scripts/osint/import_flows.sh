#!/usr/bin/env bash
# Read a mitmproxy .flow dump and append rows to mitm_mcp_traffic.db.
# Uses mitmdump -nr to re-emit HAR, then a small Python helper to insert.

set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

FLOW_FILE="${1:-}"
DB_PATH="${MITM_DB:-./mitm_mcp_traffic.db}"

if [[ -z "$FLOW_FILE" ]]; then
    echo "usage: $0 <file.flow> [MITM_DB=./mitm_mcp_traffic.db]" >&2
    exit 2
fi

if [[ ! -s "$FLOW_FILE" ]]; then
    echo "import_flows: flow file empty or missing: $FLOW_FILE" >&2
    exit 1
fi

if [[ ! -f "$DB_PATH" ]]; then
    echo "import_flows: DB not found: $DB_PATH" >&2
    exit 1
fi

# Re-emit as HAR, then load with sqlite3
TMP_HAR="$(mktemp --suffix=.har)"
trap 'rm -f "$TMP_HAR"' EXIT

mitmdump -nr "$FLOW_FILE" --export-har "$TMP_HAR" >/dev/null 2>&1 || true

if [[ ! -s "$TMP_HAR" ]]; then
    echo "import_flows: mitmdump -export-har produced no output" >&2
    exit 1
fi

python3 - "$DB_PATH" "$TMP_HAR" <<'PY'
import json
import sqlite3
import sys
import uuid

db_path, har_path = sys.argv[1], sys.argv[2]
with open(har_path, "r", encoding="utf-8") as f:
    har = json.load(f)

con = sqlite3.connect(db_path)
cur = con.cursor()

inserted = 0
for entry in har.get("log", {}).get("entries", []):
    req = entry.get("request", {})
    resp = entry.get("response", {})
    flow_id = str(uuid.uuid4())
    url = req.get("url", "")
    method = req.get("method", "")
    status = resp.get("status", 0)
    req_headers = json.dumps(req.get("headers", []))
    resp_headers = json.dumps(resp.get("headers", []))
    req_body = "".join(
        part.get("text", "") for part in req.get("postData", {}).get("params", [])
    ) if req.get("postData") else ""
    resp_body_b64 = resp.get("content", {}).get("text", "")
    timestamp = entry.get("startedDateTime", "")
    size = resp.get("bodySize", 0)
    cur.execute(
        """INSERT OR IGNORE INTO flows
           (id, url, method, status_code, request_headers, request_body,
            response_headers, response_body, timestamp, size)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)""",
        (
            flow_id, url, method, status, req_headers, req_body,
            resp_headers, resp_body_b64, timestamp, size,
        ),
    )
    inserted += cur.rowcount

con.commit()
con.close()
print(f"import_flows: inserted {inserted} rows into {db_path}")
PY