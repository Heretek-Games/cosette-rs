# cosette-proto-extraction — Design

**Date:** 2026-06-29
**Status:** Draft for review
**Project:** cosette-rs (Rust reimplementation of `takt op.` takasho gRPC backend)
**Scope:** Recover the takasho `.proto` source files by mirroring Yoshk4e's extraction pipeline (per the inventory of `Yoshk4e/NisDec`, `Yoshk4e/OxidizedRelay`, and `Yoshk4e/perlica-rs`). This unblocks the deferred Phase 0 of `docs/superpowers/specs/2026-06-29-cosette-osint-toolchain-design.md`.

---

## Context (why this design exists)

The prior OSINT toolchain spec (`docs/superpowers/specs/2026-06-29-cosette-osint-toolchain-design.md`) shipped Phases 1-2 successfully, but Phase 0 (vendor protos + env-overridable `build.rs`) was deferred because the takasho proto source at `/home/yohk4e/Downloads/proto/protos` is not accessible from this machine. The user asked: where else might the protos live?

**Inventory of `https://github.com/Yoshk4e`** (the user implied they wrote the original commits / had the protos):

- **`Yoshk4e/perlica-rs`** — Rust server reimpl for an "old version of a certain ani[me] game". Its `lib/proto/build.rs` references `perlica.proto` + `cmd_id.proto`, but `perlica-rs/.gitignore` excludes `*.proto`. The protos exist on Yoshk4e's local machine but are never committed.
- **`Yoshk4e/OxidizedRelay`** — "CLI tool for decrypting and parsing ChaCha20-encrypted protobuf anime[game traffic]". This is the runtime extraction tool: it decrypts ChaCha20-encrypted gRPC bodies and emits protobuf-shaped output.
- **`Yoshk4e/NisDec`** — "1.2.50 takt" C# tool. Has `Decryptor/` and `PeParser/`. Likely extracts proto descriptor strings from the Unity IL2CPP binary (`libil2cpp.so`).
- **No public gists, no public orgs**, Heretek-Games org has no public members.

**Conclusion:** the protos are *not* publicly downloadable. They live on Yoshk4e's local machine, produced by running the capture-and-decrypt pipeline. To recover them locally we must mirror that pipeline: Android device + mitm capture + ChaCha20 decrypt + IL2CPP binary parse + reconstruction into `.proto` files.

This document specifies that pipeline.

## Goals

1. Build Yoshk4e's tools (`OxidizedRelay`, `NisDec`) locally so they're available for the extraction run.
2. Drive a real Android device (or emulator) with `takt op. v1.2.217` installed, traffic captured through mitm, Frida attached for key extraction.
3. Run OxidizedRelay + NisDec against the captured session + APK binary to extract structured proto data.
4. Reconstruct `.proto` files matching the existing `cosette-protocol/include/takasho/schema/...` shape, into `cosette-protocol/third_party/protos/`.
5. Document the provenance + reconstruction procedure in `cosette-protocol/third_party/protos/README.md`.

## Non-goals (explicit)

- Server-side reimplementation (any of `cosette-dispatchserver/`, `cosette-gameserver/`, `cosette-sdkserver/`). Out of scope for this spec.
- Frida-based anti-tamper bypass. If the game detects Frida, document and stop; do NOT attempt bypass.
- Re-running the OSINT Phase 0 build.rs vendor step. That's a follow-up spec that depends on this one.
- Account creation / first-run onboarding capture. Login + reach-main-menu is the minimum session.
- Region-server differences. v1.2.217 (Google Play build, JP takasho backend) is the only target.

---

## Architecture

