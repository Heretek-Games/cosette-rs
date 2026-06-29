# Vendored protos

## Provenance

These `.proto` files were **reverse-engineered** from the generated Rust
types in `cosette-protocol/include/takasho/schema/` by
`scripts/osint/rs_to_proto.py`. The script parses
`#[prost(TYPE, ..., tag = "N")]` attributes and
`pub struct Foo { pub field: Type }` lines, then emits matching `.proto`
files mirroring the source directory structure.

The generated Rust preserves field tags as prost attributes, so the
emitted `.proto` reproduces the original wire format (modulo type-mapping
edge cases). `cargo build -p cosette-protocol` succeeds offline against
this tree.

## Regenerating

```bash
python3 scripts/osint/rs_to_proto.py
cargo build -p cosette-protocol
```

The script is idempotent (overwrites existing files). Stale `.proto`
files (from deleted `.rs` sources) are NOT auto-removed — manual cleanup
needed.

## Known limitations

- **Best-effort type mapping.** Unknown Rust types are emitted unchanged
  with a stderr warning. If a critical type is dropped, `cargo build`
  will fail and the warning tells you which file to inspect.
- **No `oneof`, `enum`, or `map` support.** Skipped with warning if
  encountered. The smoke catches the case where this drops a critical
  field.
- **Cross-file refs use `super::` heuristics.** The script emits
  `import` statements based on `super::path` patterns in the source.
  Edges cases may require manual fixup.
- **Wire-format round-trip is NOT verified.** No traffic capture in
  scope. The build is self-consistent but does not guarantee the
  regenerated types decode real traffic exactly. If a real capture
  becomes available, compare decoded messages against existing schema
  types.

## License

Apache-2.0 (matches `cosette-protocol/Cargo.toml`).