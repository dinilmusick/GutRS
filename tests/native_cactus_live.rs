//! Non-mocked end-to-end model inference smoke against official Cactus weights.
//! Intentionally ignored by default because downloading weights needs internet
//! and CPU execution. CI explicitly runs this test on supported Linux hardware.
#![cfg(all(
    feature = "cactus-native",
    target_os = "linux",
    target_arch = "x86_64"
))]
use gutrs::cactus_native::native::{
    CactusNeedle3, CactusWhistle, Needle3Weights, NodeCactusNeedle3Runtime,
    NodeCactusWhistleRuntime, WhistleWeights,
};
use gutrs::model_operations::{Needle3Inference, WhistleInference};

#[test]
#[ignore = "downloads public Cactus model weights and performs real CPU inference"]
fn live_needle3_and_whistle_cpu_models() -> Result<(), Box<dyn std::error::Error>> {
    let needle = NodeCactusNeedle3Runtime::builder(Needle3Weights::fetch()?).build()?;
    let needle = CactusNeedle3::new(needle);
    let output = needle.complete("Name an instrument used in music", None, None)?;
    assert!(output.is_object());
    assert!(output.get("type").is_some());
    assert!(!needle.embed("piano")?.is_empty());

    let whistle = NodeCactusWhistleRuntime::builder(WhistleWeights::fetch()?).build()?;
    let whistle = CactusWhistle::new(whistle);
    let pcm16_silence = vec![0_u8; 32_000];
    let transcript = whistle.transcribe(&pcm16_silence, Some("en"))?;
    assert!(transcript.len() < 16_000);
    Ok(())
}