```
┌──────────────────────────────────────────────────────────────────────┐
│  EXTERNAL                                                             │
│    • Android device or emulator with takt op. v1.2.217 installed      │
│    • The 1.1 GB XAPK + 25+ config.*.apk splits (already in review/)  │
│    • mitmproxy instance (mitmproxy-mcp installed)                     │
└──────────────────────────────┬───────────────────────────────────────┘
                               │ captures
┌──────────────────────────────▼───────────────────────────────────────┐
│  EXTRACTION PIPELINE (this design)                                    │
│    • OxidizedRelay (Yoshk4e/OxidizedRelay) — ChaCha20 decrypt +       │
│      protobuf parser                                                  │
│    • NisDec (Yoshk4e/NisDec) — PE/IL2CPP extractor for game-binary    │
│      strings                                                          │
│    • Frida + custom hook script — dump session keys at runtime       │
│    • Existing scripts/osint/capture.sh + scope_addon.py (Phase 1-2)   │
│    • New: scripts/osint/extract_protos.sh — orchestrates the two      │
│      tools + Frida and emits proto files into third_party/protos/    │
└──────────────────────────────┬───────────────────────────────────────┘
                               │ vendored protos
┌──────────────────────────────▼───────────────────────────────────────┐
│  cosette-rs (existing)                                                │
│    cosette-protocol/third_party/protos/  ← output target              │
│    cosette-protocol/build.rs  ← already env-overridable (Phase 1-2)  │
└──────────────────────────────────────────────────────────────────────┘
```

**Two distinct extraction paths:**

1. **Traffic-derived (OxidizedRelay + Frida).** Capture mitm traffic → Frida dumps ChaCha20 session key → OxidizedRelay decrypts + parses → runtime proto messages (request/response shapes).
2. **Binary-derived (NisDec).** Parse the APK's IL2CPP binary → extract proto descriptor strings + cmd_id mappings → service definitions that traffic alone can't reveal.

