# GutRS

Native Rust counterparts of GutTS public contracts, pinned at `3e2cbc0990ba29830ce91c5d1a1d552675d5334c`.

**Work in progress:** see [complete per-export traceability](docs/upstream-parity.md). Some validated portable core utilities and auth wire contracts are implemented; model inference, semantic-input and SDK host adapters remain missing. Do not treat this crate as feature-complete GutTS parity.

## Consumer

```rust
use gutrs::{identity, json_utils};
let app = identity::identity_ref("application", "aiko")?;
let config = json_utils::parse_json(r#"{"enabled":true}"#)?;
```

See `examples/external-consumer` for a complete, independently compiled Cargo example.

## Supported targets and verification

The current implementation uses `std` and runs on hosted Rust targets. CI exercises Ubuntu Linux, macOS and Windows. `no_std` and allocation-only builds are **not supported** by this version because the `url` and JSON dependencies need hosted features. Native data validators never authenticate a principal or authorize capabilities.

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo test --doc --all-features
cargo run --manifest-path examples/external-consumer/Cargo.toml
```

See `testdata/upstream-golden/portable.json` for source-derived positive/negative/boundary cases. Execution of these same cases under pinned GutTS remains necessary for independent cross-language certification.
