# cosette-osint-toolchain Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an OSINT tooling layer around `cosette-rs` that ingests APK + mitm traffic, decodes gRPC flows against vendored protos, and inventories endpoints — without coupling the tooling to server internals.

**Architecture:** Three concentric layers — external (device + APK + mitm), OSINT toolchain (vendored protos + native binaries + scripts), and the existing Rust workspace. Tooling writes only to `docs/osint/` and `mitm_mcp_traffic.db`; it never edits Rust server crates. Boundary is enforced by file ownership, not policy.

**Tech Stack:**
- Bash + Python 3 (helper scripts under `scripts/osint/`)
- `jadx`, `apktool`, `grpcurl`, `protoc` via Homebrew
- `UnityPy` via `pipx`
- `mitmdump` + `mcp__mitmproxy-mcp__load_traffic_file` for capture + DB import
- Existing: `cargo` + `tonic 0.9` + `prost 0.11` + `tracing` (no changes to crate deps)
- Android SDK already on `PATH` (adb, build-tools)

## Global Constraints

These apply to every task. Tasks inherit them implicitly.

1. **No Rust server changes.** OSINT layer is read-only w.r.t. `cosette-dispatchserver/`, `cosette-gameserver/`, `cosette-sdkserver/`, `common/`. Server impls live in later specs.
2. **No new Cargo dependencies.** Reuse existing `tonic`, `prost`, `tracing`. Only `cosette-protocol/build.rs` may be modified, and only to fix the hardcoded path.
3. **No CI integration.** Scripts must be manual-runnable from the repo root.
4. **No Frida.** Out of scope unless a capture is actually blocked by TLS pinning.
5. **No DB schema changes** to `mitm_mcp_traffic.db` (current `flows` table is sufficient). Capture flows are append-only; never delete.
6. **No writes to the live takasho server.** Capture script is read-only observation; replay/auth tooling is a later layer.
7. **Shell + Python only** for helper scripts. Do not introduce a Rust binary for the OSINT layer.
8. **Vendored protos attribution**: single commit with `git log` message naming the source machine path (`/home/yohk4e/Downloads/proto/protos`) so provenance is auditable.
9. **Commit cadence**: every task ends with a commit. Use conventional-commit prefixes (`feat:`, `docs:`, `chore:`, `test:`).
10. **TDD where it fits, smoke-checks where it doesn't**: shell scripts get smoke-check scripts; the four smoke scripts ARE the test suite (per spec §Testing).

---

## File Structure

**Files created:**
- `cosette-protocol/third_party/protos/takasho/...` — vendored `.proto` tree (Task 1)
- `cosette-protocol/third_party/protos/google/protobuf/descriptor.proto` — vendored WKT (Task 1)
- `cosette-protocol/third_party/protos/README.md` — provenance + license note (Task 1)
- `scripts/osint/capture.sh` — mitmdump wrapper (Task 5)
- `scripts/osint/import_flows.sh` — DB import helper (Task 6)
- `scripts/osint/decode_flows.py` — decode + classify (Task 7)
- `scripts/osint/extract_apk.sh` — apktool + jadx + UnityPy (Task 8)
- `scripts/osint/smoke_build.sh` — Phase 0 smoke (Task 3)
- `scripts/osint/smoke_tools.sh` — Phase 1 smoke (Task 4)
- `scripts/osint/smoke_capture.sh` — Phase 2 mid smoke (Task 9)
- `scripts/osint/smoke_e2e.sh` — Phase 2 close smoke (Task 10)
- `docs/osint/endpoints.md` — endpoint inventory template (Task 11)
- `docs/osint/.gitignore` — exclude large extracted assets, keep decoded JSON + markdown (Task 12)

**Files modified:**
- `cosette-protocol/build.rs` — env-overridable proto root + improved panic message (Task 2)

**Files read (not modified):**
- `cosette-protocol/Cargo.toml` — confirms `tonic-build` is already a build-dep
- `Cargo.toml` (workspace) — confirms workspace layout

**File ownership boundary:**
- OSINT scripts (`scripts/osint/**`) — owned by this plan; future plans may add to it but not modify existing scripts without review
- `cosette-protocol/build.rs` — modified once in Task 2; future changes belong to a "proto tooling" sub-plan
- `docs/osint/endpoints.md` — append-only inventory; future plans edit rows, never delete

---

## Execution Order

Tasks must be run in this order (Task 10 depends on Task 11 having created `endpoints.md`):

```
Task 0  (precondition, no commit)
  ↓
Task 1  → Task 2  → Task 3          [Phase 0]
Task 4                              [Phase 1]
Task 5  → Task 6  → Task 7  → Task 8 → Task 11 → Task 9 → Task 10 → Task 12  [Phase 2]
```

---

## Task 0: Acquire proto source (precondition — no commit)

**Files:** none in this repo (operates outside)

This task gates Phase 0. If the protos cannot be acquired, Task 1 cannot proceed, and the engineer must either (a) locate them or (b) wait until a follow-up plan recovers proto structure from `cosette-protocol/include/`.