The boundary between them is fuzzy (Yoshk4e's tooling may do both). The spec discovers the actual split during implementation. The `extract_protos.sh` orchestrator runs both paths in parallel and merges outputs.

**Hard rule:** capture uses a real device with a real account. No constructed traffic. Yoshk4e's tooling was built against real game sessions; we won't replicate it with synthetic data.

---

## Components

Nine concrete additions, ordered so each phase produces a verifiable artifact before the next starts.

| # | Component | What it is | Verifiable output |
|---|-----------|------------|-------------------|
| 1 | **OxidizedRelay clone + build** | `git clone https://github.com/Yoshk4e/OxidizedRelay /tmp/oxrel && cd /tmp/oxrel && cargo build --release`. Same-origin, public repo. | Binary at `/tmp/oxrel/target/release/oxrel` runs and prints `--help`. |
| 2 | **NisDec clone + build** | `git clone https://github.com/Yoshk4e/NisDec /tmp/nisdec && cd /tmp/nisdec && dotnet build -c Release`. C#/.NET — requires `dotnet` SDK on PATH (install via brew/linuxbrew if missing). | Binary at `/tmp/nisdec/bin/Release/net*/NisDec.dll` runs and prints help. |
| 3 | **Android device/emulator setup** | Either physical device with `takt op. v1.2.217` installed and debug-bridge enabled, OR Android Studio AVD (already installed via `/home/john/Android/Sdk/emulator`). Install APK via `adb install` or `mobile-mcp`. | `adb devices` lists an emulator or device with the package `com.dgames.g65002002.google`. |
| 4 | **Frida install** | `pipx install frida-tools` (provides `frida` CLI + Python bindings). Plus optional `frida-server` on device for non-rooted paths. | `frida --version` resolves. |
| 5 | **Frida hook development** | New `scripts/osint/frida_hook.js` — JavaScript hooking `chacha20_poly1305_*` (or equivalent) in `libil2cpp.so`, dumping keys to stdout when the game encrypts a request. May iterate on hook targets based on observed symbols. | `frida -U com.dgames.g65002002.google -l scripts/osint/frida_hook.js` emits at least one key during a 30s session. |
| 6 | **Capture orchestration** | Reuses existing `scripts/osint/capture.sh` (Phase 1-2) with `--scope` loosened. Output is a `.flow` dump via mitmproxy. | `.flow` file contains takasho-schema URLs after a 5-minute launch-and-reach-main-menu session. |
| 7 | **OxidizedRelay pass** | Pipes the .flow dump + Frida key file through OxidizedRelay to decrypt ChaCha20-encrypted bodies and emit protobuf-shaped output (`.pb` files keyed by URL or service). | `oxrel --in <flow> --key <keyfile> --out <dir>` produces a tree of `.pb` files; `protoc --decode_raw` on one parses cleanly. |
| 8 | **NisDec pass** | Runs NisDec against `libil2cpp.so` extracted from the APK split, captures class metadata + cmd_id mappings into a JSON/CSV inventory. | NisDec output lists ≥30 takasho schema names (matching the existing `cosette-protocol/include/takasho/schema/` resource count). |
| 9 | **Proto reconstruction** | New script `scripts/osint/extract_protos.sh` orchestrates 6-8 and merges outputs into a directory tree under `cosette-protocol/third_party/protos/takasho/...` shaped like the build.rs path expectation. | `./scripts/osint/extract_protos.sh` exits 0; `cosette-protocol/third_party/protos/takasho/` contains the reconstructed tree. |

### Phased rollout

| Phase | Tasks | Output |
|-------|-------|--------|
| **Phase A — Tooling** | 1, 2, 4 | OxidizedRelay + NisDec + Frida built locally. All binaries run. |
| **Phase B — Device + capture** | 3, 6 | Real device launches takt op. with mitm in between. .flow file contains takasho traffic. |
| **Phase B.5 — Frida hook** | 5 | Hook script dumps ChaCha20 session keys during a 30s probe session. |
| **Phase C — Extraction** | 7, 8 | Two parallel extraction runs produce structured output. |
| **Phase D — Reconstruction** | 9 | Protos vendored into `cosette-protocol/third_party/protos/`; smoke green. |

### YAGNI guardrails

- **No Frida anti-tamper bypass.** If the game detects Frida, document and stop. The boundary is: Yoshk4e's tooling was built against real game sessions, possibly with Frida on rooted devices; if v1.2.217 detects Frida, that's a finding, not a blocker to fix.
- **No private fork of upstream tools.** Don't fork `OxidizedRelay` or `NisDec`. Use upstream `main`. If a bug blocks progress, document it and either submit upstream or work around it locally.
- **No release artifacts.** `cargo build` / `dotnet build` from source. If upstream publishes releases later, switch then.
- **No schema guessing.** If coverage < 30 services, do NOT vendor partial protos. The orchestrator exits 2 (data-quality failure) and the artifacts stay in `/tmp/extract/<session>/proto/` for inspection.
- **No account reuse.** Use a fresh Google account for the capture session. The game may ban accounts that exhibit Frida behavior.

---

## Data flow

```
┌────────────────────┐  HTTPS    ┌─────────────────┐  gRPC+TLS   ┌──────────────────────┐
│  Android device    │──────────▶│  mitmdump       │────────────▶│  takasho live server │
│  takt op. v1.2.217 │◀──────────│  scripts/osint/ │◀────────────│  (ChaCha20 encrypted │
│  Frida attached    │           │  capture.sh     │             │   on the wire)       │
└────────────────────┘           └────────┬────────┘             └──────────────────────┘
                                          │ writes                        ▲
                                          ▼                                │ Frida hook
                              ┌──────────────────────────┐                  │ dumps key
                              │  capture.flow            │                  │ at runtime
                              │  (mitmproxy .flow dump) │                  │
                              └────────┬─────────────────┘                  │
                                       │                                 │
                                       ▼                                 │
                              ┌──────────────────────────┐◀────────────────┘
                              │  OxidizedRelay           │  key file
                              │  • reads .flow           │
                              │  • ChaCha20-decrypt      │
                              │  • parse protobuf        │
                              │  • emit .pb per request  │
                              └────────┬─────────────────┘
                                       │ writes
                                       ▼
                              ┌──────────────────────────────────┐
                              │  .pb tree  keyed by URL/service │
                              │  /tmp/extract/<session>/proto/  │
                              └────────┬─────────────────────────┘
                                       │
                                       │ NisDec
                                       ▼
                              ┌──────────────────────────────────┐
                              │  NisDec on libil2cpp.so         │
                              │  • extract class metadata       │
                              │  • extract cmd_id enum          │
                              │  • emit service registry .json  │
                              └────────┬─────────────────────────┘
                                       │
                                       ▼
                              ┌──────────────────────────────────────────┐
                              │  scripts/osint/extract_protos.sh        │
                              │  • merge .pb (request/response shapes)  │
                              │  • merge .json (service/cmd_id)         │
                              │  • emit .proto files into               │
                              │    cosette-protocol/third_party/protos/ │
                              │      takasho/schema/<svc>/<res>.proto   │
                              └──────────────────────────────────────────┘
```

### Three flow-control details

1. **Per-session directory.** Each extraction session creates `/tmp/extract/<YYYY-MM-DD-HHMM>/` containing raw .pb, raw NisDec output, and the merged protos. The mitm DB and `docs/osint/` are not touched (this is a one-shot extraction, not a continuous capture loop).
2. **Frida gate.** Frida runs first (Phase B.5). If the game crashes / detects Frida / produces no key dump, OxidizedRelay's decryption fails loudly. The orchestrator documents this in its exit code; no automatic fallback to "decrypt without key" (that path doesn't exist).
3. **Coverage assertion.** After extraction, count distinct `.proto` files in the merged output. If <30 (matching the existing schema count from `cosette-protocol/include/takasho/schema/`), flag the run as incomplete and don't overwrite the (currently empty) `third_party/protos/` tree.

