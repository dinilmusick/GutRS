//! Real ONNX cross-encoder inference using the exact Ettin 17M weights.
#![cfg(feature = "ettin-native")]
use gutrs::ettin_native::native::NativeEttin;
use gutrs::model_operations::EttinInference;

#[test]
#[ignore = "downloads genuine Ettin 17M ONNX weights and runs CPU reranking"]
fn exact_ettin_17m_scoring() -> Result<(), Box<dyn std::error::Error>> {
    let model = NativeEttin::load(17)?;
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