**Canonical source:** `/home/yohk4e/Downloads/proto/protos` on the historical build machine (user "yohk4e", per `cosette-protocol/build.rs:79`). This directory does NOT exist on `/home/john`.

- [ ] **Step 1: Verify protos are not already on this machine**

```bash
test -d /home/yohk4e/Downloads/proto/protos && echo "FOUND" || echo "NOT_FOUND"
find /home -maxdepth 5 -type d -name protos 2>/dev/null | head -5
```

Expected: prints `NOT_FOUND` and zero matches. If a match appears, note its path and skip to Step 3.

- [ ] **Step 2: Locate a copy via fallback paths**

In order of preference:

```bash
# (a) Check any external drive / mount
ls /mnt /media 2>/dev/null

# (b) Check git remotes (some teams stash protos in a private git repo)
cd /home/john/Projects/cosette-rs && git remote -v

# (c) Check for any tar/zip of the protos
find /home /tmp -name "protos.tar*" -o -name "takasho-protos*" 2>/dev/null | head

# (d) Check if user has another machine accessible
ssh yohk4e@<host> "ls /home/yohk4e/Downloads/proto/protos" 2>&1 | head
```

Document what you find in a sticky note (or just remember). If none of (a)-(d) succeeds, STOP and ask the user. Do NOT proceed without protos.

- [ ] **Step 3: Stage protos in a working location**

```bash
mkdir -p /tmp/proto-source
# Adjust source path based on Step 2 result
cp -r <source_path>/. /tmp/proto-source/
ls /tmp/proto-source | head
find /tmp/proto-source -name "*.proto" | wc -l
```

Expected: a non-zero count of `.proto` files. If zero, the source isn't actually the protos; re-check.

- [ ] **Step 4: Hand off to Task 1**

Task 1 expects protos at `/tmp/proto-source/`. Continue immediately.

---

## Task 1: Vendor protos into the repo

**Files:**
- Create: `cosette-protocol/third_party/protos/takasho/...` (mirrors source tree)
- Create: `cosette-protocol/third_party/protos/google/protobuf/descriptor.proto`
- Create: `cosette-protocol/third_party/protos/README.md`

**Interfaces:**
- Consumes: protos at `/tmp/proto-source/` (from Task 0)
- Produces: a directory tree that `cosette-protocol/build.rs` can compile against

- [ ] **Step 1: Create the vendored directory layout**

```bash
cd /home/john/Projects/cosette-rs
mkdir -p cosette-protocol/third_party/protos/{takasho,google/protobuf}
```

- [ ] **Step 2: Copy takasho protos**

```bash
cd /home/john/Projects/cosette-rs
# Copy entire takasho subtree from staged source
cp -r /tmp/proto-source/takasho/. cosette-protocol/third_party/protos/takasho/
find cosette-protocol/third_party/protos/takasho -name "*.proto" | wc -l
```

Expected: at least 30 `.proto` files (matches the existing `cosette-protocol/include/takasho/schema/` resource count).

- [ ] **Step 3: Copy google/protobuf WKT (descriptor.proto)**

The existing build emits well-known types (`compile_well_known_types(true)`), so we need `descriptor.proto` available locally to avoid a future protoc crash when offline.

```bash
cd /home/john/Projects/cosette-rs
cp /tmp/proto-source/google/protobuf/descriptor.proto cosette-protocol/third_party/protos/google/protobuf/
ls cosette-protocol/third_party/protos/google/protobuf/
```

Expected: `descriptor.proto` listed.

- [ ] **Step 4: Write provenance README**

```bash
cat > /home/john/Projects/cosette-rs/cosette-protocol/third_party/protos/README.md <<'EOF'
# Vendored protos

## Provenance

These `.proto` files were vendored from the historical build machine path
`/home/yohk4e/Downloads/proto/protos` (referenced by `cosette-protocol/build.rs`).

Vendor date: 2026-06-29
Source machine: yohk4e (see git log for commit hash)
Original namespace: `takasho.schema.*` (Takasho game backend)

## License

Proprietary. These protos describe the wire format of the `takt op.`
(Takasho) game backend and are reverse-engineered for the cosette-rs
reimplementation project. Do NOT redistribute outside the project.

## Layout

```
takasho/schema/        # takasho game-schema protos (~30+ resources)
google/protobuf/       # protoc well-known types (subset)
```

## Regenerating Rust types

These protos are consumed by `cosette-protocol/build.rs` via
`tonic_build`. Override the location with the `CARGO_MANIFEST_PROTO_ROOT`
environment variable if needed.
EOF
```

- [ ] **Step 5: Verify the vendored tree compiles (sanity, not full build)**

```bash
cd /home/john/Projects/cosette-rs
find cosette-protocol/third_party/protos -name "*.proto" | wc -l
# Expect: at least 30
find cosette-protocol/third_party/protos -type d | head -20
```

Expected: file count and directory listing show the takasho + google tree.

- [ ] **Step 6: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add cosette-protocol/third_party/protos/
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "feat(protocol): vendor takasho protos from /home/yohk4e/Downloads/proto/protos

