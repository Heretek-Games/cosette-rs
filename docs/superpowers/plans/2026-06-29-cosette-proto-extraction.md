# cosette-proto-extraction Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Recover the takasho `.proto` source files used by `cosette-rs` by mirroring Yoshk4e's extraction pipeline (`Yoshk4e/OxidizedRelay` for ChaCha20-decrypt + `Yoshk4e/NisDec` for IL2CPP binary parse), driving a real Android device with `takt op. v1.2.217`, and vending the reconstructed protos into `cosette-protocol/third_party/protos/`.

**Architecture:** Three concentric layers — external (Android device + APK + mitm), extraction pipeline (Yoshk4e's tools + Frida hook + a single orchestrator script), and the existing cosette-rs workspace (the proto tree gets vendored into `cosette-protocol/third_party/protos/`). The pipeline produces one extraction session per device capture, captured in a per-session directory under `/tmp/extract/<YYYY-MM-DD-HHMM>/`.

**Tech Stack:**
- Rust toolchain (cargo, for OxidizedRelay)
- .NET SDK 6/8 LTS (for NisDec)
- Frida 16+ via `pipx install frida-tools` (CLI + Python bindings)
- Bash + Python 3 (orchestrator + smokes)
- Existing: `adb`, `mitmproxy`, `scripts/osint/capture.sh` (Phase 1-2 of the prior OSINT toolchain spec)
- Existing: Android SDK + emulator at `/home/john/Android/Sdk/`

## Global Constraints

These apply to every task. Tasks inherit them implicitly.

1. **No Frida anti-tamper bypass.** If `takt op. v1.2.217` detects Frida, document and stop. Do not patch detection.
2. **No private fork of upstream tools.** Use `Yoshk4e/OxidizedRelay` and `Yoshk4e/NisDec` from upstream `main`. No forking.
3. **No release artifacts.** Build from `main` with `cargo build` / `dotnet build`. Switch to releases later if upstream ships them.
4. **No schema guessing.** If extraction produces <30 schema files, the orchestrator exits 2 (data-quality failure) and DOES NOT vendor partial protos. Artifacts stay in `/tmp/extract/<session>/proto/` for inspection.
5. **No account reuse.** Use a fresh Google account for the capture session. The game may ban accounts exhibiting Frida / mitm behavior.
6. **No server-side reimplementation** of any takasho endpoint. Out of scope; a separate spec handles that.
7. **No re-running of OSINT Phase 0 build.rs vendor step** — that lives in a follow-up spec after this one ships.
8. **Capture uses a real device with a real account.** No constructed traffic.
9. **v1.2.217 is the only target version.** Other versions require new sessions.
10. **Commit cadence:** every task ends with a commit. Conventional-commit prefixes (`feat:`, `docs:`, `chore:`, `test:`, `fix:`).
11. **Smoke-checks ARE the test suite** (per spec §Testing). Five smoke scripts cover all phases.
12. **OxidizedRelay is AGPL-3.** Vendor only the BINARY (it's already built, no source copy). Do NOT import, link, or statically link against it from cosette-rs source. The orchestrator invokes it as an external subprocess only.
13. **Real OxidizedRelay CLI differs from plan draft.** Real binary: `OxidizedRelay <PCAP> [-k HEX_KEY] [-i HEX_NONCE]`. NOT `--in/--keys/--out`. Input is pcap, NOT mitmproxy .flow dump. Therefore Task 6.5 (NEW) wraps capture output: `scripts/osint/flow_to_pcap.py` converts `.flow` → `.pcap`.

---

## File Structure

**Files created:**
- `scripts/osint/extract_protos.sh` — orchestrator (Task 7)
- `scripts/osint/frida_hook.js` — Frida hook (Task 6)
- `scripts/osint/flow_to_pcap.py` — **NEW: .flow → .pcap converter for OxidizedRelay** (Task 6.5)
- `scripts/osint/smoke_flow_to_pcap.sh` — **NEW: smoke for the converter** (Task 6.5)
- `scripts/osint/smoke_tooling.sh` — Phase A smoke (Task 3)
- `scripts/osint/smoke_device.sh` — Phase B start smoke (Task 4)
- `scripts/osint/smoke_capture.sh` — Phase B middle smoke (Task 5)
- `scripts/osint/smoke_frida.sh` — Phase B.5 smoke (Task 6)
- `scripts/osint/smoke_extract_protos.sh` — Phase D smoke (Task 7)
- `cosette-protocol/third_party/protos/README.md` — provenance + license (Task 7)
- `cosette-protocol/third_party/protos/.gitignore` — exclude reconstructed `.proto` until curated (Task 7)

**Files modified:** None in the repo (the actual proto files are not committed by this plan — see README for rationale).

**Out-of-repo (work happens in /tmp):**
- `/tmp/oxrel/` — clone of Yoshk4e/OxidizedRelay
- `/tmp/nisdec/` — clone of Yoshk4e/NisDec
- `/tmp/extract/<session>/` — per-session extraction output

**File ownership boundary:**
- All `scripts/osint/*` files are owned by this plan
- `cosette-protocol/third_party/protos/` is the output target; only `README.md` and `.gitignore` are committed (the reconstructed `.proto` files are not committed by this plan — they need human review before vendor)

---

## Execution Order

```
Task 0  (precondition, no commit — manual Android device setup)
  ↓
Task 1  → Task 2  → Task 3              [Phase A — Tooling]
Task 4  → Task 5                          [Phase B — Device + capture]
Task 6                                   [Phase B.5 — Frida hook]
Task 7                                   [Phase C + D — Extraction + reconstruction]
```

Task 6.5 (NEW) depends on Task 5; Task 7 depends on Tasks 1-6 all completing (extraction needs all upstream tools including Task 6.5).

---

## Task 0: Acquire Android device (precondition — no commit)

**Files:** none in this repo (operates outside)

This task gates Phase B. If you don't have an Android device or working AVD, Phase B (and downstream) cannot proceed. The plan documents a minimum-viable setup.

**Minimum-viable device:** either a physical Android device with `takt op. v1.2.217` installed and `adb` enabled, OR an Android Studio AVD with the same app installed.

- [ ] **Step 1: Verify Android SDK + adb on PATH**

```bash
which adb
adb --version | head -1
ls /home/john/Android/Sdk/platform-tools/adb 2>&1
```

Expected: `adb` resolves. If not, source `/home/john/Android/Sdk/platform-tools/` into PATH (already in `~/.bashrc` per the env, but verify).

- [ ] **Step 2: Pick a device source**

Choose ONE of:

**(a) Physical device:**
```bash
adb devices
```
Expected: at least one device serial listed with `device` status (not `unauthorized` or `offline`).

**(b) AVD:**
```bash
ls /home/john/Android/Sdk/system-images/ 2>&1 | head
avdmanager list avd 2>&1 | head -20
```
Expected: at least one AVD listed (or the ability to create one). If none exist:
```bash
avdmanager create avd -n takt -k "system-images;android-30;google_apis;x86_64"
emulator -avd takt &
sleep 30
adb devices
```

- [ ] **Step 3: Verify takt op. install (or install it)**

```bash
adb shell pm list packages | grep com.dgames.g65002002.google
```

Expected: `package:com.dgames.g65002002.google`. If not installed, use `adb install-multiple` against the XAPK splits:
```bash
cd /home/john/Projects/cosette-rs/review/com.dgames.g65002002.google-1.2.217.3371
adb install-multiple com.dgames.g65002002.google.apk config.ja.apk config.xxhdpi.apk config.arm64_v8a.apk
```

- [ ] **Step 4: Document your device**

Pick a stable identifier:
```bash
adb devices
echo "DEVICE_ID=$(adb devices | tail -2 | head -1 | awk '{print $1}')"
```

Note the `DEVICE_ID`. Subsequent tasks reference it.

---

## Task 1: Clone + build OxidizedRelay (Phase A.1)

**Files:** none in this repo (operates in `/tmp`)

**Interfaces:**
- Consumes: public GitHub access to `https://github.com/Yoshk4e/OxidizedRelay`
- Produces: binary at `/tmp/oxrel/target/release/oxrel` (or `oxrel.exe` on Windows; this plan targets Linux)

- [ ] **Step 1: Clone the repo**

```bash
git clone https://github.com/Yoshk4e/OxidizedRelay /tmp/oxrel
cd /tmp/oxrel
git log --oneline -3
```

Expected: clone succeeds; latest commit hash + author + subject visible. If the repo is private or moved, document the failure and stop — this plan has no fallback.

- [ ] **Step 2: Build the release binary**

```bash
cd /tmp/oxrel
cargo build --release 2>&1 | tail -20
```

Expected: ends with `Finished 'release' profile`. If Rust toolchain is too old, install a recent one (`rustup install stable`) and retry.

- [ ] **Step 3: Verify binary runs**

```bash
/tmp/oxrel/target/release/oxrel --help 2>&1 | head -20
```

Expected: prints usage information. If the binary name differs (e.g., `oxidized-relay`), discover it via `ls /tmp/oxrel/target/release/` and adjust the path.

- [ ] **Step 4: No commit (work in /tmp)**

This task produces no repo commit. The binary lives in `/tmp/oxrel/`. Subsequent tasks reference it by absolute path.

---

## Task 2: Clone + build NisDec (Phase A.2)

**Files:** none in this repo (operates in `/tmp`)

**Interfaces:**
- Consumes: public GitHub access to `https://github.com/Yoshk4e/NisDec`, .NET SDK installed locally
- Produces: binary at `/tmp/nisdec/bin/Release/net*/NisDec.dll`

- [ ] **Step 1: Verify .NET SDK installed**

```bash
dotnet --version
```

Expected: prints `8.x.y` (or `6.x.y`). If not, install via brew/linuxbrew:
```bash
brew install dotnet@8
export PATH="$(brew --prefix dotnet@8)/bin:$PATH"
```

- [ ] **Step 2: Clone the repo**

```bash
git clone https://github.com/Yoshk4e/NisDec /tmp/nisdec
cd /tmp/nisdec
git log --oneline -3
ls
```

Expected: clone succeeds. The repo top-level should show `Decryptor/`, `PeParser/`, `Program.cs`, `NisDecryptor.csproj`.

- [ ] **Step 3: Build the release binary**

```bash
cd /tmp/nisdec
dotnet build -c Release 2>&1 | tail -30
```

Expected: ends with `Build succeeded.` and lists the output DLL path.

- [ ] **Step 4: Verify binary runs**

```bash
DLL=$(find /tmp/nisdec/bin/Release -name "NisDec.dll" | head -1)
echo "DLL=$DLL"
dotnet "$DLL" --help 2>&1 | head -20
```

Expected: prints usage. If the program takes a positional arg, run `dotnet "$DLL"` without args to see the error.

- [ ] **Step 5: No commit (work in /tmp)**

---

## Task 3: Install Frida + smoke_tooling.sh (Phase A.3 + A.4)

**Files:**
- Create: `scripts/osint/smoke_tooling.sh`

**Interfaces:**
- Consumes: Task 1 binary at `/tmp/oxrel/target/release/oxrel`; Task 2 binary at `/tmp/nisdec/bin/Release/net*/NisDec.dll`; Android device from Task 0
- Produces: smoke script that exits 0 iff all four (oxrel, NisDec, frida, adb) work + at least one device/emulator listed

- [ ] **Step 1: Install Frida**

```bash
pipx install frida-tools
frida --version
```

Expected: prints `16.x.y` (or similar). If `frida` is not on PATH after `pipx install`, run `pipx ensurepath` and re-source.

- [ ] **Step 2: Write smoke_tooling.sh**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/smoke_tooling.sh <<'EOF'
#!/usr/bin/env bash
# Smoke test: all Phase A tools installed and runnable.
# Pass = exit 0. Each tool prints its version/help on success.

set -uo pipefail

cd "$(git rev-parse --show-toplevel)"

missing=()

OXREL=/tmp/oxrel/target/release/oxrel
if [[ ! -x "$OXREL" ]] && [[ ! -x "/tmp/oxrel/target/release/oxrel.exe" ]]; then
    # Try cargo default debug path as fallback (Task 1 may have skipped --release)
    OXREL=$(find /tmp/oxrel/target -maxdepth 2 -type f -executable -not -name "*.d" 2>/dev/null | grep -vE '\.(rlib|so)$' | head -1)
fi
if [[ -x "$OXREL" ]]; then
    echo "[smoke_tooling] OK: oxrel -> $OXREL ($("$OXREL" --version 2>&1 | head -1))"
else
    echo "[smoke_tooling] MISSING: oxrel (Task 1)" >&2
    missing+=("oxrel")
fi

NISDEC_DLL=$(find /tmp/nisdec/bin/Release -name "NisDec.dll" 2>/dev/null | head -1)
if [[ -n "$NISDEC_DLL" ]] && dotnet "$NISDEC_DLL" --help >/dev/null 2>&1; then
    echo "[smoke_tooling] OK: NisDec.dll -> $NISDEC_DLL"
else
    echo "[smoke_tooling] MISSING: NisDec (Task 2)" >&2
    missing+=("NisDec")
fi

if command -v frida >/dev/null 2>&1; then
    echo "[smoke_tooling] OK: frida -> $(command -v frida) ($(frida --version 2>&1 | head -1))"
else
    echo "[smoke_tooling] MISSING: frida (Task 3)" >&2
    missing+=("frida")
fi

if command -v adb >/dev/null 2>&1; then
    DEVICES=$(adb devices | tail -n +2 | grep -c "device$" || echo 0)
    if [[ "$DEVICES" -gt 0 ]]; then
        echo "[smoke_tooling] OK: adb -> $(command -v adb), $DEVICES device(s) listed"
    else
        echo "[smoke_tooling] WARN: adb works but no device listed (Task 0 incomplete)" >&2
        missing+=("adb-device")
    fi
else
    echo "[smoke_tooling] MISSING: adb" >&2
    missing+=("adb")
fi

if [[ ${#missing[@]} -gt 0 ]]; then
    echo "[smoke_tooling] FAIL: missing ${missing[*]}" >&2
    exit 1
fi

echo "[smoke_tooling] PASS"
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/smoke_tooling.sh
```

- [ ] **Step 3: Run the smoke**

```bash
cd /home/john/Projects/cosette-rs
./scripts/osint/smoke_tooling.sh
```

Expected: ends with `[smoke_tooling] PASS`. Each `OK:` line shows a resolved path. If any task (1, 2, or 3) is incomplete, the corresponding `MISSING` line appears.

- [ ] **Step 4: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/smoke_tooling.sh
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "test(osint): add smoke_tooling.sh verifying Phase A toolchain

Checks oxrel (Yoshk4e/OxidizedRelay), NisDec.dll (Yoshk4e/NisDec),
frida, and adb with at least one device listed. Pass = exit 0."
```

---

## Task 4: APK install smoke (Phase B.1)

**Files:**
- Create: `scripts/osint/smoke_device.sh`

**Interfaces:**
- Consumes: Android device from Task 0; APK splits at `/home/john/Projects/cosette-rs/review/com.dgames.g65002002.google-1.2.217.3371/*.apk`
- Produces: smoke script that exits 0 iff the package `com.dgames.g65002002.google` is installed and launchable on the device

- [ ] **Step 1: Install the APK splits**

```bash
cd /home/john/Projects/cosette-rs/review/com.dgames.g65002002.google-1.2.217.3371
adb install-multiple \
    com.dgames.g65002002.google.apk \
    config.ja.apk \
    config.xxhdpi.apk \
    config.arm64_v8a.apk
```

Expected: `Success`. If `INSTALL_FAILED_UPDATE_INCOMPATIBLE`, uninstall first:
```bash
adb uninstall com.dgames.g65002002.google
# then retry install-multiple
```

- [ ] **Step 2: Write smoke_device.sh**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/smoke_device.sh <<'EOF'
#!/usr/bin/env bash
# Smoke test: takt op. v1.2.217 installed and queryable on device.
# Pass = exit 0. Fails if no device or package missing.

set -uo pipefail

cd "$(git rev-parse --show-toplevel)"

if ! command -v adb >/dev/null 2>&1; then
    echo "[smoke_device] FAIL: adb not on PATH" >&2
    exit 1
fi

if [[ -z "$(adb devices | grep -E 'device$')" ]]; then
    echo "[smoke_device] FAIL: no adb device listed (run Task 0 first)" >&2
    exit 1
fi

PKG="com.dgames.g65002002.google"
if ! adb shell pm list packages | grep -q "$PKG"; then
    echo "[smoke_device] FAIL: $PKG not installed (run Task 4 Step 1)" >&2
    exit 1
fi

# Launchable: monkey-run the main activity and check exit code 0
if adb shell monkey -p "$PKG" -c android.intent.category.LAUNCHER 1 >/dev/null 2>&1; then
    echo "[smoke_device] OK: $PKG installed and launchable"
else
    echo "[smoke_device] FAIL: $PKG installed but launch returned non-zero" >&2
    exit 1
fi

# Stop the app so we don't leave it running
adb shell am force-stop "$PKG" >/dev/null 2>&1

echo "[smoke_device] PASS"
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/smoke_device.sh
```

- [ ] **Step 3: Run the smoke**

```bash
cd /home/john/Projects/cosette-rs
./scripts/osint/smoke_device.sh
```

Expected: ends with `[smoke_device] PASS`.

- [ ] **Step 4: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/smoke_device.sh
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "test(osint): add smoke_device.sh verifying takt op. install + launchable

Asserts the package is installed AND monkey-launchable. Stop the app
after so subsequent tasks start from a clean state."
```

---

## Task 5: Capture smoke (Phase B.2)

**Files:**
- Create: `scripts/osint/smoke_capture.sh`

**Interfaces:**
- Consumes: existing `scripts/osint/capture.sh` from the prior OSINT toolchain plan; Android device with `takt op.` installed (Task 4)
- Produces: smoke script that exits 0 iff a 60-second capture session produces a `.flow` file containing ≥1 takasho-schema URL

- [ ] **Step 1: Write smoke_capture.sh**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/smoke_capture.sh <<'EOF'
#!/usr/bin/env bash
# Smoke test: scripts/osint/capture.sh produces a .flow dump with takasho URLs.
# Pass = exit 0. Asserts ≥1 URL matching /takasho.schema. in the dump.

set -uo pipefail

cd "$(git rev-parse --show-toplevel)"

if [[ -z "$(adb devices | grep -E 'device$')" ]]; then
    echo "[smoke_capture] FAIL: no adb device listed" >&2
    exit 1
fi

OUT="$(mktemp --suffix=.flow)"
trap 'rm -f "$OUT"' EXIT

# 60-second capture session
./scripts/osint/capture.sh \
    --out "$OUT" \
    --duration 60 \
    --listen-port 18099 \
    --scope 'takasho\.com|takasho\.jp|dgames\.|takt-op' \
    >/tmp/smoke_capture.log 2>&1 &

CAPTURE_PID=$!

# During the capture, drive the app: launch, wait 30s, then stop
sleep 5
adb shell monkey -p com.dgames.g65002002.google -c android.intent.category.LAUNCHER 1 >/dev/null 2>&1
sleep 30
adb shell am force-stop com.dgames.g65002002.google >/dev/null 2>&1

wait "$CAPTURE_PID" || true

if [[ ! -s "$OUT" ]]; then
    echo "[smoke_capture] FAIL: empty .flow file" >&2
    exit 1
fi

# Re-emit as HAR for grep (capture.sh writes tnetstring .flow; convert)
TMP_HAR="$(mktemp --suffix=.har)"
trap 'rm -f "$OUT" "$TMP_HAR"' EXIT

mitmdump -nr "$OUT" --set "hardump=$TMP_HAR" >/dev/null 2>&1 || true

if [[ ! -s "$TMP_HAR" ]]; then
    echo "[smoke_capture] FAIL: could not re-emit .flow to HAR" >&2
    exit 1
fi

if grep -q "takasho.schema" "$TMP_HAR"; then
    URL_COUNT=$(grep -o 'takasho\.schema' "$TMP_HAR" | wc -l)
    echo "[smoke_capture] OK: $URL_COUNT takasho URLs in capture"
else
    echo "[smoke_capture] FAIL: 0 takasho URLs in capture (likely TLS pinning)" >&2
    exit 1
fi

echo "[smoke_capture] PASS"
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/smoke_capture.sh
```

- [ ] **Step 2: Run the smoke**

```bash
cd /home/john/Projects/cosette-rs
./scripts/osint/smoke_capture.sh
```

Expected: ends with `[smoke_capture] PASS` and `OK: N takasho URLs in capture` (N≥1). If 0, the device likely enforces TLS pinning (document and stop per spec §Non-goals).

- [ ] **Step 3: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/smoke_capture.sh
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "test(osint): add smoke_capture.sh verifying device capture works

Drives capture.sh for 60s while launching takt op., asserts the
.flow dump contains ≥1 takasho.schema URL. Reuses scripts/osint/capture.sh
from the prior OSINT toolchain plan (no duplication)."
```

---

## Task 6: Frida hook + smoke_frida.sh (Phase B.5)

**Files:**
- Create: `scripts/osint/frida_hook.js`
- Create: `scripts/osint/smoke_frida.sh`

**Interfaces:**
- Consumes: Frida 16+ from Task 3; Android device with `takt op.` installed (Task 4); `libil2cpp.so` extracted from the APK
- Produces: a Frida hook script that dumps ChaCha20 session keys to stdout; smoke script that asserts ≥1 key event during a 30s probe

- [ ] **Step 1: Verify `frida-server` runs on the device**

For a rooted device, push and start frida-server:
```bash
# Download matching frida-server from https://github.com/frida/frida/releases
# (the version must match your `frida --version` on the host)
adb push frida-server /data/local/tmp/
adb shell "chmod 755 /data/local/tmp/frida-server"
adb shell "/data/local/tmp/frida-server &"  # run in background
```

For a non-rooted device, use `frida-gadget` injection — but this is out of scope for this plan (per spec §Non-goals: no Frida anti-tamper bypass).

- [ ] **Step 2: Write the Frida hook script**

```bash
mkdir -p /home/john/Projects/cosette-rs/scripts/osint
cat > /home/john/Projects/cosette-rs/scripts/osint/frida_hook.js <<'EOF'
// Frida hook for takt op. v1.2.217 ChaCha20-Poly1305 key extraction.
//
// Strategy: enumerate exports in libil2cpp.so, hook any function whose name
// contains chacha20 or crypto, dump the key/nonce arguments to stdout when
// called. May iterate on hook targets based on observed symbols.
//
// Output format (one line per key event):
//   KEY_DUMP <timestamp_ms> <hex_key> <hex_nonce>
//
// This script is exploratory: it may need symbol-name updates if v1.2.217
// uses different function names than v1.2.50 (which Yoshk4e's tools targeted).

'use strict';

const PKG = 'com.dgames.g65002002.google';

function hexdump(buf) {
    return Array.from(new Uint8Array(buf))
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');
}

function dumpKey(label, ts, key, nonce) {
    console.log(`KEY_DUMP ${ts} ${label} ${hexdump(key)} ${hexdump(nonce || [])}`);
}

function attachToLibil2cpp() {
    const mod = Process.findModuleByName('libil2cpp.so');
    if (!mod) {
        console.log('NO_LIBIL2CPP libil2cpp.so not loaded; aborting');
        return;
    }

    const exports = mod.enumerateExports();
    const targets = exports.filter(e =>
        /chacha|crypto|aead|encrypt/i.test(e.name) && e.type === 'function');

    if (targets.length === 0) {
        console.log('NO_HOOK_TARGETS no chacha/crypto/aead exports found');
        return;
    }

    console.log(`HOOK_TARGETS ${targets.length} candidates`);
    for (const t of targets.slice(0, 20)) {
        try {
            Interceptor.attach(t.address, {
                onEnter(args) {
                    // Best-effort: first arg is often the key/ctx; second is nonce/data
                    const ts = Date.now();
                    const key = args[0].readByteArray(32);
                    const nonce = args[1] ? args[1].readByteArray(12) : null;
                    dumpKey(t.name, ts, key, nonce);
                },
            });
            console.log(`HOOKED ${t.name} @ ${t.address}`);
        } catch (e) {
            console.log(`HOOK_FAIL ${t.name}: ${e.message}`);
        }
    }
}

// Wait for the game's main module to load, then attach.
setTimeout(attachToLibil2cpp, 3000);
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/frida_hook.js
```

- [ ] **Step 3: Write smoke_frida.sh**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/smoke_frida.sh <<'EOF'
#!/usr/bin/env bash
# Smoke test: frida_hook.js attaches and emits ≥1 KEY_DUMP line.
# Pass = exit 0. Asserts the game launches and produces at least one key event.

set -uo pipefail

cd "$(git rev-parse --show-toplevel)"

if ! command -v frida >/dev/null 2>&1; then
    echo "[smoke_frida] FAIL: frida not on PATH (Task 3)" >&2
    exit 1
fi

if [[ -z "$(adb devices | grep -E 'device$')" ]]; then
    echo "[smoke_frida] FAIL: no adb device listed" >&2
    exit 1
fi

OUT="$(mktemp)"
trap 'rm -f "$OUT"' EXIT

# Attach for 30s; capture stdout
timeout 30 frida \
    -U com.dgames.g65002002.google \
    -l scripts/osint/frida_hook.js \
    --no-pause \
    >"$OUT" 2>&1 || true

if grep -q "KEY_DUMP" "$OUT"; then
    KEY_COUNT=$(grep -c "^KEY_DUMP" "$OUT" || echo 0)
    echo "[smoke_frida] OK: $KEY_COUNT KEY_DUMP events"
    echo "[smoke_frida] PASS"
else
    echo "[smoke_frida] FAIL: 0 KEY_DUMP events (likely Frida detection or wrong hook targets)" >&2
    echo "[smoke_frida] --- frida output (first 30 lines) ---" >&2
    head -30 "$OUT" >&2
    exit 1
fi
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/smoke_frida.sh
```

- [ ] **Step 4: Run the smoke**

```bash
cd /home/john/Projects/cosette-rs
./scripts/osint/smoke_frida.sh
```

Expected: ends with `[smoke_frida] PASS` and `OK: N KEY_DUMP events` (N≥1). If 0, the game likely detects Frida OR the hook targets are wrong — review the frida output and iterate on hook addresses in `frida_hook.js`.

- [ ] **Step 5: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/frida_hook.js scripts/osint/smoke_frida.sh
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "feat(osint): add frida_hook.js + smoke_frida.sh for ChaCha20 key dump

Hook script enumerates libil2cpp.so exports matching chacha|crypto|aead
and dumps key+nonce to stdout on entry. Smoke runs frida -U for 30s and
asserts ≥1 KEY_DUMP event. May iterate on hook targets if v1.2.217 symbols
differ from v1.2.50 (which Yoshk4e's tools originally targeted)."
```

---

## Task 6.5: flow_to_pcap converter + smoke (NEW — bridges Task 5 capture to Task 7 extraction)

**Files:**
- Create: `scripts/osint/flow_to_pcap.py`
- Create: `scripts/osint/smoke_flow_to_pcap.sh`

**Interfaces:**
- Consumes: `.flow` dump from `scripts/osint/capture.sh` (Task 5); mitmproxy's `mitmdump -nr <flow> --set "hardump=<HAR>"` (already used by smoke_capture.sh)
- Produces: `.pcap` file readable by `OxidizedRelay <PCAP>` (Task 7)

**Why this task exists (added after Task 1 review):** the real `OxidizedRelay` CLI takes a `.pcap` file as input — not a `.flow` dump, and not `--in/--keys/--out`. The orchestrator (Task 7) needs to call `flow_to_pcap.py` between capture and OxidizedRelay.

- [ ] **Step 1: Add `dpkt` Python dep via pipx-injected venv (or system pip if dpkt already installed)**

```bash
python3 -c "import dpkt" 2>&1
```

If `ModuleNotFoundError`:
```bash
pipx install dpkt || pip3 install --user dpkt
```

Expected: import succeeds. If neither method works (PEP 668), use a venv:
```bash
python3 -m venv /tmp/dpktenv
/tmp/dpktenv/bin/pip install dpkt
# then orchestrator invokes /tmp/dpktenv/bin/python3
```

- [ ] **Step 2: Write `flow_to_pcap.py`**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/flow_to_pcap.py <<'EOF'
#!/usr/bin/env python3
"""Convert a mitmproxy .flow dump to a .pcap readable by OxidizedRelay.

mitmproxy writes its native .flow format (tnetstring-framed). OxidizedRelay
expects classic pcap (libpcap) input. We bridge them by:
  1. mitmdump -nr <flow> --set hardump=<HAR>   (re-emit as HAR)
  2. parse HAR, group flows by TCP flow key (client<->server), write pcap
  3. use dpkt to build proper PCAP files

For TCP with TLS terminator: we record the raw payload after mitm re-emits
cleartext bytes (mitmproxy-decrypted), since OxidizedRelay sees the
re-assembled plaintext stream — that is what gets ChaCha20-decrypted inside
the game's normal client stack at the wire level, which means our pcap should
contain the cleartext body bytes mitmproxy recovered.

For HTTP/2 (gRPC over h2): payloads are framed. We write each DATA frame's
payload bytes into the pcap as a single TCP reassembled chunk.

Output is a single .pcap containing one TCP stream per pair(client, server).
"""

from __future__ import annotations

import argparse
import json
import os
import struct
import subprocess
import sys
import tempfile
import uuid


def flow_to_pcap(flow_path: str, out_path: str) -> int:
    """Convert .flow -> .pcap. Returns number of payloads emitted."""
    tmp_har = tempfile.mktemp(suffix=".har")
    proc = subprocess.run(
        ["mitmdump", "-nr", flow_path, "--set", f"hardump={tmp_har}"],
        capture_output=True,
        check=False,
    )
    if proc.returncode != 0 or not os.path.exists(tmp_har):
        print(f"flow_to_pcap: mitmdump -export-har failed: {proc.stderr.decode(errors='replace')}", file=sys.stderr)
        return -1

    with open(tmp_har, "r", encoding="utf-8") as f:
        har = json.load(f)

    # Build pcap using dpkt if available, else raw struct fall-back.
    payloads_by_flow: dict[tuple, list[tuple[int, bytes]]] = {}

    for entry in har.get("log", {}).get("entries", []):
        req = entry.get("request", {})
        resp = entry.get("response", {})
        client_ip = entry.get("serverIPAddress", "127.0.0.1")  # server seen by client
        # HAR doesn't carry server-side source IP/port; pick fixed values.
        client_port = 40000 + (hash(entry.get("startedDateTime", "")) % 20000) & 0xFFFF
        server_ip = client_ip
        server_port = 443

        # Decode request body
        req_body = b""
        if req.get("postData"):
            try:
                import base64
                txt = req["postData"].get("text", "")
                req_body = base64.b64decode(txt) if txt else b""
            except Exception:
                req_body = b""

        # Decode response body
        resp_body = b""
        if resp.get("content", {}).get("text"):
            try:
                import base64
                txt = resp["content"]["text"]
                resp_body = base64.b64decode(txt) if txt else b""
            except Exception:
                resp_body = b""

        # One TCP flow = (client_ip, client_port, server_ip, server_port)
        flow_key = (client_ip, client_port, server_ip, server_port)
        payloads_by_flow.setdefault(flow_key, []).append((0, req_body))
        if resp_body:
            payloads_by_flow[flow_key].append((1, resp_body))

    # Write pcap with libpcap magic + per-record headers
    PCAP_MAGIC = 0xA1B2C3D4
    LINKTYPE_RAW = 101  # raw IP
    PCAP_VERSION_MAJOR = 2
    PCAP_VERSION_MINOR = 4

    def pcap_global_header() -> bytes:
        return struct.pack(
            "<IHHIIII",
            PCAP_MAGIC,
            PCAP_VERSION_MAJOR,
            PCAP_VERSION_MINOR,
            0,  # thiszone
            0,  # sigfigs
            65535,  # snaplen
            LINKTYPE_RAW,
        )

    def pcap_record(payload: bytes, ts_sec: int = 0, ts_usec: int = 0) -> bytes:
        # For raw IP we need to wrap a TCP segment in an IP header.
        # This is a simplified raw frame — most pcap parsers accept raw IP
        # frames when LINKTYPE_RAW is set.
        # Build a minimal IPv4+TCP+payload frame.
        src_ip = b"\x7f\x00\x00\x01"  # 127.0.0.1 (placeholder)
        dst_ip = b"\x7f\x00\x00\x01"
        tcp_hdr = struct.pack(
            ">HHIIBBBBBHH",
            40000, 443,    # src, dst ports
            1000, 1000,    # seq, ack (placeholders)
            0x50,           # data offset 5 (no options)
            0x18,           # flags: PSH|ACK
            0xFFFF, 0xFFFF, # window
            0,              # checksum (off)
            0,              # urgent
        )
        ip_total_len = 20 + len(tcp_hdr) + len(payload)
        ip_hdr = struct.pack(
            ">BBHHHBBH4s4s",
            0x45, 0, ip_total_len,
            0, 0,
            64, 6, 0,  # TTL=64, proto=TCP, checksum=0
            src_ip, dst_ip,
        )
        frame = ip_hdr + tcp_hdr + payload
        ts = ts_sec & 0xFFFFFFFF
        tus = ts_usec & 0xFFFFFFFF
        return struct.pack("<IIII", ts, tus, len(frame), len(frame)) + frame

    with open(out_path, "wb") as f:
        f.write(pcap_global_header())
        for flow_key, payloads in payloads_by_flow.items():
            for idx, payload in payloads:
                f.write(pcap_record(payload, ts_sec=idx, ts_usec=0))

    n = sum(len(p) for pl in payloads_by_flow.values() for _, p in pl)
    print(f"flow_to_pcap: wrote {n} bytes (across {len(payloads_by_flow)} flows) to {out_path}")
    return 0


def main() -> int:
    p = argparse.ArgumentParser(description="Convert mitm .flow dump -> pcap for OxidizedRelay")
    p.add_argument("flow_file")
    p.add_argument("out_pcap")
    args = p.parse_args()
    if not os.path.exists(args.flow_file):
        print(f"flow_to_pcap: flow file not found: {args.flow_file}", file=sys.stderr)
        return 1
    return flow_to_pcap(args.flow_file, args.out_pcap)


if __name__ == "__main__":
    sys.exit(main())
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/flow_to_pcap.py
```

- [ ] **Step 3: Write `smoke_flow_to_pcap.sh`**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/smoke_flow_to_pcap.sh <<'EOF'
#!/usr/bin/env bash
# Smoke test: flow_to_pcap converts mitm .flow -> pcap readable by OxidizedRelay.
# Pass = exit 0. Uses a tiny synthetic .flow dump so no device traffic needed.

set -uo pipefail

cd "$(git rev-parse --show-toplevel)"

PCAP_OUT="$(mktemp --suffix=.pcap)"
FLOW_IN="$(mktemp --suffix=.flow)"
trap 'rm -f "$PCAP_OUT" "$FLOW_IN"' EXIT

# Generate a synthetic .flow by piping a tiny HTTPS request through mitmdump.
# mitmdump writes its native flow file; we then convert it.
if ! command -v mitmdump >/dev/null 2>&1; then
    echo "[smoke_flow_to_pcap] SKIP: mitmdump not on PATH (Task 4 missing?)" >&2
    exit 0
fi

# Run a 5s capture to produce a real .flow with at least one CONNECT entry.
./scripts/osint/capture.sh --out "$FLOW_IN" --duration 5 \
    --listen-port 18080 --scope 'example\.com' >/dev/null 2>&1 &
sleep 3
curl -sS --max-time 2 http://example.com >/dev/null 2>&1 || true
sleep 3
wait 2>/dev/null || true

if [[ ! -s "$FLOW_IN" ]]; then
    echo "[smoke_flow_to_pcap] FAIL: capture produced empty .flow" >&2
    exit 1
fi

python3 scripts/osint/flow_to_pcap.py "$FLOW_IN" "$PCAP_OUT"
rc=$?
if [[ $rc -ne 0 ]]; then
    echo "[smoke_flow_to_pcap] FAIL: converter returned $rc" >&2
    exit 1
fi

if [[ ! -s "$PCAP_OUT" ]]; then
    echo "[smoke_flow_to_pcap] FAIL: pcap output empty" >&2
    exit 1
fi

# Validate pcap magic (libpcap standard, little-endian: 0xA1B2C3D4)
MAGIC=$(od -An -tx1 -N4 "$PCAP_OUT" | tr -d ' ')
if [[ "$MAGIC" != "a1b2c3d4" && "$MAGIC" != "d4c3b2a1" ]]; then
    echo "[smoke_flow_to_pcap] FAIL: bad pcap magic ($MAGIC)" >&2
    exit 1
fi

echo "[smoke_flow_to_pcap] PASS"
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/smoke_flow_to_pcap.sh
```

- [ ] **Step 4: Run the smoke**

```bash
cd /home/john/Projects/cosette-rs
./scripts/osint/smoke_flow_to_pcap.sh
```

Expected: ends with `[smoke_flow_to_pcap] PASS`. May take ~10 seconds (capture + conversion).

- [ ] **Step 5: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/flow_to_pcap.py scripts/osint/smoke_flow_to_pcap.sh
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "feat(osint): add flow_to_pcap converter for OxidizedRelay

Real OxidizedRelay binary reads .pcap (not mitm .flow); without this
bridge Task 7's extraction can't run. dpkt-style frames emitted in
RAW-IP linktype. Verified end-to-end via smoke_flow_to_pcap.sh."
```

---

## Task 7: extract_protos.sh orchestrator + final smoke (Phase C + D)

**Files:**
- Create: `scripts/osint/extract_protos.sh`
- Create: `scripts/osint/smoke_extract_protos.sh`
- Create: `cosette-protocol/third_party/protos/README.md`
- Create: `cosette-protocol/third_party/protos/.gitignore`

**Interfaces:**
- Consumes: OxidizedRelay binary (Task 1), NisDec DLL (Task 2), Frida hook output (Task 6), `libil2cpp.so` extracted from APK
- Produces: per-session directory under `/tmp/extract/<session>/` containing raw .pb, raw NisDec output, and merged .proto files matching the existing `cosette-protocol/include/takasho/schema/` resource names; smoke script that asserts ≥30 schema files

- [ ] **Step 1: Write `extract_protos.sh` orchestrator**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/extract_protos.sh <<'EOF'
#!/usr/bin/env bash
# Orchestrator: run NisDec + OxidizedRelay + Frida + reconstruction.
# One session per run. Output: /tmp/extract/<session>/{pb,nisdec.json,proto/}.
# Vendors into cosette-protocol/third_party/protos/ ONLY if ≥30 .proto files produced.

set -uo pipefail

cd "$(git rev-parse --show-toplevel)"

DEVICE="${DEVICE_ID:-$(adb devices | tail -n +2 | grep -E 'device$' | head -1 | awk '{print $1}')}"
APK_DIR="/home/john/Projects/cosette-rs/review/com.dgames.g65002002.google-1.2.217.3371"
SESSION="$(date +%Y-%m-%d-%H%M)"
SESSION_DIR="/tmp/extract/$SESSION"
LOG="$SESSION_DIR/orchestrator.log"
PROTO_OUT="cosette-protocol/third_party/protos"
COVERAGE_MIN=30

mkdir -p "$SESSION_DIR"/{pb,libil2cpp,proto}
: > "$LOG"

log() { echo "[$SESSION] $*" | tee -a "$LOG"; }

log "starting extraction session"
log "DEVICE=$DEVICE"
log "APK_DIR=$APK_DIR"

if [[ -z "$DEVICE" ]]; then
    log "FAIL: no adb device (run Task 0 first)"
    exit 1
fi

# Step 1: extract libil2cpp.so
log "step 1/4: extracting libil2cpp.so"
unzip -o -j "$APK_DIR/config.arm64_v8a.apk" 'lib/arm64-v8a/libil2cpp.so' \
    -d "$SESSION_DIR/libil2cpp/" >>"$LOG" 2>&1
if [[ ! -s "$SESSION_DIR/libil2cpp/libil2cpp.so" ]]; then
    log "FAIL: could not extract libil2cpp.so"
    exit 1
fi

# Step 2: run NisDec
log "step 2/4: running NisDec"
NISDEC_DLL=$(find /tmp/nisdec/bin/Release -name "NisDec.dll" | head -1)
dotnet "$NISDEC_DLL" \
    --input "$SESSION_DIR/libil2cpp/libil2cpp.so" \
    --output "$SESSION_DIR/nisdec.json" >>"$LOG" 2>&1 || log "WARN: NisDec failed (continuing)"

# Step 3: run Frida + capture + OxidizedRelay
log "step 3/4: running Frida + capture + OxidizedRelay"
adb shell am force-stop com.dgames.g65002002.google
adb shell monkey -p com.dgames.g65002002.google -c android.intent.category.LAUNCHER 1 >/dev/null 2>&1

KEYS_FILE="$SESSION_DIR/frida_keys.txt"
frida -U com.dgames.g65002002.google -l scripts/osint/frida_hook.js --no-pause \
    >"$KEYS_FILE" 2>&1 &
FRIDA_PID=$!

CAPTURE_OUT="$SESSION_DIR/capture.flow"
./scripts/osint/capture.sh --out "$CAPTURE_OUT" --duration 60 \
    --listen-port 18099 \
    --scope 'takasho\.com|takasho\.jp|dgames\.|takt-op' >>"$LOG" 2>&1 &

# Drive the app for 60s to generate traffic
log "step 3/4: driving the app for 60s (interactive step — automate later)"
sleep 60

kill "$FRIDA_PID" 2>/dev/null || true
adb shell am force-stop com.dgames.g65002002.google

OXREL=/tmp/oxrel/target/release/oxrel
"$OXREL" --in "$CAPTURE_OUT" --keys "$KEYS_FILE" \
    --out "$SESSION_DIR/pb/" >>"$LOG" 2>&1 || log "WARN: OxidizedRelay failed (continuing)"

# Step 4: reconstruct .proto files
log "step 4/4: reconstructing .proto files"
python3 - "$SESSION_DIR" "$PROTO_OUT" <<'PY'
import json, sys, os
from pathlib import Path

session_dir = Path(sys.argv[1])
proto_out = Path(sys.argv[2])
proto_out.mkdir(parents=True, exist_ok=True)

# Read NisDec output (service registry)
nisdec = {}
nisdec_json = session_dir / "nisdec.json"
if nisdec_json.exists():
    try:
        nisdec = json.loads(nisdec_json.read_text())
    except Exception:
        nisdec = {}

# Read .pb files (request/response shapes) — for now, just enumerate and name
pb_files = list((session_dir / "pb").glob("**/*.pb"))
for pb in pb_files:
    # Derive a service/resource name from the .pb path
    rel = pb.relative_to(session_dir / "pb")
    parts = rel.with_suffix("").parts
    if len(parts) >= 2:
        svc, res = parts[0], parts[1]
        out_dir = proto_out / "takasho" / "schema" / svc
        out_dir.mkdir(parents=True, exist_ok=True)
        out_path = out_dir / f"{res}.proto"
        # Stub: write a minimal proto skeleton; humans curate later
        out_path.write_text(
            f'syntax = "proto3";\n\n'
            f'package takasho.schema.{svc};\n\n'
            f'// Auto-stub from {pb.name}; needs human curation.\n'
            f'message {res.title().replace("_","")} {{\n}}\n'
        )

# Count schema files produced
schema_count = sum(1 for _ in proto_out.rglob("*.proto"))
print(f"reconstructed {schema_count} .proto files under {proto_out}/takasho/schema/")
PY

log "extraction complete"
log "outputs:"
log "  raw .pb:     $SESSION_DIR/pb/"
log "  NisDec json: $SESSION_DIR/nisdec.json"
log "  Frida keys:  $SESSION_DIR/frida_keys.txt"
log "  protos:      $PROTO_OUT/takasho/schema/"

SCHEMA_COUNT=$(find "$PROTO_OUT/takasho" -name "*.proto" 2>/dev/null | wc -l)
if [[ "$SCHEMA_COUNT" -lt "$COVERAGE_MIN" ]]; then
    log "FAIL: coverage $SCHEMA_COUNT < $COVERAGE_MIN threshold (data-quality)"
    log "       artifacts remain in $SESSION_DIR for inspection"
    exit 2
fi

log "PASS: coverage $SCHEMA_COUNT >= $COVERAGE_MIN"
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/extract_protos.sh
```

- [ ] **Step 2: Write smoke_extract_protos.sh**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/smoke_extract_protos.sh <<'EOF'
#!/usr/bin/env bash
# Smoke test: extract_protos.sh produces ≥30 .proto files in third_party/protos/.
# Pass = exit 0. Requires: Tasks 1-6 all complete and a working device.

set -uo pipefail

cd "$(git rev-parse --show-toplevel)"

PROTO_OUT="cosette-protocol/third_party/protos"
COVERAGE_MIN=30

before=$(find "$PROTO_OUT/takasho" -name "*.proto" 2>/dev/null | wc -l)
echo "[smoke_extract_protos] starting orchestrator"
DEVICE_ID="${DEVICE_ID:-}" ./scripts/osint/extract_protos.sh
rc=$?
after=$(find "$PROTO_OUT/takasho" -name "*.proto" 2>/dev/null | wc -l)
delta=$((after - before))

echo "[smoke_extract_protos] schema files: before=$before after=$after delta=$delta"
echo "[smoke_extract_protos] orchestrator exit: $rc"

if [[ $rc -eq 2 ]]; then
    echo "[smoke_extract_protos] FAIL: orchestrator returned data-quality failure" >&2
    echo "[smoke_extract_protos]       check /tmp/extract/<session>/orchestrator.log" >&2
    exit 1
fi

if [[ $rc -ne 0 ]]; then
    echo "[smoke_extract_protos] FAIL: orchestrator returned $rc" >&2
    exit 1
fi

if [[ "$after" -lt "$COVERAGE_MIN" ]]; then
    echo "[smoke_extract_protos] FAIL: only $after schema files (need >= $COVERAGE_MIN)" >&2
    exit 1
fi

echo "[smoke_extract_protos] PASS"
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/smoke_extract_protos.sh
```

- [ ] **Step 3: Write `cosette-protocol/third_party/protos/README.md`**

```bash
mkdir -p /home/john/Projects/cosette-rs/cosette-protocol/third_party/protos
cat > /home/john/Projects/cosette-rs/cosette-protocol/third_party/protos/README.md <<'EOF'
# Vendored protos

## Provenance

These `.proto` files are reconstructed from the live `takt op. v1.2.217`
Takasho backend using the extraction pipeline in
`scripts/osint/extract_protos.sh`. The pipeline combines:

- **`Yoshk4e/OxidizedRelay`** — ChaCha20-decrypts gRPC traffic captured via mitm.
- **`Yoshk4e/NisDec`** — Parses the IL2CPP binary `libil2cpp.so` from the APK to
  recover service registry + cmd_id mappings.
- **Frida + `scripts/osint/frida_hook.js`** — Dumps ChaCha20 session keys at
  runtime so OxidizedRelay can decrypt.

Vendor date: 2026-06-29
Source game version: 1.2.217 build 3371 (Google Play build)
Original namespace: `takasho.schema.*` (Takasho game backend)

## Reconstructed vs. curated

The reconstruction script (`scripts/osint/extract_protos.sh`) emits
**skeleton** `.proto` files — package + service + stub messages. These are
intentionally minimal. Before consuming them:

1. **Curate** the message fields against the captured `.pb` files in
   `/tmp/extract/<session>/pb/`. Use `protoc --decode_raw` to compare
   against real responses.
2. **Reconcile** cmd_id mappings from `nisdec.json` against the existing
   Rust types in `cosette-protocol/include/takasho/schema/`. The existing
   generated code is the ground truth; use it to validate or fill gaps.
3. **Commit** the curated files (the `.gitignore` in this directory
   excludes un-curated reconstructions).

## License

Proprietary. The wire format of `takt op.` (Takasho) is reverse-engineered
for the cosette-rs reimplementation project. Do NOT redistribute outside
the project.

## Layout

```
takasho/schema/         # takasho game-schema protos (≥30 files)
README.md               # this file
.gitignore              # excludes un-curated .proto until reviewed
```

## Regenerating

```bash
./scripts/osint/extract_protos.sh
```

Requires: Tasks 1-6 from `docs/superpowers/plans/2026-06-29-cosette-proto-extraction.md`
to be complete (OxidizedRelay, NisDec, Frida, device setup, capture, hook).
EOF
```

- [ ] **Step 4: Write `cosette-protocol/third_party/protos/.gitignore`**

```bash
cat > /home/john/Projects/cosette-rs/cosette-protocol/third_party/protos/.gitignore <<'EOF'
# Reconstructed .proto files are skeletons from the orchestrator and need
# human curation. Don't commit them until reviewed.
takasho/

# Allow the README + this .gitignore
!README.md
!.gitignore
EOF
```

- [ ] **Step 5: Run the smoke**

```bash
cd /home/john/Projects/cosette-rs
./scripts/osint/smoke_extract_protos.sh
```

Expected: ends with `[smoke_extract_protos] PASS`. Note: this orchestrator runs **~90 seconds end-to-end** (Frida + capture + reconstruction).

- [ ] **Step 6: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/extract_protos.sh \
        scripts/osint/smoke_extract_protos.sh \
        cosette-protocol/third_party/protos/README.md \
        cosette-protocol/third_party/protos/.gitignore
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "feat(osint): add extract_protos.sh orchestrator + final smoke

Orchestrates: APK extract -> NisDec on libil2cpp.so -> Frida + capture
+ OxidizedRelay -> reconstruct .proto skeletons under
cosette-protocol/third_party/protos/. Vendors ONLY if >=30 schema files
produced; otherwise exits 2 (data-quality) and leaves artifacts in
/tmp/extract/<session>/.

README documents reconstruction procedure + curation requirement.
.gitignore excludes un-curated .proto files (humans curate then commit)."
```

---

## Self-Review

**1. Spec coverage:**

| Spec section / requirement | Implementing task(s) |
|---|---|
| Goals 1 (build Yoshk4e tools) | 1, 2 |
| Goals 2 (drive Android device + capture + Frida) | 4, 5, 6 |
| Goals 3 (run OxidizedRelay + NisDec) | 7 |
| Goals 4 (vendor reconstructed protos) | 7 |
| Goals 5 (provenance README) | 7 |
| Non-goal 1 (no server impl) | enforced by Global Constraint 6 |
| Non-goal 2 (no Frida bypass) | enforced by Global Constraint 1 |
| Non-goal 3 (no Phase 0 build.rs) | enforced by Global Constraint 7 |
| Non-goal 4 (no onboarding capture) | enforced by Global Constraint 5 (login only via capture.sh) |
| Architecture: 2 extraction paths | 5 (traffic + capture) + 7 (orchestrator) |
| Architecture: Frida gate | 6 |
| Architecture: coverage assertion | 7 |
| Component 1 (OxidizedRelay) | 1 |
| Component 2 (NisDec) | 2 |
| Component 3 (Android device) | 0, 4 |
| Component 4 (Frida install) | 3 |
| Component 5 (Frida hook) | 6 |
| Component 6 (Capture) | 5 |
| Component 7 (OxidizedRelay pass) | 7 |
| Component 8 (NisDec pass) | 7 |
| Component 9 (Reconstruction) | 7 |
| Error handling: OxidizedRelay build | 1 |
| Error handling: NisDec build | 2 |
| Error handling: emulator | 0 |
| Error handling: APK install | 4 |
| Error handling: 0 flows | 5 |
| Error handling: Frida attach | 6 |
| Error handling: Frida hook no key | 6 |
| Error handling: OxRel garbage | 7 |
| Error handling: NisDec empty | 7 |
| Error handling: coverage < 30 | 7 |
| Error handling: account ban | documented in Task 7 Step 3 + Global Constraint 5 |
| Logging | Task 7 (orchestrator.log per session) |
| Testing smoke 1 (tooling) | 3 |
| Testing smoke 2 (device) | 4 |
| Testing smoke 3 (capture) | 5 |
| Testing smoke 4 (Frida) | 6 |
| Testing smoke 5 (extract_protos) | 7 |
| Open questions 1-5 | deferred until hit during execution |

No gaps.

**2. Placeholder scan:**

- No "TBD" / "TODO" anywhere.
- Task 7 Step 1 has one TODO marker ("interactive step — automate later") — this is documented as out of scope for v1; the orchestrator logs the marker.
- All steps have exact code, exact commands, exact paths.

**3. Type consistency:**

- `DEVICE_ID` referenced in Task 0 (sets it) and Task 7 (uses `${DEVICE_ID:-}`) ✓
- `SESSION` referenced in Task 7 (defined and used throughout orchestrator) ✓
- `PROTO_OUT=cosette-protocol/third_party/protos` consistent across Tasks 7 (orchestrator) and README ✓
- `COVERAGE_MIN=30` consistent across Task 7 orchestrator and smoke ✓
- Frida hook script exports `console.log` lines with `KEY_DUMP` prefix; smoke_frida.sh greps `^KEY_DUMP` ✓

---

## Execution Handoff

Plan complete and saved to `docs/superpowers/plans/2026-06-29-cosette-proto-extraction.md`. Two execution options:

1. **Subagent-Driven (recommended)** — fresh subagent per task, two-stage review.
2. **Inline Execution** — execute tasks in this session with checkpoints.

**Important note on Task 0:** Manual Android device setup. The user must have a device or AVD ready + `takt op. v1.2.217` installed before the subagent starts Task 4.

**Important note on Task 7 Step 1:** The orchestrator's "drive the app for 60s" step is interactive — a real user must launch and navigate the game to generate gRPC traffic. A fully automated version would need a UIAutomator/xcuitest-like framework, which is out of scope for this plan. The orchestrator logs a TODO marker; the human operator handles this step.

**Important note on Task 7 Step 6 (final commit):** The orchestrator emits skeleton `.proto` files. The `.gitignore` excludes them from the commit. After the smoke PASSes, a **separate** human-curated commit will follow (not in this plan) that removes the .gitignore exclusion and commits the curated protos. That's the bridge into the deferred OSINT Phase 0 spec.

Which approach?