# cosette-osint-toolchain — Design

**Date:** 2026-06-29
**Status:** Draft for review
**Project:** cosette-rs (Rust reimplementation of `takt op.` takasho gRPC backend)
**Scope:** OSINT tooling layer only. Server implementation lives in later specs.

---

## Context (why this design exists)

`cosette-rs` is a Rust workspace (`cosette-protocol`, `cosette-dispatchserver`, `cosette-gameserver`, `cosette-sdkserver`, `common`) that aims to reimplement the gRPC backend for the Android game **`takt op.`** (Takasho, package `com.dgames.g65002002.google`, version 1.2.217 build 3371). The protocol crate is generated via `tonic-build` from `.proto` files under `takasho/schema/{score,takt,common_featureset,...}` — already 30+ resource categories typed. Server crates are stub `main.rs` files. A 1.1 GB XAPK (split into 25+ `config.*.apk` Unity Asset Bundles) and a 7.5 GB device data archive sit at `review/`. A mitmproxy traffic DB (`mitm_mcp_traffic.db`) exists with the right schema but zero captured flows.

The reverse-engineering goal is **full game-server reimplementation** (fidelity-focused), not just a protocol map. To get there we need an OSINT layer that can:

- Bootstrap traffic capture from a real device into the existing DB.
- Decode captured gRPC flows against vendored protos and surface messages whose protos we don't yet have.
- Mine the APK (manifest, smali, asset bundles) for endpoints, permissions, and configuration.
- Feed the resulting proto patches and message samples into `cosette-protocol` without coupling OSINT tooling to server internals.

This document specifies that OSINT layer.

## Goals

1. Make `cosette-protocol` buildable offline from vendored protos (no `/home/yohk4e/Downloads/proto/protos` hardcoded dependency).
2. Install the native toolchain needed for APK analysis + gRPC probing + Unity asset extraction.
3. Bootstrap one full capture → decode → inventory cycle end-to-end.
4. Keep the OSINT layer read-only with respect to the live takasho server: it observes what a real device does, never constructs authenticated requests of its own.

## Non-goals (explicit)

- Server-side reimplementation of any endpoint. Out of scope for this spec; lives in later specs.
- Dynamic instrumentation / Frida-based TLS pinning bypass. Deferred until pinning actually blocks a capture.
- gRPC reflection against the live takasho server (assumed disabled; fall back to traffic-derived introspection).
- CI integration of any of this. Manual scripts only.
- Asset bundle translation or master-data injection. Extraction only — semantics are a later problem.

---

## Architecture

Three concentric layers, inwards = the Rust server project, outwards = the OSINT tools that feed it.

```
┌──────────────────────────────────────────────────────────────────────┐
│  EXTERNAL (outside repo)                                             │
│    • Android device/emulator (mobile-mcp installed)                  │
│    • APK + splits at cosette-rs/review/                              │
│    • mitmproxy instance (mitmproxy-mcp installed)                    │
└──────────────────────────────┬───────────────────────────────────────┘
                               │ captures
┌──────────────────────────────▼───────────────────────────────────────┐
│  OSINT TOOLCHAIN (this design)                                       │
│    • Native: jadx, apktool, grpcurl, UnityPy, protoc                 │
│    • Vendored: cosette-protocol/third_party/protos/                  │
│    • DB: mitm_mcp_traffic.db (existing)                              │
│    • Out: cosette-rs/docs/osint/  (decoded samples, inventory, notes)│
└──────────────────────────────┬───────────────────────────────────────┘
                               │ decoded protos, decoded traffic
┌──────────────────────────────▼───────────────────────────────────────┐
│  cosette-rs (existing)                                               │
│    cosette-protocol/   — generated protos; gains new messages        │
│    {dispatch,game,sdk}server/ — stubs; one subsystem per later spec  │
│    common/            — shared types                                 │
└──────────────────────────────────────────────────────────────────────┘
```

**Key boundary:** the OSINT layer never writes Rust. It produces a `.proto` patch + a decoded-message sample. A human (or a future planning session) writes the corresponding protocol struct and server handler. This keeps the tooling re-runnable without coupling.

The protocol crate's existing design pattern (`tonic` + `prost`, generated types) makes this natural — every captured gRPC message can be re-emitted as a `.proto` patch and gets free type safety + free client stubs.

