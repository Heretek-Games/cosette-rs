#!/usr/bin/env bash
# Smoke test: native OSINT toolchain is installed and on PATH.
# Pass = exit 0. Each tool's version is printed on success.

set -uo pipefail

cd "$(git rev-parse --show-toplevel)"

missing=()

for tool in jadx apktool grpcurl protoc mitmdump; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "[smoke_tools] MISSING: $tool" >&2
        missing+=("$tool")
    else
        echo "[smoke_tools] OK: $tool -> $(command -v "$tool")"
    fi
done

# UnityPy is installed via pipx into an isolated venv at
# ~/.local/share/pipx/venvs/unitypy. `python3` does not know about that
# venv's site-packages by default, so we inject the venv path explicitly
# (derived from the on-PATH `UnityPy` shim's real location) before importing.
unitypy_sitepkg() {
    local py_bin
    py_bin=$(readlink -f "$(command -v UnityPy)" 2>/dev/null || true)
    if [ -n "$py_bin" ] && [ -x "$py_bin" ]; then
        local venv_root
        venv_root=$(dirname "$(dirname "$py_bin")")
        echo "$venv_root/lib/python3.14/site-packages"
    fi
}

if unitypy_site=$(unitypy_sitepkg) && [ -n "$unitypy_site" ] && \
   PYTHONPATH="$unitypy_site" python3 -c "import UnityPy; print(UnityPy.__version__)" 2>/dev/null; then
    echo "[smoke_tools] OK: UnityPy -> $(PYTHONPATH="$unitypy_site" python3 -c "import UnityPy; print(UnityPy.__version__)")"
else
    echo "[smoke_tools] MISSING: python3 -m UnityPy" >&2
    missing+=("UnityPy")
fi

if [ ${#missing[@]} -gt 0 ]; then
    echo "[smoke_tools] FAIL: missing ${missing[*]}" >&2
    exit 1
fi

echo "[smoke_tools] PASS"