---

## Error handling

| Failure mode | How we detect | How we respond |
|---|---|---|
| **OxidizedRelay build fails** | `cargo build --release` exits non-zero | Print cargo error verbatim. Fix dependency mismatch (Rust toolchain older? Pin via `rust-toolchain.toml`?). |
| **NisDec build fails** | `dotnet build -c Release` exits non-zero | Print dotnet error. Install correct SDK version (likely .NET 6 or 8 LTS). |
| **Android emulator fails to start** | `adb devices` empty after 60s | Try alternate AVD; if no AVD exists, `avdmanager create avd -n takt -k "system-images;android-30;google_apis;x86_64"`. |
| **APK install fails** | `adb install` exits non-zero | Log `adb logcat` for INSTALL_FAILED_* codes. Likely cause: signature mismatch with installed app → `adb uninstall com.dgames.g65002002.google` first. |
| **mitm capture produces 0 flows** | `.flow` file empty after session | Document in extract session log. Verify mitmproxy CA cert installed on device: `adb shell ls /system/etc/security/cacerts/` should contain `mitmproxy-*`. |
| **Frida attach fails** | `frida -U com.dgames.g65002002.google` errors with "unable to attach" | Game has anti-Frida. Document; do NOT attempt bypass in this spec. Failure cascades to (key-dump failure). |
| **Frida hook produces no key** | Hook script runs but no `key_dump` event in output | Hook targets wrong function. Try alternate hook addresses (search for `chacha20_poly1305_*` symbols in libil2cpp.so via `nm` or `radare2`). If still no key, mark session as "no encryption key" and stop. |
| **OxidizedRelay decryption produces garbage** | `protoc --decode_raw` rejects first byte of every .pb | Key format mismatch (e.g., wrong nonce length). Document expected key shape from OxidizedRelay README; do NOT modify the tool. |
| **NisDec produces no metadata** | NisDec output file empty or "0 classes" | IL2CPP binary format may have changed. Inspect `libil2cpp.so` for known magic strings. Document; do NOT modify NisDec. |
| **Coverage < 30 schema files** | `find third_party/protos -name '*.proto' \| wc -l` < 30 | Don't vendor. Keep extracted artifacts in `/tmp/extract/<session>/proto/` for inspection. Surface as orchestrator exit 2 (data-quality failure, distinct from build failure). |
| **Account ban / anti-cheat** | `adb logcat` shows ban notification OR login fails post-session | Document. The game may detect Frida, the modified APK, or the proxy. Use a fresh Google account; do NOT use the user's primary. |

### Logging

Every script writes to stdout (human) AND `/tmp/extract/<session>/orchestrator.log` (machine). The session log captures every detection event so a future engineer can diagnose why a session underperformed.

### One failure this layer CAN'T catch

The takasho backend rotates or upgrades the ChaCha20 key derivation between game versions. v1.2.217 keys may not work on v1.2.218+. Acceptable — version-specific extraction is the norm for game RE; each version gets its own session.

---

## Testing

Five smoke checks, one per phase. No frameworks, no fixtures, no per-function suites.

