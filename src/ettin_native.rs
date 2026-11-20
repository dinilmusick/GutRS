//! Exact Ettin-v1 cross-encoder reranking over real ONNX model weights.
//! There is no fallback to a generic reranker or synthetic relevance scores.
#[cfg(feature = "ettin-native")]
pub mod native {
    use crate::model_operations::EttinInference;
    use fastembed::{
        RerankInitOptionsUserDefined, TextRerank, TokenizerFiles,
        UserDefinedRerankingModel,
    };
    use std::sync::Mutex;

    pub fn ettin_repo(size: u16) -> Result<String, String> {
        if !matches!(size, 17 | 32 | 68 | 150) {
            return Err(format!("unsupported Ettin model size: {size}"));
        }
        Ok(format!("cross-encoder/ettin-reranker-{size}m-v1"))
    }
    /// Exact model archive is fetched by the Hugging Face client and retained
    /// in its verified local cache. Requires a host with network access on
    /// the first load (or a prepopulated HF cache).
    pub struct NativeEttin {
        pub model_id: String,
        reranker: Mutex<TextRerank>,
    }
    impl NativeEttin {
        pub fn load(size: u16) -> Result<Self, String> {
            let model_id = ettin_repo(size)?;
            let repo = hf_hub::api::sync::Api::new()
                .map_err(|e| e.to_string())?.model(model_id.clone());
            let onnx = repo.get("onnx/model.onnx")
                .map_err(|e| format!("could not load exact Ettin ONNX weights: {e}"))?;
            let read = |name: &str| -> Result<Vec<u8>, String> {
                let path = repo.get(name).map_err(|e| format!("{name}: {e}"))?;
                std::fs::read(path).map_err(|e| e.to_string())
            };
            let tokenizer_files = TokenizerFiles {
                tokenizer_file: read("tokenizer.json")?,
                config_file: read("config.json")?,
                // Some published Ettin checkpoints do not include a separate
                // special-tokens map; the tokenizer JSON is authoritative.
                special_tokens_map_file: repo.get("special_tokens_map.json")
                    .ok().and_then(|p| std::fs::read(p).ok())
                    .unwrap_or_else(|| b"{}".to_vec()),
                tokenizer_config_file: read("tokenizer_config.json")?,
            };
            let model = UserDefinedRerankingModel::new(onnx, tokenizer_files);
            let reranker = TextRerank::try_new_from_user_defined(
                model,
                RerankInitOptionsUserDefined::new().with_max_length(512)
            ).map_err(|e| e.to_string())?;
            Ok(Self { model_id, reranker: Mutex::new(reranker) })
        }
    }
    impl EttinInference for NativeEttin {
        type Error = String;
        fn rerank(&self, query: &str, candidates: &[String]) -> Result<Vec<f64>, Self::Error> {
            if candidates.is_empty() { return Ok(Vec::new()); }
            let mut engine = self.reranker.lock()
                .map_err(|_|"Ettin model lock is poisoned".to_string())?;
            let ranked = engine.rerank(query,candidates,false,Some(16))
                .map_err(|e|e.to_string())?;
            if ranked.len() != candidates.len() {
                return Err("Ettin ONNX returned unexpected score count".into());
            }
            let mut scores = vec![0.0; candidates.len()];
            let mut visited = vec![false; candidates.len()];
            for result in ranked {
                if result.index >= candidates.len() || visited[result.index] ||
                    !result.score.is_finite() {
                    return Err("Ettin returned duplicate index or invalid score".into());
                }
                visited[result.index] = true;
                scores[result.index] = f64::from(result.score);
            }
            if visited.iter().any(|seen| !seen) {
                return Err("Ettin omitted candidate scores".into());
            }
            Ok(scores)
        }
    }
    #[test]
    fn model_ids_are_exact_and_unknown_sizes_rejected() {
        assert_eq!(ettin_repo(17).unwrap(),"cross-encoder/ettin-reranker-17m-v1");
        assert_eq!(ettin_repo(150).unwrap(),"cross-encoder/ettin-reranker-150m-v1");
        assert!(ettin_repo(42).is_err());
    }
}
