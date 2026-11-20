# GutRS

Native Rust counterparts of GutTS public contracts, pinned at `3e2cbc0990ba29830ce91c5d1a1d552675d5334c`.

**Status:** Wave 1 implementation merged to canonical main with all eight exact-head checks passing. The pinned inventory maps 195/195 public symbol contracts, including platform-specific adapters. This establishes API traceability and tested portable behavior, not universal behavioral parity: JavaScript-only runtime values and live credentialed storage operations remain limited or unverified. See [per-export traceability and exceptions](docs/upstream-parity.md).

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

See `testdata/upstream-golden/portable.json` for source-derived positive/negative/boundary cases. The independent CI differential job executes these same cases under the pinned GutTS commit and compares all 38 API/FOD definitions and eight model metadata exports. This is a bounded differential suite, not proof of every JavaScript edge case.

## Hosted cloud storage

```toml
# Optional feature flags: aws-s3 and google-drive
```

With `aws-s3`, access `gutrs::storage_adapters::s3::S3Client` and seven native S3 request-builder types. The SDK uses a pinned behavior version via `behavior-version-latest`. With `google-drive`, use `gutrs::storage_adapters::google_drive::create_google_drive_client` to construct an actual Google Drive v3 client using caller-supplied OAuth and HTTPS transport. Neither SDK is included in default builds. Cloud credentials are not embedded, and offline checks do not establish live cloud access.

## Native Cactus (target-specific)

On Linux x86-64 or macOS ARM64, enable `cactus-native` and use `gutrs::cactus_native::native::NodeCactusNeedle3Runtime` / `NodeCactusWhistleRuntime` with real `Needle3Weights` / `WhistleWeights`. These are the genuine native Cactus engine types, not the JavaScript runtime. Windows MSVC and Intel macOS do not have the required upstream static archives and are deliberately excluded. A model weight file and an enabled native SDK feature are required before inference or transcription can run.