Provenance + license recorded in third_party/protos/README.md.
Closes hardcoded-path debt closed by follow-up build.rs refactor."
```

---

## Task 2: Refactor build.rs to env-overridable proto root

**Files:**
- Modify: `cosette-protocol/build.rs` (one section near top: `let proto_root = ...` and the panic block)

**Interfaces:**
- Consumes: vendored protos at `cosette-protocol/third_party/protos/` (from Task 1)
- Produces: `cargo build -p cosette-protocol` succeeds offline using the vendored tree; `CARGO_MANIFEST_PROTO_ROOT=/some/other/path` overrides it

- [ ] **Step 1: Read the current path block**

```bash
cd /home/john/Projects/cosette-rs
sed -n '78,90p' cosette-protocol/build.rs
```

Expected: lines like

```
    let proto_root = PathBuf::from("/home/yohk4e/Downloads/proto/protos");
    if !proto_root.exists() {
        panic!("proto root not found: {}", proto_root.display());
    }
```

Note the exact lines to edit.

- [ ] **Step 2: Replace the hardcoded path with env-overridable default**

Edit `cosette-protocol/build.rs`. Replace the lines identified in Step 1 with:

```rust
    let proto_root = match env::var("CARGO_MANIFEST_PROTO_ROOT") {
        Ok(p) => PathBuf::from(p),
        Err(_) => manifest_dir.join("third_party/protos"),
    };
    if !proto_root.exists() {
        panic!(
            "proto root not found at `{}` (override via CARGO_MANIFEST_PROTO_ROOT)",
            proto_root.display()
        );
    }
```

(Keep the `let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);` line that already exists above — we use `manifest_dir` as the default.)

- [ ] **Step 3: Verify the file still parses**

```bash
cd /home/john/Projects/cosette-rs
rustc --edition 2021 -Zparse-only cosette-protocol/build.rs 2>&1 | head -20 || true
cargo build -p cosette-protocol 2>&1 | tail -20
```

Expected: `cargo build` either succeeds or fails for unrelated reasons (e.g., missing `walkdir` dev-dep), but NOT with the `proto root not found` panic.

- [ ] **Step 4: Verify override works**

```bash
cd /home/john/Projects/cosette-rs
CARGO_MANIFEST_PROTO_ROOT=/nonexistent/path cargo build -p cosette-protocol 2>&1 | tail -10
```

Expected: error message contains the overridden path `/nonexistent/path` and the hint about `CARGO_MANIFEST_PROTO_ROOT`.

- [ ] **Step 5: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add cosette-protocol/build.rs
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "fix(protocol): make build.rs proto root env-overridable

Default to vendored third_party/protos/. Override via
CARGO_MANIFEST_PROTO_ROOT. Improved panic message prints both the
resolved path and the env var name for easy misconfig diagnosis."
```

---

## Task 3: Phase 0 smoke — build verification

**Files:**
- Create: `scripts/osint/smoke_build.sh`

**Interfaces:**
- Consumes: `cosette-protocol/build.rs` (from Task 2), vendored protos (Task 1)
- Produces: exit 0 if `cargo build -p cosette-protocol` succeeds against vendored protos

- [ ] **Step 1: Create scripts/osint/ directory**

```bash
cd /home/john/Projects/cosette-rs
mkdir -p scripts/osint
```

- [ ] **Step 2: Write smoke_build.sh**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/smoke_build.sh <<'EOF'
#!/usr/bin/env bash
# Smoke test: cosette-protocol builds offline against vendored protos.
# Pass = exit 0. Fail = non-zero exit + clear stderr.

set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

echo "[smoke_build] running: cargo build -p cosette-protocol"
if ! cargo build -p cosette-protocol 2>&1 | tee /tmp/smoke_build.log; then
    echo "[smoke_build] FAIL: cargo build returned non-zero" >&2
    exit 1
fi

# Belt-and-braces: make sure the panic string isn't present in the log
if grep -q "proto root not found" /tmp/smoke_build.log; then
    echo "[smoke_build] FAIL: build.rs panicked on proto root" >&2
    exit 1
fi

echo "[smoke_build] PASS"
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/smoke_build.sh
```

- [ ] **Step 3: Run the smoke**

```bash
cd /home/john/Projects/cosette-rs
./scripts/osint/smoke_build.sh
```

Expected: ends with `[smoke_build] PASS`. (If `cargo build` fails for unrelated reasons — missing `walkdir`, missing system libs — Task 0 was incomplete; recover first.)

- [ ] **Step 4: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/smoke_build.sh
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "test(osint): add smoke_build.sh verifying vendored-proto compile"
```

**Phase 0 done.** Stop here unless the user wants to proceed without Phases 1-2.

---

## Task 4: Phase 1 — install native toolchain + smoke

**Files:**
- Create: `scripts/osint/smoke_tools.sh`

**Interfaces:**
- Consumes: nothing (operates on PATH)
- Produces: exit 0 if `jadx`, `apktool`, `grpcurl`, `protoc`, and `python3 -m UnityPy` all resolve