| # | Check | Script | Pass = |
|---|-------|--------|--------|
| 1 | **Tooling smoke** (Phase A) | `scripts/osint/smoke_tooling.sh` — runs `oxrel --help`, `dotnet NisDec.dll --help`, `frida --version`, `adb devices` | All four commands exit 0 with non-empty output; emulator or device listed by `adb`. |
| 2 | **Device + APK smoke** (Phase B start) | `scripts/osint/smoke_device.sh` — installs the XAPK splits via `adb install-multiple`, asserts `pm list packages | grep dgames` matches | `adb shell pm list packages | grep com.dgames.g65002002.google` returns the package. |
| 3 | **Capture smoke** (Phase B middle) | `scripts/osint/smoke_capture.sh` — runs mitmdump for 60s against takasho domains | `.flow` file contains at least 1 takasho-schema URL. |
| 4 | **Frida key dump smoke** (Phase B.5) | `scripts/osint/smoke_frida.sh` — attaches Frida, sends hook event, asserts key dumped to stdout | At least 1 key event printed. |
| 5 | **End-to-end extraction smoke** (Phase D) | `scripts/osint/smoke_extract_protos.sh` — runs `extract_protos.sh` against a recorded session, asserts ≥30 schema files extracted, vendors into `third_party/protos/` | `find cosette-protocol/third_party/protos -name '*.proto' \| wc -l >= 30`. |

These five scripts ARE the test suite — they're also the documentation for how the layer works.

### What we deliberately do NOT test

- **Server behavior.** Once protos land, `cosette-protocol/build.rs` should compile them. That's the deferred Phase 0 of the prior plan — a separate spec handles it.
- **Network reliability.** Capture may flap; the smoke just needs ≥1 takasho URL.
- **Coverage completeness.** 30-service threshold is a sanity check, not a guarantee. Real coverage may require multiple sessions + manual curation.

---

## Open questions (defer until hit, don't pre-answer)

1. **What version of .NET does NisDec require?** Probably 6 or 8 LTS. Discover during task 2.
2. **Does `takt op. v1.2.217` enforce TLS pinning?** If yes, mitmproxy CA won't be trusted without pinning bypass. Discover during task 6.
3. **Does v1.2.217 have anti-Frida?** Probably yes (it's a 2025-era commercial game). Discover during task 5.
4. **How is the ChaCha20 key derived?** Per-session random? Hardcoded? KDF from device-bound secret? Discover during task 7.
5. **What's the structure of `libil2cpp.so` for v1.2.217?** May differ from older versions that NisDec was built for. Discover during task 8.

---

## Risks

- **Coverage may be <30 services** even with full pipeline success. The existing schema count is from a previous game version; v1.2.217 may have added/removed services. Discovery during implementation; orchestrator surfaces this honestly.
- **Frida detection in v1.2.217.** Modern Unity games use various anti-tamper (root detection, debugger detection, Frida signature checks). Bypass is out of scope; if detected, this spec blocks.
- **Game version drift.** v1.2.217 is the only target. Future versions need new sessions.
- **Account ban risk.** The capture session uses a real account with mitm + Frida. The game may ban. Mitigation: fresh throwaway account only.
- **NisDec may not parse v1.2.217's IL2CPP.** IL2CPP binary format evolves. If NisDec can't parse, we lose the binary-derived path. Traffic-derived path may still work.

---

## Success criteria for THIS design

This design is "done" when:

1. `oxrel --help`, `dotnet NisDec.dll --help`, `frida --version`, `adb devices` all resolve (smoke 1).
2. `takt op. v1.2.217` installs and launches on an Android device (smoke 2).
3. A 5-minute capture session produces a `.flow` dump with takasho URLs (smoke 3).
4. A Frida-attached session produces at least one dumped key (smoke 4, contingent on anti-tamper).
5. `extract_protos.sh` produces ≥30 `.proto` files (one per resource category under the existing `cosette-protocol/include/takasho/schema/` tree), vendored into `cosette-protocol/third_party/protos/` (smoke 5).

After Phase D ships, this spec is closed and a follow-up spec handles "Phase 0 of OSINT toolchain: vendor protos + env-overridable build.rs" (the previously deferred piece).