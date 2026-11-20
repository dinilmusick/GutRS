//! Real 384-dimensional all-MiniLM-L6-v2 ONNX CPU inference using FastEmbed.
//! The exact sentence-transformers model is downloaded by the explicit
//! `load` constructor, never during ordinary core library initialization.
#[cfg(feature = "minilm-native")]
pub mod native {
    use crate::model_operations::MiniLMInference;
    use fastembed::{EmbeddingModel, TextEmbedding, TextInitOptions};
    use std::sync::Mutex;

    pub struct NativeMiniLM {
        model: Mutex<TextEmbedding>,
    }
    impl NativeMiniLM {
        /// Loads the actual sentence-transformers/all-MiniLM-L6-v2 weights
        /// into ONNX Runtime, with cache directory managed by FastEmbed.
        pub fn load() -> Result<Self, String> {
            TextEmbedding::try_new(
                TextInitOptions::new(EmbeddingModel::AllMiniLML6V2)
                    .with_show_download_progress(false)
            ).map(|model| Self { model: Mutex::new(model) })
                .map_err(|error| error.to_string())
        }
    }
    /// Deterministic vector cosine; never returns fabricated default success.
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> Result<f64, String> {
        if a.len() != b.len() || a.is_empty() {
            return Err("embedding vectors must have matching nonzero length".into());
        }
        let (mut dot, mut aa, mut bb) = (0.0_f64, 0.0_f64, 0.0_f64);
        for (&left, &right) in a.iter().zip(b) {
            if !left.is_finite() || !right.is_finite() {
                return Err("embedding vectors contain nonfinite samples".into());
            }
            let (left, right) = (f64::from(left), f64::from(right));
            dot += left * right;
            aa += left * left;
            bb += right * right;
        }
        let norm = (aa * bb).sqrt();
        if !norm.is_finite() || norm <= f64::EPSILON {
            return Err("embedding vectors have zero norm".into());
        }
        Ok((dot / norm).clamp(-1.0, 1.0))
    }
    impl MiniLMInference for NativeMiniLM {
        type Error = String;
        fn similarity(&self, left: &str, right: &str) -> Result<f64, Self::Error> {
            let mut model = self.model.lock()
                .map_err(|_|"MiniLM ONNX model lock is poisoned".to_string())?;
            let embeddings = model.embed(vec![left, right], None)
                .map_err(|error| error.to_string())?;
            if embeddings.len() != 2 {
                return Err("MiniLM runtime returned an unexpected embedding count".into());
            }
            cosine_similarity(&embeddings[0], &embeddings[1])
        }
    }
    #[test]
    fn cosine_engine_rejects_invalid_vectors() {
        assert!((cosine_similarity(&[1.0,0.0], &[1.0,0.0]).unwrap()-1.0).abs()<1e-12);
        assert!(cosine_similarity(&[0.0], &[0.0]).is_err());
        assert!(cosine_similarity(&[f32::NAN], &[1.0]).is_err());
        assert!(cosine_similarity(&[1.0], &[1.0,0.0]).is_err());
    }
}
