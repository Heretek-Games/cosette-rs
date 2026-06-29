#!/usr/bin/env python3
"""Convert mitmproxy .flow dump -> .pcap readable by OxidizedRelay.

REAL OxidizedRelay CLI: `<PCAP> [-k HEX_KEY] [-i HEX_NONCE]` — takes libpcap
input. mitmproxy writes tnetstring-framed .flow. We bridge via:
  1. mitmdump -nr <flow> --set hardump=<HAR>   (re-emit as HAR)
  2. parse HAR, build minimal IPv4+TCP+payload frames
  3. emit a libpcap file (LINKTYPE_RAW) the parser will accept

This is a best-effort bridge; the ChaCha20-decrypt is the real work, which
OxidizedRelay handles once given a pcap + key.
"""
from __future__ import annotations

import argparse
import base64
import json
import os
import struct
import subprocess
import sys
import tempfile


def har_to_pcap(har_path: str, out_path: str) -> int:
    payloads: list[bytes] = []
    with open(har_path, "r", encoding="utf-8") as f:
        har = json.load(f)

    for entry in har.get("log", {}).get("entries", []):
        req = entry.get("request", {})
        resp = entry.get("response", {})

        # Request body
        rb = b""
        if req.get("postData", {}).get("text"):
            try:
                rb = base64.b64decode(req["postData"]["text"])
            except Exception:
                rb = b""

        # Response body
        sb = b""
        ct = resp.get("content", {}).get("text")
        if ct:
            try:
                sb = base64.b64decode(ct)
            except Exception:
                sb = b""

        if rb:
            payloads.append(rb)
        if sb:
            payloads.append(sb)

    # libpcap global header: magic, version 2.4, thiszone, sigfigs, snaplen, linktype
    # LINKTYPE_RAW (101) = raw IPv4/v6 frames
    PCAP_MAGIC = 0xA1B2C3D4
    global_header = struct.pack(
        "<IHHiIII",
        PCAP_MAGIC,
        2, 4,            # version major.minor
        0,               # thiszone
        0,               # sigfigs
        65535,           # snaplen
        101,             # linktype LINKTYPE_RAW
    )

    def tcp_wrap(payload: bytes) -> bytes:
        # Minimal IPv4 + TCP headers wrapping the payload.
        # We don't compute real checksums; OxidizedRelay only needs the payload
        # at the right place in the frame.
        tcp_hdr = struct.pack(
            ">HHIIBBBBBHH",
            40000, 443,           # src, dst ports
            1000, 1000,           # seq, ack
            0x50, 0x18,           # data offset, PSH|ACK flags
            0xFFFF, 0xFFFF,       # window
            0, 0,                 # checksum, urgent
        )
        ip_total = 20 + len(tcp_hdr) + len(payload)
        ip_hdr = struct.pack(
            ">BBHHHBBH4s4s",
            0x45, 0, ip_total,
            0, 0,
            64, 6, 0,           # TTL, proto=TCP, checksum=0
            b"\x7f\x00\x00\x01",  # src
            b"\x7f\x00\x00\x01",  # dst
        )
        return ip_hdr + tcp_hdr + payload

    def pcap_record(payload: bytes, idx: int) -> bytes:
        frame = tcp_wrap(payload)
        return struct.pack("<IIII", idx, 0, len(frame), len(frame)) + frame

    with open(out_path, "wb") as f:
        f.write(global_header)
        for i, p in enumerate(payloads):
            f.write(pcap_record(p, i))

    print(f"flow_to_pcap: wrote {len(payloads)} payloads to {out_path}")
    return 0


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("flow_file")
    p.add_argument("out_pcap")
    args = p.parse_args()
    if not os.path.exists(args.flow_file):
        print(f"flow_to_pcap: not found: {args.flow_file}", file=sys.stderr)
        return 1

    tmp_har = tempfile.mktemp(suffix=".har")
    proc = subprocess.run(
        ["mitmdump", "-nr", args.flow_file, "--set", f"hardump={tmp_har}"],
        capture_output=True, check=False,
    )
    if proc.returncode != 0 or not os.path.exists(tmp_har):
        print(f"flow_to_pcap: mitmdump re-emit failed", file=sys.stderr)
        return 1
    return har_to_pcap(tmp_har, args.out_pcap)


if __name__ == "__main__":
    sys.exit(main())