---

## Components

Eight concrete additions, ordered so each phase produces a verifiable artifact before the next starts.

| # | Component | What it is | Verifiable output |
|---|-----------|------------|-------------------|
| 1 | Vendored proto tree | Copy `protos/` from the external `/home/yohk4e/Downloads/proto/protos` into `cosette-protocol/third_party/protos/`. Single commit, attributed to original source. | `git ls-files cosette-protocol/third_party/protos` shows the tree |
| 2 | `build.rs` path refactor | Replace hardcoded path with `env::var("CARGO_MANIFEST_PROTO_ROOT")` defaulting to `cosette-protocol/third_party/protos/`. Replace the existing panic with: `panic!("proto root not found at `{}` (override via CARGO_MANIFEST_PROTO_ROOT)", proto_root.display())`. | `cargo build -p cosette-protocol` succeeds offline |
| 3 | Native toolchain | `brew install jadx apktool grpcurl protobuf` (the `protobuf` formula ships `protoc`; `grpcurl` ships its own gRPC server reflection support) + `pipx install UnityPy`. Frida is NOT installed — out of scope unless needed. | `which jadx apktool grpcurl protoc` all resolve; `python3 -m UnityPy` resolves |
| 4 | Capture script | `scripts/osint/capture.sh` — wraps `mitmdump` with takasho-domain scope; emits flows as a `.flow` dump file. A second helper, `scripts/osint/import_flows.sh`, calls `mcp__mitmproxy-mcp__load_traffic_file` (or its CLI equivalent) to append into `mitm_mcp_traffic.db`. Two scripts keeps mitmdump unaware of the DB schema. | `./scripts/osint/capture.sh --out capture.flow --duration 30m` exits 0; `./scripts/osint/import_flows.sh capture.flow` exits 0; DB row count grows |
| 5 | Decode helper | `scripts/osint/decode_flows.py` — reads flows, classifies by path prefix, decodes known protos from `cosette-protocol`, falls back to raw protobuf hexdump. Emits JSON samples. | `python decode_flows.py --limit 20` produces `docs/osint/decoded/login.json` and similar |
| 6 | APK extraction helper | `scripts/osint/extract_apk.sh` — runs `apktool` on `base.apk` + each `config.*.apk`, `jadx` on `base.apk`, UnityPy on `install_time_asset_pack.apk`. Continues past per-split failures. | `docs/osint/apk/base/manifest.xml`, `base/smali/`, `install_time_asset_pack/assets/` populated |
| 7 | Endpoint inventory | `docs/osint/endpoints.md` — incrementally-populated table: method, URL, gRPC service/method, request type, response type, status | First row after one capture session |
| 8 | Capture notes / decoded samples | `docs/osint/decoded/<flow_name>.json` + `docs/osint/captures/<session>.md` | First note after one capture session |

### Phased rollout

| Phase | Tasks | Output |
|-------|-------|--------|
| **Phase 0 — Stabilize the build** | 1, 2 | `cargo build -p cosette-protocol` succeeds offline against vendored protos. Highest-risk debt (hardcoded external path) closed first. |
| **Phase 1 — Toolchain ready** | 3 | All native binaries on `PATH`. |
| **Phase 2 — First capture cycle** | 4, 5, 6, 7, 8 | At least login → main-menu loop captured, decoded, and inventoried; one row per distinct flow. |

### YAGNI guardrails

- **Frida:** skip unless dynamic instrumentation becomes necessary (likely needed if anti-tamper is hit). Deferred.
- **DB schema extensions:** no new columns on `mitm_mcp_traffic.db` until a specific need surfaces. Current schema (`flows` table with id, url, method, status, headers, body, timestamp, size) covers basics.
- **No CI:** this is exploration, not PR validation. Keep scripts manual-runnable.

---

## Data flow