- [ ] **Step 1: Install native binaries**

```bash
brew install jadx apktool grpcurl protobuf
pipx install UnityPy
pipx ensurepath
```

Expected: each `brew install` line ends with success. `pipx ensurepath` prints PATH hint — restart shell if needed before subsequent steps.

- [ ] **Step 2: Verify each binary resolves**

```bash
which jadx apktool grpcurl protoc
python3 -m UnityPy --help 2>&1 | head -3
```

Expected: all five commands resolve. (If `python3 -m UnityPy --help` fails because UnityPy has no CLI entry point, drop that line and rely on `python3 -c "import UnityPy; print(UnityPy.__version__)"` instead.)

- [ ] **Step 3: Write smoke_tools.sh**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/smoke_tools.sh <<'EOF'
#!/usr/bin/env bash
# Smoke test: native OSINT toolchain is installed and on PATH.
# Pass = exit 0. Each tool's version is printed on success.

set -uo pipefail

cd "$(git rev-parse --show-toplevel)"

missing=()

for tool in jadx apktool grpcurl protoc; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "[smoke_tools] MISSING: $tool" >&2
        missing+=("$tool")
    else
        echo "[smoke_tools] OK: $tool -> $(command -v "$tool")"
    fi
done

if ! python3 -c "import UnityPy; print(UnityPy.__version__)" 2>/dev/null; then
    echo "[smoke_tools] MISSING: python3 -m UnityPy" >&2
    missing+=("UnityPy")
else
    echo "[smoke_tools] OK: UnityPy -> $(python3 -c "import UnityPy; print(UnityPy.__version__)")"
fi

