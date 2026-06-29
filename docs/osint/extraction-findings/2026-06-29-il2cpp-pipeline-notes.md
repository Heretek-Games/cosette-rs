# Extraction Pipeline Findings (2026-06-29)

## Goal

Recover takasho `.proto` source files used by `cosette-rs` by mirroring
Yoshk4e's extraction pipeline (OxidizedRelay + NisDecryptor + Frida).

## Tool findings (real CLIs, not what the plan assumed)

| Tool | Real CLI | Real input | Real output |
|---|---|---|---|
| `OxidizedRelay` | `<PCAP> [-k HEX_KEY] [-i HEX_NONCE]` | libpcap file | decrypted plaintext to stdout |
| `NisDecryptor` | `<input.dll> [output]` | Windows .NET DLL (managed Unity) | decrypted DLL |
| `libil2cpp.so` (Android IL2CPP) | n/a — not a tool | ELF shared library | (encrypted blob) |
| `global-metadata.dat` | n/a — not a tool | IL2CPP metadata file | (binary metadata) |

## Plan mismatches

1. **OxidizedRelay**: plan assumed `--in/--keys/--out` flags. Real CLI is
   positional `<PCAP>`. Real input is libpcap, not mitm `.flow`. Mitigated
   by `scripts/osint/flow_to_pcap.py` (Task 6.5, commit d277826).

2. **NisDecryptor**: plan assumed it extracts proto descriptors as JSON
   from `libil2cpp.so`. Real CLI only decrypts Windows .NET DLLs. It fails
   on Android IL2CPP blobs with "Not a PE file". **Does not apply to this game.**

3. **No bundled takasho protos in the APK.** The `.proto` assets shipped
   in `com.dgames.g65002002.google.apk` are only Firebase / Google Play
   Services / transport protos (third-party SDKs). The actual takasho
   protos are NOT present as static files.

## What the actual pipeline needs

For IL2CPP Android games, the takasho game protos are embedded as:
- Class/method metadata in `global-metadata.dat` (a 13 MB IL2CPP metadata
  blob — IL2CPP-specific binary format)
- Encrypted code in `libil2cpp.so` (93 MB ELF)

The standard toolchain is **Il2CppDumper** (Perfare/Il2CppDumper on GitHub),
which reads both files and produces:
- `dump.cs` — every class, method, field with names + types
- `il2cpp.h` — C++ headers reconstructing the IL2CPP API surface

From `dump.cs`, takasho's gRPC service classes can be identified (likely
under `Takasho.Schema.*` or similar). The proto structure can then be
reverse-engineered from the C# class definitions.

Alternatively: capture real gRPC traffic (Task 5 capture), decrypt with
OxidizedRelay using Frida-dumped ChaCha20 keys (Task 6 hook), and reverse-
engineer field-by-field from the decrypted payloads.

## Status

- `OxidizedRelay` binary built at `/tmp/oxrel/target/release/OxidizedRelay` (AGPL-3, NOT vendored to repo).
- `NisDecryptor` built at `/tmp/nisdec-*/bin/Release/net8.0/NisDecryptor.dll` (runnable via linuxbrew .NET 8).
- `scripts/osint/flow_to_pcap.py` shipped (commit d277826).
- Pixel 10 Pro XL emulator with `takt op. v1.2.217` installed (Task 0).
- Frida hook NOT yet attempted (likely blocked by game anti-tamper).
- No protos recovered from APK assets or libil2cpp.so strings.

## Realistic next steps (NOT in the current plan; need a follow-up spec)

1. **Add Il2CppDumper**: clone + build `Perfare/Il2CppDumper`, run on
   `libil2cpp.so` + `global-metadata.dat`. Produces C# class dump.
2. **Reverse-engineer proto descriptors from `dump.cs`**: human-curated
   mapping from IL2CPP classes → proto messages.
3. **Optional: capture traffic + Frida decrypt** to validate the
   reverse-engineered protos against real payloads.

This is a substantial detour from the original spec. Stopping here to
get direction before continuing.

## Update: Il2CppDumper attempt

- Cloned `Perfare/Il2CppDumper` v latest; built successfully.
- Ran on `libil2cpp.so` + `global-metadata.dat` → "Metadata file not found
  or encrypted."
- Inspected metadata magic: bytes `48 54 50 58` ("HTPX"), NOT the standard
  `il2cpp` magic (`69 6c 32 63 70 70`). Confirms encrypted v29+ metadata.
- Perfare's dumper doesn't handle v29+ encrypted metadata. Forks that
  do: `djkaty/Il2CppDumper`, `nicehash/Il2CppDumper` (newer Unity metadata).

## State: extraction blocked

Both paths (traffic-derived via OxidizedRelay + binary-derived via
Il2CppDumper) require additional work:

- OxidizedRelay path: needs Frida hook that produces ChaCha20 keys (Task 6,
  NOT yet attempted; likely blocked by takt op. v1.2.217 anti-tamper).
- Il2CppDumper path: needs a v29+ fork + likely the decryption key
  extracted from `libil2cpp.so` symbols.

Recommended next move: use the EXISTING generated Rust types in
`cosette-protocol/include/takasho/schema/` as the proto source. The
fields are already known; reverse-engineer the .proto files from the
.rs files (and where Rust types are unambiguous, hand-write the .proto
directly). This is what the original OSINT spec called Phase 0 / option
C — reverse-engineer from generated Rust.

## Files in /tmp (for future reference)

- `/tmp/oxrel/target/release/OxidizedRelay` — built, AGPL-3, not vendored.
- `/tmp/nisdec-1782766495/bin/Release/net8.0/NisDecryptor.dll` — built,
  but doesn't apply to Android IL2CPP games.
- `/tmp/il2cppdumper-1782770467/Il2CppDumper/bin/Release/net6.0/Il2CppDumper.dll`
  — built, doesn't handle v29+ encrypted metadata.
- `/tmp/native/libil2cpp.so` — 93 MB encrypted IL2CPP blob.
- `/tmp/native/apk-assets/assets/bin/Data/Managed/Metadata/global-metadata.dat`
  — 13 MB encrypted metadata.