```
┌────────────────────┐  HTTPS    ┌─────────────────┐  gRPC+TLS   ┌──────────────────────┐
│  Android device    │──────────▶│  mitmdump       │────────────▶│  takasho live server │
│  takt op. v1.2.217 │◀──────────│  (filter: takasho│◀────────────│  (production)        │
│  mobile-mcp pilot  │           │   domains only) │             │                      │
└────────────────────┘           └────────┬────────┘             └──────────────────────┘
                                          │ writes
                                          ▼
                              ┌──────────────────────────┐
                              │ mitm_mcp_traffic.db      │
                              │ (flows table)            │
                              └────────┬─────────────────┘
                                       │ read by
                                       ▼
                              ┌──────────────────────────┐
                              │ decode_flows.py          │
                              │ • classify paths         │
                              │ • decode body via protos │
                              │   from cosette-protocol/ │
                              │ • emit per-flow JSON     │
                              └────────┬─────────────────┘
                                       │ writes
                                       ▼
                              ┌──────────────────────────────────────────┐
                              │ docs/osint/decoded/<flow>.json           │
                              │ docs/osint/endpoints.md  ← manual edits   │
                              │ docs/osint/captures/<session>.md (notes) │
                              └──────────────────────────────────────────┘
                                       │ informs
                                       ▼
                              ┌──────────────────────────┐
                              │ cosette-protocol/        │
                              │ • add missing .proto     │
                              │ • re-run tonic_build     │
                              │ • new typed structs      │
                              └──────────────────────────┘
```

### Three flow-control details

1. **Per-session namespace.** Each capture session gets a `captures/<YYYY-MM-DD-HHMM>.md` file (human notes) plus the rows that landed in `mitm_mcp_traffic.db` during that window. The DB is append-only — we never delete flows. A `session_id` column gets added in a future iteration if/when we need to slice the DB by session; not added now.
2. **Decode match strategy.** `decode_flows.py` tries known protos first (the `takasho/schema/...` resource paths in `cosette-protocol`), falls back to raw protobuf hexdump if no match. **Missing protos surface as a TODO list** in `endpoints.md`, not as silent failures. This is the inventory driver.
3. **No writes back to the live server.** The capture script is read-only — it observes. Server-side testing happens later, when `cosette-dispatchserver/`, `cosette-gameserver/`, and `cosette-sdkserver/` have at least one flow implemented. Until then, the server crates stay stubbed.

---

## Error handling

Detection first, response second — order matters for what to instrument.

| Failure mode | How we detect | How we respond |
|---|---|---|
| **Vendor protos incomplete / moved** | `cargo build -p cosette-protocol` fails on first panic in `build.rs` | `build.rs` already has `panic!("proto root not found")` — extend to print both the resolved path and the `CARGO_MANIFEST_PROTO_ROOT` value so misconfig is obvious. (Concrete wording pinned in Components row 2.) |
| **APK split / AAB variant breaks apktool** | `apktool d` exits non-zero with "unsupported resource" or similar | Log the failing split, continue with the rest, emit `docs/osint/apk/_FAILED_SPLITS.md`. Don't abort the whole extraction. |
| **Device TLS pinning kills mitm** | Capture session produces 0 flows OR app throws "connection not secure" errors visible in `adb logcat` | Two-stage fallback: (a) try `adb shell settings put global http_proxy …` first; (b) if pinned, document in `captures/<session>.md` and skip that run — don't fight it manually. Frida bypass is opt-in future work. |
| **gRPC reflection disabled on prod** | `grpcurl -plaintext takasho.example.com list` returns "server does not support reflection" | Fall back to traffic-derived introspection: every distinct URL/path becomes an inventory row in `endpoints.md`. |
| **Captured flow body won't decode** | `decode_flows.py` matches zero candidates for the body bytes | Emit `docs/osint/decoded/<flow>_raw_hex.bin` + an empty entry in `endpoints.md` flagged `# missing-proto`. Inventory driver. |
| **UnityPy version mismatch on asset bundle** | `UnityPy.exceptions.InvalidSerializedFileException` | Wrap in try/except in `extract_apk.sh`. Continue; note unparseable bundle names in `docs/osint/apk/_UNITY_BUNDLE_NOTES.md`. |
| **Auth token replay by accident** | (Future, when server crates implement any auth flow) | Out of scope. Boundary holds: OSINT tooling captures what the real device does; never constructs authenticated requests. Replay-capable tooling belongs to a later "server testing" layer. |

### Logging

