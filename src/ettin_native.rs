//! Exact Ettin reranker v1 ONNX cross-encoder inference.
//! Published checkpoints expose an unnamed ONNX output; unlike FastEmbed's
//! hardcoded "logits" key, the native adapter reads the actual first output.
#[cfg(feature = "ettin-native")]
pub mod native {
    use crate::model_operations::EttinInference;
    use ort::{session::Session, value::Tensor};
    use std::sync::Mutex;
    use tokenizers::{Tokenizer, TruncationParams};

    pub fn ettin_repo(size: u16) -> Result<String, String> {
        if !matches!(size, 17 | 32 | 68 | 150) {
            return Err(format!("unsupported Ettin model size: {size}"));
        }
        Ok(format!("cross-encoder/ettin-reranker-{size}m-v1"))
    }
    pub struct NativeEttin {
        pub model_id: String,
        tokenizer: Tokenizer,
        session: Mutex<Session>,
    }
    impl NativeEttin {
        pub fn load(size: u16) -> Result<Self, String> {
            let environment = ort::init().build().map_err(|e| e.to_string())?;
            Self::load_in_environment(size, &environment)
        }
        /// Use an existing host-owned ONNX Runtime environment when other
        /// model sessions already share its process-wide resources.
        pub fn load_in_environment(
            size: u16,
            environment: &ort::environment::Environment,
        ) -> Result<Self, String> {
            let model_id = ettin_repo(size)?;
            let repo = hf_hub::api::sync::Api::new()
                .map_err(|e| e.to_string())?
                .model(model_id.clone());
            let model_path = repo
                .get("onnx/model.onnx")
                .map_err(|e| format!("exact Ettin ONNX model unavailable: {e}"))?;
            let tokenizer_path = repo
                .get("tokenizer.json")
                .map_err(|e| format!("Ettin tokenizer unavailable: {e}"))?;
            let mut tokenizer = Tokenizer::from_file(tokenizer_path).map_err(|e| e.to_string())?;
            tokenizer
                .with_truncation(Some(TruncationParams {
                    max_length: 512,
                    ..Default::default()
                }))
                .map_err(|e| e.to_string())?;
            let session = Session::builder(environment)
                .map_err(|e| e.to_string())?
                .commit_from_file(model_path)
                .map_err(|e| e.to_string())?;
            Ok(Self {
                model_id,
                tokenizer,
                session: Mutex::new(session),
            })
        }
    }
    impl EttinInference for NativeEttin {
        type Error = String;
        fn rerank(&self, query: &str, candidates: &[String]) -> Result<Vec<f64>, Self::Error> {
            let mut session = self
                .session
                .lock()
                .map_err(|_| "Ettin inference session poisoned".to_owned())?;
            let token_types = session
                .inputs()
                .iter()
                .any(|input| input.name() == "token_type_ids");
            let mut scores = Vec::with_capacity(candidates.len());
            for document in candidates {
                let encoding = self
                    .tokenizer
                    .encode((query, document.as_str()), true)
                    .map_err(|e| e.to_string())?;
                let len = encoding.len();
                if len == 0 {
                    return Err("Ettin generated an empty encoding".into());
                }
                let ids: Vec<i64> = encoding.get_ids().iter().map(|v| i64::from(*v)).collect();
                let mask: Vec<i64> = encoding
                    .get_attention_mask()
                    .iter()
                    .map(|v| i64::from(*v))
                    .collect();
                let types: Vec<i64> = encoding
                    .get_type_ids()
                    .iter()
                    .map(|v| i64::from(*v))
                    .collect();
                let outputs = if token_types {
                    session
                        .run(ort::inputs![
                            "input_ids" => Tensor::from_array(([1_usize,len], ids))
                                .map_err(|e| e.to_string())?,
                            "attention_mask" => Tensor::from_array(([1_usize,len], mask))
                                .map_err(|e| e.to_string())?,
                            "token_type_ids" => Tensor::from_array(([1_usize,len], types))
                                .map_err(|e| e.to_string())?,
                        ])
                        .map_err(|e| e.to_string())?
                } else {
                    session
                        .run(ort::inputs![
                            "input_ids" => Tensor::from_array(([1_usize,len], ids))
                                .map_err(|e| e.to_string())?,
                            "attention_mask" => Tensor::from_array(([1_usize,len], mask))
                                .map_err(|e| e.to_string())?,
                        ])
                        .map_err(|e| e.to_string())?
                };
                let (_, data) = outputs[0]
                    .try_extract_tensor::<f32>()
                    .map_err(|e| e.to_string())?;
                let score = data.first().ok_or("Ettin returned empty output tensor")?;
                if !score.is_finite() {
                    return Err("Ettin returned a nonfinite score".into());
                }
                scores.push(f64::from(*score));
            }
            Ok(scores)
        }
    }
    #[test]
    fn exact_v1_repo_names_reject_any_other_model() {
        for size in [17, 32, 68, 150] {
            assert_eq!(
                ettin_repo(size).unwrap(),
                format!("cross-encoder/ettin-reranker-{size}m-v1")
            );
        }
        assert!(ettin_repo(400).is_err());
    }
}
