//! Live, independently downloaded ONNX all-MiniLM-L6-v2 model verification.
#![cfg(feature = "minilm-native")]
use gutrs::minilm_native::native::NativeMiniLM;
use gutrs::model_operations::MiniLMInference;

#[test]
#[ignore = "downloads real MiniLM L6-v2 model and runs ONNX CPU inference"]
fn native_minilm_self_similarity() -> Result<(), Box<dyn std::error::Error>> {
    let model = NativeMiniLM::load()?;
    let same = model.similarity("piano music", "piano music")?;
    let different = model.similarity("piano music", "a coastal highway")?;
    assert!(same.is_finite() && different.is_finite());
    assert!(same > 0.95, "exact same text should have near-unit cosine");
    assert!((-1.0..=1.0).contains(&different));
    Ok(())
}
