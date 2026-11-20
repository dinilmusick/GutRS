//! Actual Candle CPU model generation with public SmolLM2-135M weights.
#![cfg(feature = "smollm2-native")]
use gutrs::model_operations::SmolLM2Inference;
use gutrs::smollm2_native::native::NativeSmolLM2;

#[test]
#[ignore = "downloads exact SmolLM2-135M-Instruct safetensors and generates CPU tokens"]
fn genuine_smollm2_135m_generation() -> Result<(), Box<dyn std::error::Error>> {
    let model = NativeSmolLM2::load()?;
    let prompt = "<|im_start|>user\nWhat is 2+2? Answer with a number.\n<|im_end|>\n<|im_start|>assistant\n";
    let response = model.generate(prompt, Some(24))?;
    assert!(
        response.starts_with(prompt),
        "Transformers.js full-text generation must retain the supplied prompt"
    );
    assert!(
        response.len() > prompt.len(),
        "SmolLM2 must produce new tokens for the live test prompt"
    );
    assert!(
        !response.trim().is_empty(),
        "SmolLM2 returned no generated text"
    );
    Ok(())
}
