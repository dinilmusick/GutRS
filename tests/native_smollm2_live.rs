//! Actual Candle CPU model generation with public SmolLM2-135M weights.
#![cfg(feature = "smollm2-native")]
use gutrs::model_operations::SmolLM2Inference;
use gutrs::smollm2_native::native::NativeSmolLM2;

#[test]
#[ignore = "downloads exact SmolLM2-135M-Instruct safetensors and generates CPU tokens"]
fn genuine_smollm2_135m_generation() -> Result<(), Box<dyn std::error::Error>> {
    let model = NativeSmolLM2::load()?;
    let response = model.generate(
        "<|im_start|>user\nWhat is 2+2? Answer with a number.\n<|im_end|>\n<|im_start|>assistant\n",
        Some(24),
    )?;
    assert!(
        !response.trim().is_empty(),
        "SmolLM2 returned no generated text"
    );
    Ok(())
}