Every script writes to stdout (human) AND a `docs/osint/captures/<session>.log` file (machine). That's how future analysis tools (and you) figure out why a capture produced 12 flows when it should have produced 1200.

### Failure this layer CAN'T catch

The takasho backend shipping a new scheme that breaks `tonic`'s generated types at compile time. That's a `cosette-protocol` problem, not an OSINT problem — and it'll show up as a build failure, which is the desired signal.

---

## Testing

Four smoke checks, one per phase. No frameworks, no fixtures, no per-function suites — this is an exploration layer.

| # | Check | Script | Pass = |
|---|-------|--------|--------|
| 1 | **Build smoke** (Phase 0) | `scripts/osint/smoke_build.sh` — runs `cargo build -p cosette-protocol` | Exits 0, no `proto root not found` panic |
| 2 | **Native tools smoke** (Phase 1) | `scripts/osint/smoke_tools.sh` — runs `which jadx apktool grpcurl UnityPy` and prints versions | All four resolve; exit 0 |
| 3 | **Capture smoke** (Phase 2 mid) | `scripts/osint/smoke_capture.sh` — runs mitmdump for 30s against `takasho.com:443`, asserts `flows` row count > 0 | Exits 0 with DB row count printed |
| 4 | **End-to-end smoke** (Phase 2 close) | `scripts/osint/smoke_e2e.sh` — boots capture, drives the device through "launch game → reach main menu → log out" via mobile-mcp, runs `decode_flows.py`, asserts `endpoints.md` gained at least 3 new rows | New rows in `endpoints.md`; exit 0 |

These four scripts ARE the test suite — they're also the documentation for how the layer works.

### What we deliberately do NOT test

- gRPC message-by-message field-level correctness — that's `cosette-protocol`'s job, using `prost`'s built-in assertions.
- Server behavior under attack — out of scope, separate "server testing" layer later.
- UI behavior of the game — we don't drive the game past login.
- Performance — exploration layer, perf is irrelevant until we know the shape.

### Smoke-check failure response

A failing smoke is a 30-second debug, not a triage meeting. The fix is usually one of: wrong `PATH`, wrong proto root, device not connected, certificate not installed on device.

---

## Open questions (defer until hit, don't pre-answer)

1. Do we need to capture **login only**, or also account-creation / first-run onboarding flows? Phase 2 will pick the simpler one (login) first; expand if needed.
2. Region-server differences (JP vs global)? APK is Google-Play build (`com.dgames.g65002002.google`); the canonical server is JP takasho. Other regions are out of scope until the JP one is fully captured.
3. Master-data format (`master_data`, `ondemand_master` protos are already in `cosette-protocol`) — does the live server return schema'd data or has its own dialect? Decide once we have the first decoded master-data response in hand.
4. Auth scheme — token shape, rotation cadence, and device-bound vs session-bound. Hold off until we capture a login flow.

---

## Risks

- **Vendor protos may be incomplete** vs what the live server actually speaks. Mitigated by the decode-failure-as-inventory-entry rule (Section: Error handling).
- **Unity Asset Bundle format may have changed** in v1.2.217 vs what UnityPy supports. Mitigated by per-bundle try/except + `_UNITY_BUNDLE_NOTES.md`.
- **Capture device availability** — physical device needed for `mobile-mcp`-driven smoke. If unavailable, smokes 3 and 4 must wait. Smoke 1 and 2 don't depend on hardware.
- **TLS pinning** if takasho enforces it on this build. Documented fallback; Frida deferred.
- **Hardcoded proto path was a personal-machine shortcut.** Vendor + env-overridable build.rs removes that fragility.

---

## Success criteria for THIS design

This design is "done" when:

1. `cargo build -p cosette-protocol` succeeds offline against vendored protos.
2. `which jadx apktool grpcurl` all resolve and `python3 -m UnityPy` resolves.
3. A device can launch `takt op.` with mitm proxy in between, and `mitm_mcp_traffic.db` gains flows.
4. `decode_flows.py` produces at least one decoded JSON sample per distinct endpoint touched.
5. `docs/osint/endpoints.md` has at least one row per touched endpoint, with the missing-proto flag set where applicable.

After Phase 2 ships, this spec is closed and a follow-up spec handles "implement the first server flow end-to-end."