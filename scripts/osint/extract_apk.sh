#!/usr/bin/env bash
# Extract AndroidManifest, smali, asset bundles from the cosette review/ APKs.
# Continues past per-split failures; logs them to _FAILED_SPLITS.md.
#
# Requires: apktool, jadx, python3 + UnityPy (pipx venv), and a working JDK
# reachable via JAVA_HOME. The script auto-detects a JDK at runtime because
# the user's JAVA_HOME may point to a broken Flatpak path; both apktool and
# jadx wrappers (Homebrew) refuse to run when JAVA_HOME is unset or invalid.

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

# --- JAVA_HOME auto-detection ----------------------------------------------
# apktool/jadx (Homebrew wrappers) refuse to run if JAVA_HOME is unset or
# points to a non-existent directory (e.g. a stale Flatpak path). Prefer:
#   1. The existing JAVA_HOME, if its bin/java is actually executable.
#   2. The Homebrew openjdk libexec directory the wrappers were written for.
#   3. A system openjdk under /usr/lib/jvm/java-*-openjdk.
#   4. A default-java symlink, if present.
# If nothing works, fail loud — the wrappers will produce a confusing error
# otherwise and we'd rather point the operator at the missing JDK.
detect_java_home() {
    local candidate
    local candidates=(
        "${JAVA_HOME:-}"
        /home/linuxbrew/.linuxbrew/opt/openjdk/libexec
        /home/linuxbrew/.linuxbrew/Cellar/openjdk/26.0.1/libexec
        /usr/lib/jvm/java-25-openjdk
        /usr/lib/jvm/java-21-openjdk
        /usr/lib/jvm/default-java
    )
    for candidate in "${candidates[@]}"; do
        [[ -z "$candidate" ]] && continue
        if [[ -x "$candidate/bin/java" ]]; then
            echo "$candidate"
            return 0
        fi
    done
    # Last resort: scan /usr/lib/jvm/* for the first directory with bin/java.
    for candidate in /usr/lib/jvm/*/; do
        [[ -x "$candidate/bin/java" ]] && { echo "${candidate%/}"; return 0; }
    done
    return 1
}

if JAVA_HOME="$(detect_java_home)"; then
    export JAVA_HOME
    echo "[extract_apk] JAVA_HOME=$JAVA_HOME"
else
    echo "[extract_apk] FATAL: no working JDK found (checked JAVA_HOME, " >&2
    echo "  /home/linuxbrew/.linuxbrew/opt/openjdk/libexec, /usr/lib/jvm/*)." >&2
    echo "  Install openjdk or set JAVA_HOME to a directory containing bin/java." >&2
    exit 1
fi

mkdir -p "$OUT_DIR"
: > "$FAILED_LOG"

# --- UnityPy discovery -----------------------------------------------------
# UnityPy is installed via pipx into an isolated venv (~/.local/share/pipx/
# venvs/unitypy) whose site-packages are NOT visible to the system python3.
# We discover the site-packages directory from the on-PATH UnityPy shim's
# resolved location and inject it via PYTHONPATH so `import UnityPy` works.
UNITYPY_SITE=""
if command -v UnityPy >/dev/null 2>&1; then
    py_bin="$(readlink -f "$(command -v UnityPy)" 2>/dev/null || true)"
    if [[ -n "$py_bin" && -x "$py_bin" ]]; then
        venv_root="$(dirname "$(dirname "$py_bin")")"
        UNITYPY_SITE="$venv_root/lib/python3.14/site-packages"
    fi
fi

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

    # UnityPy: asset bundle extraction (only when the venv site-packages are
    # discoverable; otherwise skip silently rather than bombing every APK).
    if [[ -n "$UNITYPY_SITE" ]] && [[ -d "$UNITYPY_SITE" ]]; then
        if ! PYTHONPATH="$UNITYPY_SITE" python3 - "$apk" "$target/assets" <<'PY' 2>"$target/unitypy.err"
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