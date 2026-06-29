#!/usr/bin/env bash
# Smoke test: flow_to_pcap converts mitm .flow -> pcap with valid magic.
# Pass = exit 0. Uses a brief synthetic capture.

set -uo pipefail

cd "$(git rev-parse --show-toplevel)"

PCAP_OUT="$(mktemp --suffix=.pcap)"
FLOW_IN="$(mktemp --suffix=.flow)"
trap 'rm -f "$PCAP_OUT" "$FLOW_IN"' EXIT

if ! command -v mitmdump >/dev/null 2>&1; then
    echo "[smoke_flow_to_pcap] SKIP: mitmdump not on PATH" >&2
    exit 0
fi

# 5s capture to produce a .flow with at least CONNECT entries
./scripts/osint/capture.sh --out "$FLOW_IN" --duration 5 \
    --listen-port 18080 --scope 'example\.com' >/dev/null 2>&1 &
sleep 2
curl -sS --max-time 2 -x http://127.0.0.1:18080 http://example.com >/dev/null 2>&1 || true
sleep 4
wait 2>/dev/null || true

if [[ ! -s "$FLOW_IN" ]]; then
    echo "[smoke_flow_to_pcap] FAIL: empty .flow" >&2
    exit 1
fi

python3 scripts/osint/flow_to_pcap.py "$FLOW_IN" "$PCAP_OUT" >/dev/null
if [[ ! -s "$PCAP_OUT" ]]; then
    echo "[smoke_flow_to_pcap] FAIL: empty pcap" >&2
    exit 1
fi

# Validate libpcap magic (little-endian 0xA1B2C3D4)
MAGIC=$(od -An -tx1 -N4 "$PCAP_OUT" | tr -d ' \n')
if [[ "$MAGIC" != "a1b2c3d4" ]]; then
    echo "[smoke_flow_to_pcap] FAIL: bad magic ($MAGIC)" >&2
    exit 1
fi

echo "[smoke_flow_to_pcap] PASS"
