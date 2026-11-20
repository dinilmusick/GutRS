//! Exact Ettin reranker v1 ONNX cross-encoder inference.
//! Published ONNX checkpoints expose the transformer hidden states rather than
//! a ready-made score. The separate released 2_Dense, 3_LayerNorm and 4_Dense
//! safetensors head is required to reproduce the pinned GutTS scoring path.
#[cfg(feature = "ettin-native")]
pub mod native {
    use crate::ettin_head::{compute_ettin_head_scores, EttinHeadWeights};
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
        head: EttinHeadWeights,
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
            let read_head = |subpath: &str| -> Result<Vec<u8>, String> {
                let file = repo
                    .get(subpath)
                    .map_err(|e| format!("Ettin scoring head {subpath}: {e}"))?;
                std::fs::read(file).map_err(|e| e.to_string())
            };
            let dense = read_head("2_Dense/model.safetensors")?;
            let layer_norm = read_head("3_LayerNorm/model.safetensors")?;
            let output = read_head("4_Dense/model.safetensors")?;
            let head = EttinHeadWeights::from_safetensors(&dense, &layer_norm, &output)
                .map_err(|e| e.to_string())?;
            let session = Session::builder(environment)
                .map_err(|e| e.to_string())?
                .commit_from_file(model_path)
                .map_err(|e| e.to_string())?;
            Ok(Self {
                model_id,
                tokenizer,
                session: Mutex::new(session),
                head,
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
                let (shape, hidden) = outputs[0]
                    .try_extract_tensor::<f32>()
                    .map_err(|e| e.to_string())?;
                let dims: Vec<usize> = shape
                    .iter()
                    .map(|&dim| usize::try_from(dim).map_err(|e| e.to_string()))
                    .collect::<Result<_, _>>()?;
                if dims.len() != 3 || dims[0] != 1 || dims[2] != self.head.ln_weight.len() {
                    return Err(format!(
                        "Ettin backbone returned invalid hidden-state dimensions: {dims:?}"
                    ));
                }
                let scores_for_batch =
                    compute_ettin_head_scores([dims[0], dims[1], dims[2]], hidden, &self.head)
                        .map_err(|e| e.to_string())?;
                let score = scores_for_batch
                    .first()
                    .ok_or("Ettin head returned no score")?;
                scores.push(*score);
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
