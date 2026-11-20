//! Real ONNX cross-encoder inference using exact Ettin 17M/32M/68M/150M weights.
#![cfg(feature = "ettin-native")]
use gutrs::ettin_native::native::NativeEttin;
use gutrs::model_operations::EttinInference;

#[test]
#[ignore = "downloads the exact selected Ettin ONNX weights and runs CPU reranking"]
fn exact_ettin_v1_scoring() -> Result<(), Box<dyn std::error::Error>> {
    let size = std::env::var("ETTIN_MODEL_SIZE")
        .unwrap_or_else(|_| "17".to_owned())
        .parse::<u16>()?;
    let model = NativeEttin::load(size)?;
    let sentences = vec![
        "Venus is Earth's closest planetary neighbor".to_owned(),
        "Mars is known as the Red Planet".to_owned(),
    ];
    let scores = model.rerank("What is the Red Planet?", &sentences)?;
    assert_eq!(scores.len(), sentences.len());
    assert!(scores.iter().all(|s| s.is_finite()));
    assert!(
        scores[1] > scores[0],
        "Ettin should rank a direct factual match above distractors"
    );
    Ok(())
}
