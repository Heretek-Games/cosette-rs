#!/usr/bin/env bash
# Wrap mitmdump to capture takasho game traffic.
# Writes mitmproxy's native .flow dump. Use import_flows.sh to load into
# mitm_mcp_traffic.db.

set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

OUT=""
DURATION=1800
PORT=8080
SCOPE='takasho\.com|takasho\.jp|dgames\.|takt-op'

while [[ $# -gt 0 ]]; do
    case "$1" in
        --out) OUT="$2"; shift 2 ;;
        --duration) DURATION="$2"; shift 2 ;;
        --listen-port) PORT="$2"; shift 2 ;;
        --scope) SCOPE="$2"; shift 2 ;;
        *) echo "unknown arg: $1" >&2; exit 2 ;;
    esac
done

if [[ -z "$OUT" ]]; then
    echo "usage: $0 --out <file.flow> [--duration SEC] [--listen-port PORT] [--scope REGEX]" >&2
    exit 2
fi

if ! command -v mitmdump >/dev/null 2>&1; then
    echo "mitmdump not found. Install with: pipx install mitmproxy" >&2
    exit 1
fi

mkdir -p "$(dirname "$OUT")"

echo "[capture] starting mitmdump on :$PORT, scope='$SCOPE', duration=${DURATION}s"
echo "[capture] writing to: $OUT"

# --set confdir picks up any pre-installed CA cert
# --save-stream-file streams flows to disk as they're captured
mitmdump \
    --listen-port "$PORT" \
    --set "confdir=$HOME/.mitmproxy" \
    --save-stream-file "$OUT" \
    --set "block_global=false" \
    --anticache \
    &
MITM_PID=$!

trap 'kill "$MITM_PID" 2>/dev/null || true' EXIT

sleep "$DURATION"

echo "[capture] duration elapsed; stopping mitmdump"
kill "$MITM_PID" 2>/dev/null || true
wait "$MITM_PID" 2>/dev/null || true

# Verify the file has at least one flow
if [[ ! -s "$OUT" ]]; then
    echo "[capture] FAIL: output file is empty" >&2
    exit 1
fi

echo "[capture] PASS: $OUT written ($(stat -c %s "$OUT") bytes)"