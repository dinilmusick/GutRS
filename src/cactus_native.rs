//! Real Cactus Needle3/Whistle native engine entrypoints.
//!
//! `cactus-native` links the *actual* Cactus Compute engine through the
//! safe cactus-rs v0.2.1 crate. The engine requires separately provided model
//! archives and supports only a pinned subset of host targets.
//! This is not a fabricated model or a JavaScript shim.
#[cfg(all(feature = "cactus-native", any(
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "aarch64"),
)))]
pub mod native {
    //! Actual CPU inference and transcription engines. The caller loads real
    //! weights from `needle3.cact` or `whistle.cact` using the builders.
    pub use cactus_rs::needle::{Needle as NodeCactusNeedle3Runtime, Weights as Needle3Weights, Tool};
    pub use cactus_rs::whistle::{
        Language, TranscribeOptions, Weights as WhistleWeights,
        Whistle as NodeCactusWhistleRuntime,
    };

    #[test]
    fn actual_cactus_engine_types_are_public() {
        assert!(std::any::type_name::<NodeCactusNeedle3Runtime>().contains("Needle"));
        assert!(std::any::type_name::<NodeCactusWhistleRuntime>().contains("Whistle"));
        // No model archives are downloaded/loaded by this type-level test.
    }
}
