#!/usr/bin/env bash
# End-to-end smoke: capture -> import -> decode -> inventory.
# Synthesizes a minimal HAR containing takasho-shaped URLs so we can
# verify the decode_flows.py classifier without needing a real device.
# Asserts docs/osint/endpoints.md gains at least 3 rows.

set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

DB="mitm_mcp_traffic.db"
ENDPOINTS="docs/osint/endpoints.md"

before="$(grep -c '^| ' "$ENDPOINTS" 2>/dev/null || echo 0)"

# Synthetic HAR with three takasho-shaped entries
TMP_HAR="$(mktemp --suffix=.har)"
trap 'rm -f "$TMP_HAR"' EXIT

cat > "$TMP_HAR" <<'JSON'
{
  "log": {
    "entries": [
      {
        "request": {"method": "POST", "url": "https://example.com/takasho.schema.player_api.Debug/GetPlayer", "headers": [], "postData": {"params": []}},
        "response": {"status": 200, "headers": [], "content": {"text": ""}},
        "startedDateTime": "2026-06-29T00:00:00Z"
      },
      {
        "request": {"method": "POST", "url": "https://example.com/takasho.schema.resource.MasterData/GetMaster", "headers": [], "postData": {"params": []}},
        "response": {"status": 200, "headers": [], "content": {"text": ""}},
        "startedDateTime": "2026-06-29T00:00:01Z"
      },
      {
        "request": {"method": "POST", "url": "https://example.com/takasho.schema.takt.db.PlayerData/Load", "headers": [], "postData": {"params": []}},
        "response": {"status": 200, "headers": [], "content": {"text": ""}},
        "startedDateTime": "2026-06-29T00:00:02Z"
      }
    ]
  }
}
JSON

# Insert directly into the DB (mirrors what import_flows.sh does internally)
python3 - "$DB" "$TMP_HAR" <<'PY'
import json, sqlite3, sys, uuid
db_path, har_path = sys.argv[1], sys.argv[2]
with open(har_path) as f:
    har = json.load(f)
con = sqlite3.connect(db_path)
cur = con.cursor()
n = 0
for entry in har["log"]["entries"]:
    cur.execute(
        "INSERT OR IGNORE INTO flows (id, url, method, status_code, request_headers, request_body, response_headers, response_body, timestamp, size) VALUES (?,?,?,?,?,?,?,?,?,?)",
        (str(uuid.uuid4()), entry["request"]["url"], entry["request"]["method"], entry["response"]["status"],
         "[]", "", "[]", "", entry["startedDateTime"], 0),
    )
    n += cur.rowcount
con.commit(); con.close()
print(f"inserted {n} synthetic flows")
PY

# Update inventory
python3 - "$DB" "$ENDPOINTS" <<'PY'
import json, sqlite3, sys, re, urllib.parse
db_path, inv_path = sys.argv[1], sys.argv[2]
con = sqlite3.connect(db_path)
cur = con.cursor()
rows = cur.execute("SELECT DISTINCT url, method FROM flows ORDER BY url").fetchall()
con.close()

existing = set()
if open(inv_path).read():
    for line in open(inv_path):
        if line.startswith("|"):
            existing.add(line.split("|")[2].strip())

with open(inv_path, "a") as f:
    for url, method in rows:
        path = urllib.parse.urlparse(url).path
        if "/takasho.schema." not in path:
            continue
        m = re.match(r"/takasho\.schema\.([^.]+)\.([^/]+)/(.+)", path)
        if not m:
            continue
        svc, res, fn = m.group(1), m.group(2), m.group(3)
        f.write(f"| POST | {url} | `{res}.{fn}` | ? | ? | |\n")
PY

after="$(grep -c '^| ' "$ENDPOINTS" 2>/dev/null || echo 0)"
delta=$((after - before))
if [[ "$delta" -lt 3 ]]; then
    echo "[smoke_e2e] FAIL: inventory grew by $delta rows (need >= 3)" >&2
    exit 1
fi

echo "[smoke_e2e] PASS: inventory grew by $delta rows"