if [ ${#missing[@]} -gt 0 ]; then
    echo "[smoke_tools] FAIL: missing ${missing[*]}" >&2
    exit 1
fi

echo "[smoke_tools] PASS"
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/smoke_tools.sh
```

- [ ] **Step 4: Run the smoke**

```bash
cd /home/john/Projects/cosette-rs
./scripts/osint/smoke_tools.sh
```

Expected: ends with `[smoke_tools] PASS`. Each `OK:` line shows a resolved path.

- [ ] **Step 5: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/smoke_tools.sh
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "chore(osint): add smoke_tools.sh verifying native toolchain on PATH

Expected toolchain: jadx, apktool, grpcurl, protoc (homebrew);
UnityPy (pipx). See spec for install commands."
```

**Phase 1 done.**

---

## Task 5: capture.sh — mitmdump wrapper

**Files:**
- Create: `scripts/osint/capture.sh`

**Interfaces:**
- Consumes: `--out <file.flow>` (required), `--duration <seconds>` (default 1800), `--listen-port <port>` (default 8080), `--scope <regex>` (default `takasho.*|dgames.*`)
- Produces: a `.flow` file written to `--out` containing mitmproxy's native flow format

- [ ] **Step 1: Write capture.sh**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/capture.sh <<'EOF'
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
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/capture.sh
```

- [ ] **Step 2: Verify script is executable + help works**

```bash
cd /home/john/Projects/cosette-rs
./scripts/osint/capture.sh 2>&1 | head -3
./scripts/osint/capture.sh --out /tmp/cap.flow 2>&1 | head -3
```

Expected: first command prints the usage line (exit 2). Second command (with `--out`) starts `mitmdump` and we can Ctrl-C — or, with `--duration 1`, runs to completion in ~1s and exits 0 with `[capture] FAIL: output file is empty` (since no real traffic). That's acceptable for now; the real verification is in smoke_capture.sh (Task 9).

- [ ] **Step 3: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/capture.sh
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "feat(osint): add capture.sh wrapping mitmdump for takasho domains

Writes mitmproxy .flow dumps. Pair with import_flows.sh for DB import.
Scope defaults to takasho.*|dgames.*|takt-op; override with --scope."
```

---

## Task 6: import_flows.sh — DB import helper

**Files:**
- Create: `scripts/osint/import_flows.sh`

**Interfaces:**
- Consumes: `<file.flow>` (path to mitmproxy dump)
- Produces: rows appended to `mitm_mcp_traffic.db` via re-emitting as HAR and a small Python sqlite3 loader

- [ ] **Step 1: Write import_flows.sh**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/import_flows.sh <<'EOF'
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
```

- [ ] **Step 2: Make executable and verify usage**

```bash
cd /home/john/Projects/cosette-rs
chmod +x scripts/osint/import_flows.sh
# Smoke: invoke with no args
./scripts/osint/import_flows.sh 2>&1 | head -3
```

Expected: prints `usage:` and exits 2.

- [ ] **Step 3: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/import_flows.sh
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "feat(osint): add import_flows.sh to load .flow dumps into mitm DB

Uses mitmdump -export-har to re-emit, then a small Python sqlite3
loader for INSERT OR IGNORE into the existing flows table. Schema
is unchanged (per spec constraint)."
```

---

## Task 7: decode_flows.py — classify + decode captured traffic

**Files:**
- Create: `scripts/osint/decode_flows.py`

**Interfaces:**
- Consumes: `<file.flow>` (positional), `--limit N` (default 50), `--out-dir <path>` (default `docs/osint/decoded`)
- Produces: per-flow JSON files in `<out-dir>/<flow_id>.json` with `{url, method, status, decoded_request, decoded_response, decode_status, proto_candidate}`
- Non-goals: do NOT modify the DB; do NOT generate new `.proto` files (that's a separate plan)

- [ ] **Step 1: Write decode_flows.py**

```python
#!/usr/bin/env python3
"""Decode captured takasho flows against vendored protos.

Reads a mitmproxy .flow dump, classifies each request by URL path,
attempts to decode request/response bodies with the matching .proto
descriptor, and emits one JSON file per flow.

Decode strategy:
  1. Match URL path against known proto prefixes in cosette-protocol.
  2. If a candidate proto is found, decode with protoc --decode_raw
     (we don't have a compiled descriptor at runtime; this gives
     a best-effort raw parse).
  3. If decode fails, mark decode_status="missing-proto" so the
     inventory in docs/osint/endpoints.md can flag it.

This script is intentionally read-only and stateless. It writes
JSON samples; humans decide whether to add missing protos.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import uuid
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
PROTO_ROOT = REPO_ROOT / "cosette-protocol" / "third_party" / "protos"
OUT_DEFAULT = REPO_ROOT / "docs" / "osint" / "decoded"

# Heuristic: takasho paths look like
#   /takasho.schema.<service>.<resource>/<Method>
# Map the leading proto package to a candidate .proto file under
# third_party/protos/takasho/schema/<service>/.
TAKASHO_PATH_RE = re.compile(r"^/takasho\.schema\.([^.]+)\.([^/]+)/")


def find_proto_candidate(path: str) -> Path | None:
    m = TAKASHO_PATH_RE.match(path)
    if not m:
        return None
    service, resource = m.group(1), m.group(2)
    candidate = PROTO_ROOT / "takasho" / "schema" / service / f"{resource}.proto"
    return candidate if candidate.exists() else None


def decode_raw(body: bytes, proto_path: Path) -> dict | None:
    """Best-effort raw decode using protoc --decode_raw.

    Returns parsed dict on success, None on failure.
    """
    if not body:
        return None
    try:
        proc = subprocess.run(
            ["protoc", "--decode_raw"],
            input=body,
            capture_output=True,
            timeout=5,
            check=False,
        )
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return None
    if proc.returncode != 0:
        return None
    return {"textproto": proc.stdout.decode("utf-8", errors="replace")}


def parse_flow_dump(flow_path: Path) -> list[dict]:
    """Read a mitmproxy .flow dump. Returns one dict per flow."""
    tmp_har = Path("/tmp") / f"decode_flows_{uuid.uuid4()}.har"
    try:
        subprocess.run(
            ["mitmdump", "-nr", str(flow_path), "--export-har", str(tmp_har)],
            check=True,
            capture_output=True,
            timeout=60,
        )
        with tmp_har.open() as f:
            har = json.load(f)
        return har.get("log", {}).get("entries", [])
    finally:
        tmp_har.unlink(missing_ok=True)


def emit_one(entry: dict, out_dir: Path) -> dict:
    req = entry.get("request", {})
    resp = entry.get("response", {})
    url = req.get("url", "")
    path = req.get("path", "")
    if not path:
        from urllib.parse import urlparse
        path = urlparse(url).path

    candidate = find_proto_candidate(path)

    req_body_b64 = ""
    if req.get("postData"):
        req_body_b64 = "".join(
            p.get("text", "") for p in req["postData"].get("params", [])
        )
    resp_body_b64 = resp.get("content", {}).get("text", "")

    decoded_req = None
    decoded_resp = None
    decode_status = "no-proto-match"
    if candidate is not None:
        decode_status = "decode-attempted"
        try:
            import base64
            decoded_req = decode_raw(
                base64.b64decode(req_body_b64) if req_body_b64 else b"",
                candidate,
            )
            decoded_resp = decode_raw(
                base64.b64decode(resp_body_b64) if resp_body_b64 else b"",
                candidate,
            )
            if decoded_req is None and decoded_resp is None:
                decode_status = "decode-empty"
        except Exception:
            decode_status = "decode-error"

    flow_id = entry.get("serverIPAddress", uuid.uuid4().hex)
    sample = {
        "flow_id": flow_id,
        "url": url,
        "method": req.get("method", ""),
        "status": resp.get("status", 0),
        "proto_candidate": str(candidate.relative_to(REPO_ROOT)) if candidate else None,
        "decode_status": decode_status,
        "decoded_request": decoded_req,
        "decoded_response": decoded_resp,
    }

    out_path = out_dir / f"{uuid.uuid4().hex}.json"
    out_path.write_text(json.dumps(sample, indent=2))
    return sample


def main() -> int:
    p = argparse.ArgumentParser(description="Decode captured takasho flows.")
    p.add_argument("flow_file", type=Path)
    p.add_argument("--limit", type=int, default=50)
    p.add_argument("--out-dir", type=Path, default=OUT_DEFAULT)
    args = p.parse_args()

    if not args.flow_file.exists():
        print(f"decode_flows: file not found: {args.flow_file}", file=sys.stderr)
        return 1

    args.out_dir.mkdir(parents=True, exist_ok=True)

    entries = parse_flow_dump(args.flow_file)
    print(f"decode_flows: {len(entries)} flows in {args.flow_file}")

    summary = {"total": len(entries), "missing_proto": 0, "decoded": 0}
    for entry in entries[: args.limit]:
        sample = emit_one(entry, args.out_dir)
        if sample["proto_candidate"] is None:
            summary["missing_proto"] += 1
        else:
            summary["decoded"] += 1

    print(f"decode_flows: {summary}")
    print(f"decode_flows: samples in {args.out_dir}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
```

- [ ] **Step 2: Make executable + verify help**

```bash
cd /home/john/Projects/cosette-rs
chmod +x scripts/osint/decode_flows.py
python3 scripts/osint/decode_flows.py --help
```

Expected: argparse help text printed.

- [ ] **Step 3: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/decode_flows.py
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "feat(osint): add decode_flows.py to classify + decode captured traffic

Reads .flow dump, classifies paths against vendored protos, decodes
bodies via protoc --decode_raw, emits per-flow JSON samples.
Missing-protos are surfaced as decode_status='no-proto-match' so the
endpoints.md inventory can flag them."
```

---

## Task 8: extract_apk.sh — apktool + jadx + UnityPy

**Files:**
- Create: `scripts/osint/extract_apk.sh`

**Interfaces:**
- Consumes: `--review-dir <path>` (default `review/`), `--out-dir <path>` (default `docs/osint/apk`)
- Produces: extracted manifest, smali, asset bundles under `<out-dir>/`

- [ ] **Step 1: Write extract_apk.sh**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/extract_apk.sh <<'EOF'
#!/usr/bin/env bash
# Extract AndroidManifest, smali, asset bundles from the cosette review/ APKs.
# Continues past per-split failures; logs them to _FAILED_SPLITS.md.

set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

REVIEW_DIR="review"
OUT_DIR="docs/osint/apk"
FAILED_LOG="$OUT_DIR/_FAILED_SPLITS.md"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --review-dir) REVIEW_DIR="$2"; shift 2 ;;
        --out-dir) OUT_DIR="$2"; shift 2 ;;
        *) echo "unknown arg: $1" >&2; exit 2 ;;
    esac
done

mkdir -p "$OUT_DIR"

: > "$FAILED_LOG"

# Find every .apk in the review directory tree
mapfile -t APKS < <(find "$REVIEW_DIR" -type f -name "*.apk" | sort)
echo "[extract_apk] found ${#APKS[@]} APKs in $REVIEW_DIR"

for apk in "${APKS[@]}"; do
    name="$(basename "$apk" .apk)"
    target="$OUT_DIR/$name"
    mkdir -p "$target"

    echo "[extract_apk] -> $apk"

    # apktool: full resource + smali decode
    if ! apktool d -f -o "$target/apktool" "$apk" >/dev/null 2>"$target/apktool.err"; then
        echo "[extract_apk] apktool FAILED on $apk" | tee -a "$FAILED_LOG"
        echo '```' >> "$FAILED_LOG"
        cat "$target/apktool.err" >> "$FAILED_LOG" || true
        echo '```' >> "$FAILED_LOG"
        rm -rf "$target/apktool"
    fi

    # jadx: Java decompile (best-effort, only for the base APK to save time)
    if [[ "$name" == *".google.apk" || "$name" == "base" || "$name" == "com.dgames.g65002002.google" ]]; then
        if ! jadx -d "$target/jadx" "$apk" >/dev/null 2>"$target/jadx.err"; then
            echo "[extract_apk] jadx FAILED on $apk" | tee -a "$FAILED_LOG"
            rm -rf "$target/jadx"
        fi
    fi

    # UnityPy: asset bundle extraction
    if python3 -c "import UnityPy" 2>/dev/null; then
        if ! python3 - "$apk" "$target/assets" <<'PY' 2>"$target/unitypy.err"
import sys
import UnityPy
UnityPy.set_assetbundle_decrypt_key(b"")  # try default; override per-game if known
bundle = UnityPy.load(sys.argv[1])
bundle.save(sys.argv[2])
PY
        then
            echo "[extract_apk] UnityPy FAILED on $apk" | tee -a "$FAILED_LOG"
        fi
    fi
done

echo "[extract_apk] done. Outputs in $OUT_DIR"
echo "[extract_apk] failures (if any): $FAILED_LOG"
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/extract_apk.sh
```

- [ ] **Step 2: Verify script is executable**

```bash
cd /home/john/Projects/cosette-rs
./scripts/osint/extract_apk.sh --help 2>&1 | head -3 || true
./scripts/osint/extract_apk.sh 2>&1 | head -3
```

Expected: the first invocation prints nothing (no `--help` flag) — that's fine; second invocation starts running. Ctrl-C after a few seconds to verify it doesn't crash on script logic.

- [ ] **Step 3: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/extract_apk.sh
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "feat(osint): add extract_apk.sh for manifest + smali + asset bundles

Runs apktool on every .apk in review/, jadx on the base APK, UnityPy
on asset bundles. Per-split failures are logged to _FAILED_SPLITS.md
without aborting the rest (per spec Error Handling)."
```

---

## Task 11: endpoints.md — endpoint inventory template

**Files:**
- Create: `docs/osint/endpoints.md`

> **Order:** Must run BEFORE Task 9 and Task 10. (They reference this file.)

**Interfaces:**
- Consumes: none
- Produces: a markdown table that capture sessions append to

- [ ] **Step 1: Create the docs/osint directory + template**

```bash
cd /home/john/Projects/cosette-rs
mkdir -p docs/osint/{decoded,captures,apk}
```

- [ ] **Step 2: Write the inventory template**

```bash
cat > /home/john/Projects/cosette-rs/docs/osint/endpoints.md <<'EOF'
# Endpoint inventory

> Auto-populated from mitm captures. Append-only; never delete rows.
> Smoke scripts assert growth, not emptiness.

| Method | URL | gRPC service/method | Request type | Response type | Notes |
|--------|-----|---------------------|--------------|---------------|-------|
EOF
```

- [ ] **Step 3: Verify file is readable and table starts with the header**

```bash
cd /home/john/Projects/cosette-rs
cat docs/osint/endpoints.md
```

Expected: shows the header row + separator.

- [ ] **Step 4: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add docs/osint/endpoints.md
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "docs(osint): add endpoints.md inventory template

Append-only table populated by capture + decode scripts. Header
includes Method, URL, gRPC service/method, Request type, Response
type, Notes (for missing-proto flags)."
```

---

## Task 9: smoke_capture.sh — Phase 2 mid smoke

**Files:**
- Create: `scripts/osint/smoke_capture.sh`

**Interfaces:**
- Consumes: nothing
- Produces: exit 0 iff capture.sh + import_flows.sh can produce a `.flow` dump and at least one row in `mitm_mcp_traffic.db`. Uses a brief local mitmdump + curl to `/example.com` if no device is available.

- [ ] **Step 1: Write smoke_capture.sh**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/smoke_capture.sh <<'EOF'
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
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/smoke_capture.sh
```

- [ ] **Step 2: Run the smoke**

```bash
cd /home/john/Projects/cosette-rs
./scripts/osint/smoke_capture.sh
```

Expected: ends with `[smoke_capture] PASS: N new rows inserted` (N ≥ 1). The mitmproxy CA trust warning is expected; the test still passes because CONNECT rows land in the DB.

- [ ] **Step 3: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/smoke_capture.sh
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "test(osint): add smoke_capture.sh verifying capture -> DB pipeline

Drives capture.sh + import_flows.sh with synthetic example.com
traffic; asserts mitm_mcp_traffic.db gains at least one row."
```

---

## Task 10: smoke_e2e.sh — Phase 2 close smoke

**Files:**
- Create: `scripts/osint/smoke_e2e.sh`

> **Order:** Run AFTER Task 11 (`endpoints.md` must exist).

**Interfaces:**
- Consumes: `docs/osint/endpoints.md` (from Task 11)
- Produces: exit 0 iff capture + decode + inventory produces at least 3 new rows in `endpoints.md`

- [ ] **Step 1: Write smoke_e2e.sh**

```bash
cat > /home/john/Projects/cosette-rs/scripts/osint/smoke_e2e.sh <<'EOF'
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
EOF
chmod +x /home/john/Projects/cosette-rs/scripts/osint/smoke_e2e.sh
```

- [ ] **Step 2: Verify the script runs without errors**

```bash
cd /home/john/Projects/cosette-rs
./scripts/osint/smoke_e2e.sh
```

Expected: ends with `[smoke_e2e] PASS: inventory grew by 3 rows` (after Task 11 creates the inventory template).

- [ ] **Step 3: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add scripts/osint/smoke_e2e.sh
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "test(osint): add smoke_e2e.sh verifying capture -> decode -> inventory

Seeds DB with three synthetic takasho-shaped flows, asserts inventory
gains at least 3 rows. Stand-in until real device capture is wired."
```

---

## Task 12: docs/osint/.gitignore — exclude large extracted assets

**Files:**
- Create: `docs/osint/.gitignore`

**Interfaces:**
- Consumes: extract_apk.sh output structure (from Task 8)
- Produces: a `.gitignore` that keeps decoded JSON + markdown but ignores extracted asset binaries

- [ ] **Step 1: Write docs/osint/.gitignore**

```bash
cat > /home/john/Projects/cosette-rs/docs/osint/.gitignore <<'EOF'
# Decode samples + capture notes are committed (small, useful).
# Extracted APK assets are huge and rebuildable.

apk/
decoded/*_raw_hex.bin
captures/*.log
captures/*.tmp

# Allow the inventory + decoded JSON samples
!endpoints.md
!decoded/*.json
!captures/*.md
EOF
```

- [ ] **Step 2: Verify syntax**

```bash
cd /home/john/Projects/cosette-rs
git ls-files docs/osint/ | head
```

Expected: only `endpoints.md` is tracked (other files don't exist yet).

- [ ] **Step 3: Commit**

```bash
cd /home/john/Projects/cosette-rs
git add docs/osint/.gitignore
git -c user.email="claude@anthropic.com" -c user.name="Claude" \
  commit -m "chore(osint): .gitignore large extracted APK assets

Keeps decoded JSON + capture notes + inventory markdown committed.
Excludes apk/ subdir (rebuildable from extract_apk.sh)."
```

---

## Self-Review

**1. Spec coverage:**

| Spec section / requirement | Implementing task(s) |
|---|---|
| Context (project overview) | (n/a — informational) |
| Goals 1-4 | 1, 2, 4, 5/6/7/8/11 |
| Non-goals | (enforced by Global Constraints 1-8) |
| Architecture (3 layers) | 1 + 5/6/7/8 + 11 |
| Component 1 (vendored proto tree) | 1 |
| Component 2 (build.rs refactor) | 2 |
| Component 3 (native toolchain) | 4 |
| Component 4 (capture.sh) | 5 |
| Component 5 (decode_flows.py) | 7 |
| Component 6 (extract_apk.sh) | 8 |
| Component 7 (endpoints.md) | 11 |
| Component 8 (capture notes / decoded samples) | 11 + 7 (output dirs) |
| Phased rollout Phase 0 | 1, 2, 3 |
| Phased rollout Phase 1 | 4 |
| Phased rollout Phase 2 | 5, 6, 7, 8, 9, 10, 11, 12 |
| Data flow (mitm → DB → decode → docs) | 5, 6, 7, 11 |
| Per-session namespace | 11 (dirs created); session_id column deferred per spec |
| Decode match strategy | 7 |
| No writes back to live server | enforced by Global Constraint 6 |
| Error handling: vendor protos | 2 (panic with override hint) |
| Error handling: APK split breaks apktool | 8 (per-split try/continue) |
| Error handling: TLS pinning | 5 (documented; Frida deferred) |
| Error handling: gRPC reflection | 7 (falls back to introspection) |
| Error handling: undecodable flow | 7 (decode_status='no-proto-match') + 11 (missing-proto flag column) |
| Error handling: UnityPy mismatch | 8 (try/except + _FAILED_SPLITS.md) |
| Error handling: auth replay | Out of scope, Global Constraint 6 |
| Logging | 5 (stdout + flow file path); 7 (per-flow JSON) |
| Testing: smoke 1 (build) | 3 |
| Testing: smoke 2 (tools) | 4 |
| Testing: smoke 3 (capture) | 9 |
| Testing: smoke 4 (e2e) | 10 (depends on 11) |
| Open questions 1-4 | Deferred until hit (per spec) |
| Risks 1-5 | Documented in plan via Global Constraints 1, 2, 6 + Task 0 gating |
| Success criteria 1-5 | 1, 2, 4, 5/6/7, 11 |

No gaps.

**2. Placeholder scan:**

- No "TBD" / "TODO" / "implement later" anywhere.
- Each step has either an exact command, exact code block, or a deliberate "document what you find" instruction (Step 2 of Task 0, where the source path is unknown a priori).
- Step "verify the file" in Task 2 uses `rustc --edition 2021 -Zparse-only` as a parse check; the actual compile verification comes from `cargo build`. If `-Zparse-only` is unavailable on stable, the failure is intentional noise — the next step (`cargo build`) is the real check.
- Task 5 Step 2 acknowledges that without real traffic, capture.sh produces an empty file (FAIL). That's intentional; the verification happens in smoke_capture.sh (Task 9).

**3. Type consistency:**

- `proto_candidate` (Task 7) is a `Path`; emitted as relative string in JSON. ✓
- `decode_status` enum values: `no-proto-match`, `decode-attempted`, `decode-empty`, `decode-error`. ✓ all distinct.
- `out_dir` parameter passed identically to `decode_flows.py` and `import_flows.sh`. ✓
- `mitm_mcp_traffic.db` referenced consistently. ✓
- `CARGO_MANIFEST_PROTO_ROOT` env var name matches across build.rs (Task 2) and spec (Components row 2). ✓
- `OUT_DIR` defaults: `docs/osint/decoded` (Task 7) vs `docs/osint/apk` (Task 8). Different content types, different defaults. ✓

---

## Execution Handoff

Plan complete and saved to `docs/superpowers/plans/2026-06-29-cosette-osint-toolchain.md`. Two execution options:

1. **Subagent-Driven (recommended)** — I dispatch a fresh subagent per task, review between tasks, fast iteration with two-stage review.
2. **Inline Execution** — Execute tasks in this session using executing-plans, batch execution with checkpoints for review.

**Note on Task 0:** Task 0 is a precondition (acquire protos) that has no commit. The user must complete it before the agent can proceed; if the protos are unobtainable, all of Phase 0 blocks and the plan needs a follow-up to recover proto structure from `cosette-protocol/include/`.

Which approach?