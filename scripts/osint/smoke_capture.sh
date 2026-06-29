#!/usr/bin/env bash
# Smoke: capture.sh + import_flows.sh actually move data into the DB.
# Uses a brief local mitmdump + curl to /example.com if no device traffic
# is available, which exercises the full pipeline without needing a phone.

set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

DB="mitm_mcp_traffic.db"
FLOW="$(mktemp --suffix=.flow)"
trap 'rm -f "$FLOW"' EXIT

# Generate synthetic HTTPS traffic through mitmdump so we have flows to import.
# (We can't actually MITM arbitrary HTTPS without trusting mitmproxy's CA,
# but mitmdump records the CONNECT requests, which is enough to verify the
# pipeline moves rows into the DB.)
echo "[smoke_capture] starting short capture against example.com"
./scripts/osint/capture.sh --out "$FLOW" --duration 8 --listen-port 18080 \
    --scope 'example\.com' &
CAPTURE_PID=$!

# Wait for mitmdump to be ready, then hit it
for _ in $(seq 1 20); do
    if curl -sS -x http://127.0.0.1:18080 --max-time 2 http://example.com >/dev/null 2>&1; then
        break
    fi
    sleep 0.5
done

# Give capture.sh a moment to finalize the file
wait "$CAPTURE_PID" || true

if [[ ! -s "$FLOW" ]]; then
    echo "[smoke_capture] FAIL: capture.sh produced empty output" >&2
    exit 1
fi

before="$(sqlite3 "$DB" 'SELECT COUNT(*) FROM flows')"
./scripts/osint/import_flows.sh "$FLOW"
after="$(sqlite3 "$DB" 'SELECT COUNT(*) FROM flows')"

if [[ "$after" -le "$before" ]]; then
    echo "[smoke_capture] FAIL: no new rows inserted (before=$before after=$after)" >&2
    exit 1
fi

echo "[smoke_capture] PASS: $((after - before)) new rows inserted"