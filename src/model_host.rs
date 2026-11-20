//! Type-safe composition of actual host inference engines.
//! This connects the five model family contracts directly to the
//! `GuttsModelDispatch` execution boundary without fabricating any output.
use crate::model_dispatch::GuttsModelDispatch;
use crate::model_operations::{
    EttinInference, MiniLMInference, Needle3Inference, SmolLM2Inference, WhistleInference,
};
use serde_json::Value;

pub struct ModelHost<N, S, W, E, M> {
    pub needle: N,
    pub smollm: S,
    pub whistle: W,
    pub ettin: E,
    pub minilm: M,
}
impl<N, S, W, E, M> ModelHost<N, S, W, E, M> {
    pub fn new(needle: N, smollm: S, whistle: W, ettin: E, minilm: M) -> Self {
        Self {
            needle,
            smollm,
            whistle,
            ettin,
            minilm,
        }
    }
}
impl<N, S, W, E, M> GuttsModelDispatch for ModelHost<N, S, W, E, M>
where
    N: Needle3Inference,
    S: SmolLM2Inference,
    W: WhistleInference,
    E: EttinInference,
    M: MiniLMInference,
    N::Error: std::fmt::Display,
    S::Error: std::fmt::Display,
    W::Error: std::fmt::Display,
    E::Error: std::fmt::Display,
    M::Error: std::fmt::Display,
{
    type Error = String;
    fn generate(&self, prompt: &str, max_new_tokens: Option<u32>) -> Result<String, Self::Error> {
        self.smollm
            .generate(prompt, max_new_tokens)
            .map_err(|e| e.to_string())
    }
    fn complete(
        &self,
        prompt: &str,
        tools: Option<&Value>,
        schema: Option<&Value>,
    ) -> Result<Value, Self::Error> {
        self.needle
            .complete(prompt, tools, schema)
            .map_err(|e| e.to_string())
    }
    fn transcribe(&self, audio: &[u8], language: Option<&str>) -> Result<String, Self::Error> {
        self.whistle
            .transcribe(audio, language)
            .map_err(|e| e.to_string())
    }
    fn rerank(&self, query: &str, candidates: &[String]) -> Result<Vec<f64>, Self::Error> {
        self.ettin
            .rerank(query, candidates)
            .map_err(|e| e.to_string())
    }
    fn similarity(&self, left: &str, right: &str) -> Result<f64, Self::Error> {
        self.minilm
            .similarity(left, right)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model_dispatch::execute_gutts_api_with_models;
    use serde_json::json;
    struct Engine;
    impl Needle3Inference for Engine {
        type Error = &'static str;
        fn complete(
            &self,
            prompt: &str,
            _: Option<&Value>,
            _: Option<&Value>,
        ) -> Result<Value, Self::Error> {
            Ok(json!({"prompt":prompt}))
        }
        fn embed(&self, _: &str) -> Result<Vec<f32>, Self::Error> {
            Ok(vec![])
        }
    }
    impl SmolLM2Inference for Engine {
        type Error = &'static str;
        fn generate(&self, prompt: &str, _: Option<u32>) -> Result<String, Self::Error> {
            Ok(prompt.into())
        }
    }
    impl WhistleInference for Engine {
        type Error = &'static str;
        fn transcribe(&self, audio: &[u8], _: Option<&str>) -> Result<String, Self::Error> {
            Ok(audio.len().to_string())
        }
    }
    impl EttinInference for Engine {
        type Error = &'static str;
        fn rerank(&self, _: &str, candidates: &[String]) -> Result<Vec<f64>, Self::Error> {
            Ok(vec![0.25; candidates.len()])
        }
    }
    impl MiniLMInference for Engine {
        type Error = &'static str;
        fn similarity(&self, _: &str, _: &str) -> Result<f64, Self::Error> {
            Ok(0.8)
        }
    }
    #[test]
    fn real_traits_drive_model_api_dispatch() {
        let host = ModelHost::new(Engine, Engine, Engine, Engine, Engine);
        assert_eq!(
            execute_gutts_api_with_models(
                "model.cactus-needle3.complete",
                &json!({"prompt":"hi"}),
                &host
            )
            .unwrap(),
            json!({"prompt":"hi"})
        );
        assert_eq!(
            execute_gutts_api_with_models(
                "model.ettin-reranker.rerank",
                &json!({"query":"q","candidates":["a"]}),
                &host
            )
            .unwrap(),
            json!([0.25])
        );
    }
}
