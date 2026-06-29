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
# -s loads the scope-filter addon; --set scope_regex/scope_out configure it.
# The addon writes the .flow file itself (mitmdump's --save-stream-file has no
# regex filter, so we can't rely on it for scope enforcement).
mitmdump \
    --listen-port "$PORT" \
    --set "confdir=$HOME/.mitmproxy" \
    --set "block_global=false" \
    --anticache \
    -s "$(dirname "$0")/scope_addon.py" \
    --set "scope_out=$OUT" \
    --set "scope_regex=$SCOPE" \
    &
MITM_PID=$!

# Traps:
#   INT/TERM  - Ctrl-C or kill: stop mitmdump immediately and exit 130.
#               A bare `sleep "$DURATION"` ignores these until it returns
#               naturally, so we use a 1-second wake loop instead.
#   EXIT      - Always try to clean up mitmdump, regardless of how we exit.
cleanup() {
    if kill -0 "$MITM_PID" 2>/dev/null; then
        kill "$MITM_PID" 2>/dev/null || true
        wait "$MITM_PID" 2>/dev/null || true
    fi
}
on_signal() {
    cleanup
    exit 130
}
trap cleanup EXIT
trap on_signal INT TERM

# Wake-up loop: each iteration sleeps 1s, so Ctrl-C is honored within ~1s
# regardless of DURATION. Signal interrupts the inner `sleep` and `wait`
# returns non-zero — that's expected, ignore it.
remaining="$DURATION"
while (( remaining > 0 )); do
    # Rescue the wake-loop so the SIGINT trap's `on_signal` can run;
    # the trap exits 130 directly.
    sleep 1 || true
    remaining=$((remaining - 1))
done

echo "[capture] duration elapsed; stopping mitmdump"
kill "$MITM_PID" 2>/dev/null || true
wait "$MITM_PID" 2>/dev/null || true

# Verify the file has at least one flow
if [[ ! -s "$OUT" ]]; then
    echo "[capture] FAIL: output file is empty" >&2
    exit 1
fi

echo "[capture] PASS: $OUT written ($(stat -c %s "$OUT") bytes)"