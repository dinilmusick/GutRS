//! GutTS model metadata, runtime adapters and concurrent model residency.
//! The registry reproduces the exact pinned metadata. No inference is faked;
//! a host runtime must supply genuine model loading/inference/unloading.
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "qualifier")]
pub enum ModelSize {
    #[serde(rename = "artifact")]
    Artifact { value: f64, unit: String },
    #[serde(rename = "parameters")]
    Parameters { value: f64, unit: String },
    #[serde(rename = "range")]
    Range { min: f64, max: f64, unit: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelMetadata {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub version: String,
    pub description: String,
    pub examples: Vec<String>,
    pub when_to_use: String,
    pub avoid_when: String,
    pub size: ModelSize,
    pub capabilities: Vec<String>,
    pub input: String,
    pub output: String,
    pub runtime: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
}

fn words(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| (*word).to_owned()).collect()
}

fn smollm2() -> ModelMetadata {
    ModelMetadata {
        id: "model.smollm2-135m-instruct".into(),
        name: "SmolLM2-135M-Instruct".into(),
        provider: "Hugging Face".into(),
        version: "135M".into(),
        description: "Compact instruction-tuned language model for general text generation.".into(),
        examples: words(&["summarize text", "rewrite text", "draft a short response"]),
        avoid_when: "Avoid for high-stakes reasoning, broad factual knowledge, or tasks requiring large context.".into(),
        when_to_use: "Use for short general-purpose text generation when a local model is preferred.".into(),
        size: ModelSize::Artifact { value: 105.0, unit: "MB".into() },
        capabilities: words(&["text-generation", "instruction-following"]),
        input: "Natural-language prompt".into(),
        output: "Generated natural-language text".into(),
        runtime: "ModelRuntime adapter".into(),
        model_uri: Some("HuggingFaceTB/SmolLM2-135M-Instruct".into()),
        license: Some("Apache-2.0".into()),
    }
}
fn needle3() -> ModelMetadata {
    ModelMetadata {
        id: "model.cactus-needle3".into(),
        name: "Needle 3".into(),
        provider: "Cactus Compute".into(),
        version: "3".into(),
        description: "Tiny structured inference model for tool selection, argument extraction, classification, and embeddings.".into(),
        examples: words(&["select a tool", "extract fields", "classify an intent"]),
        avoid_when: "Avoid for open-ended long-form generation or complex multi-step reasoning.".into(),
        when_to_use: "Use when natural language must be mapped to a small tool set or structured schema.".into(),
        size: ModelSize::Range { min: 8.0, max: 29.0, unit: "MB".into() },
        capabilities: words(&["tool-selection", "argument-extraction", "classification", "structured-output", "embeddings"]),
        input: "Natural-language text with optional tools or extraction schema".into(),
        output: "Structured JSON or embedding vector".into(),
        runtime: "Cactus Needle runtime adapter".into(),
        model_uri: Some("Cactus-Compute/needle3".into()),
        license: None,
    }
}
fn whistle() -> ModelMetadata {
    ModelMetadata {
        id: "model.cactus-whistle".into(),
        name: "Whistle".into(),
        provider: "Cactus Compute".into(),
        version: "1".into(),
        description: "Compact CPU speech-recognition model for on-device transcription.".into(),
        examples: words(&["transcribe a voice note", "transcribe a command"]),
        avoid_when: "Avoid when audio understanding beyond transcription is required.".into(),
        when_to_use: "Use for local speech-to-text when low resource use and privacy matter."
            .into(),
        size: ModelSize::Artifact {
            value: 16.9,
            unit: "MB".into(),
        },
        capabilities: words(&["speech-to-text", "on-device", "cpu"]),
        input: "Audio bytes".into(),
        output: "Transcribed natural-language text".into(),
        runtime: "Cactus Needle runtime adapter".into(),
        model_uri: Some("Cactus-Compute/whistle".into()),
        license: None,
    }
}
fn ettin(parameters: i32, desc: &str) -> ModelMetadata {
    ModelMetadata {
        id: format!("model.ettin-reranker-{parameters}m"),
        name: format!("Ettin-Reranker-{parameters}M"),
        provider: "Hugging Face".into(),
        version: parameters.to_string(),
        description: desc.into(),
        examples: words(&["rank search results", "order retrieved documents by relevance"]),
        when_to_use: "Use for cross-encoder reranking when candidate texts must be sorted by semantic relevance.".into(),
        avoid_when: "Avoid for text generation, embeddings, or transcription.".into(),
        size: ModelSize::Parameters { value: f64::from(parameters) * 1_000_000.0, unit: "parameters".into() },
        capabilities: words(&["reranking", "relevance-scoring"]),
        input: "Query and candidate texts".into(),
        output: "Candidate relevance scores".into(),
        runtime: "ModelRuntime adapter".into(),
        model_uri: Some(format!("cross-encoder/ettin-reranker-{parameters}m-v1")),
        license: Some("Apache-2.0".into()),
    }
}
fn minilm() -> ModelMetadata {
    ModelMetadata {
        id: "model.minilm-l6-v2".into(),
        name: "MiniLM-L6-v2".into(),
        provider: "Sentence Transformers".into(),
        version: "L6-v2".into(),
        description: "Compact sentence embedding model for semantic similarity.".into(),
        examples: words(&["compare two texts", "match a query to a short description"]),
        avoid_when: "Avoid when generation, extraction, or tool calling is required.".into(),
        when_to_use: "Use for lightweight semantic similarity and embedding-based matching.".into(),
        size: ModelSize::Parameters {
            value: 22_700_000.0,
            unit: "parameters".into(),
        },
        capabilities: words(&["embeddings", "semantic-similarity"]),
        input: "Two text strings".into(),
        output: "Similarity score".into(),
        runtime: "ModelRuntime adapter".into(),
        model_uri: Some("sentence-transformers/all-MiniLM-L6-v2".into()),
        license: Some("Apache-2.0".into()),
    }
}

/// Exact eight model metadata records from GutTS pinned definition files.
pub fn gutts_model_definitions() -> Vec<ModelMetadata> {
    vec![
        smollm2(),
        needle3(),
        whistle(),
        ettin(17, "Very small relevance model for ranking candidate text."),
        ettin(32, "Small relevance model for ranking candidate text."),
        ettin(
            68,
            "Compact relevance model for higher-quality candidate ranking.",
        ),
        ettin(150, "Larger compact relevance model for candidate ranking."),
        minilm(),
    ]
}
pub fn gutts_model_by_id(id: &str) -> Option<ModelMetadata> {
    gutts_model_definitions()
        .into_iter()
        .find(|entry| entry.id == id)
}

/// Native model-runtime adapter contract. Implement with an actual model host.
/// This synchronous trait isolates loading from the portable residency logic;
/// asynchronous engines can offload it without a fabricated inference result.
pub trait ModelRuntime<M> {
    type Error;
    fn load(&self, metadata: &ModelMetadata) -> Result<M, Self::Error>;
    fn unload(&self, model: Arc<M>) -> Result<(), Self::Error>;
}

struct ResidencyState<M> {
    model: Option<Arc<M>>,
    loading: bool,
    unloading: bool,
    users: usize,
}

/// Thread-safe single-flight loading and last-user unloading for hosted Rust.
/// Condvar prevents a second load while either load or unload is running.
pub struct ModelResidency<M> {
    state: Mutex<ResidencyState<M>>,
    ready: Condvar,
}

impl<M> Default for ModelResidency<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M> ModelResidency<M> {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(ResidencyState {
                model: None,
                loading: false,
                unloading: false,
                users: 0,
            }),
            ready: Condvar::new(),
        }
    }
    fn lock(&self) -> MutexGuard<'_, ResidencyState<M>> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
    fn wait<'a>(
        &self,
        guard: MutexGuard<'a, ResidencyState<M>>,
    ) -> MutexGuard<'a, ResidencyState<M>> {
        self.ready
            .wait(guard)
            .unwrap_or_else(PoisonError::into_inner)
    }
    pub fn active_users(&self) -> usize {
        self.lock().users
    }
    pub fn is_loaded(&self) -> bool {
        self.lock().model.is_some()
    }

    /// Execute one operation, keeping a shared model resident during overlapping
    /// calls. The last caller always attempts unload after its operation.
    /// Operation errors take precedence over unload errors, as in a finally path.
    pub fn run<R, E, L, U, O>(&self, load: L, unload: U, operation: O) -> Result<R, E>
    where
        L: Fn() -> Result<M, E>,
        U: Fn(Arc<M>) -> Result<(), E>,
        O: FnOnce(&M) -> Result<R, E>,
    {
        let model = {
            let mut state = self.lock();
            while state.loading || state.unloading {
                state = self.wait(state);
            }
            if let Some(model) = state.model.as_ref().cloned() {
                state.users += 1;
                model
            } else {
                state.loading = true;
                drop(state);
                let loaded = load();
                let mut state = self.lock();
                state.loading = false;
                match loaded {
                    Ok(value) => {
                        let model = Arc::new(value);
                        state.model = Some(Arc::clone(&model));
                        state.users += 1;
                        self.ready.notify_all();
                        model
                    }
                    Err(error) => {
                        self.ready.notify_all();
                        return Err(error);
                    }
                }
            }
        };

        let operation_result = operation(&model);
        let mut state = self.lock();
        state.users -= 1;
        let last = state.users == 0;
        let unload_model = if last {
            state.unloading = true;
            state.model.take()
        } else {
            None
        };
        drop(state);
        drop(model);

        if let Some(model) = unload_model {
            let unload_result = unload(model);
            let mut state = self.lock();
            state.unloading = false;
            self.ready.notify_all();
            drop(state);
            match (operation_result, unload_result) {
                (Err(operation_error), _) => Err(operation_error),
                (Ok(_), Err(unload_error)) => Err(unload_error),
                (Ok(value), Ok(())) => Ok(value),
            }
        } else {
            operation_result
        }
    }
}

/// A model library holds metadata and a resident model. Host adapters provide
/// the actual runtime; the library never manufactures inference responses.
pub struct ModelLibrary<M> {
    pub metadata: ModelMetadata,
    pub residency: ModelResidency<M>,
}

impl<M> ModelLibrary<M> {
    pub fn new(metadata: ModelMetadata) -> Self {
        Self {
            metadata,
            residency: ModelResidency::new(),
        }
    }
    pub fn with_model<R, T, F>(&self, runtime: &R, operation: F) -> Result<T, R::Error>
    where
        R: ModelRuntime<M>,
        F: FnOnce(&M) -> Result<T, R::Error>,
    {
        self.residency.run(
            || runtime.load(&self.metadata),
            |model| runtime.unload(model),
            operation,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Barrier;
    #[test]
    fn registry_matches_eight_pinned_definitions_and_wire() {
        let definitions = gutts_model_definitions();
        assert_eq!(definitions.len(), 8);
        assert_eq!(definitions[0].id, "model.smollm2-135m-instruct");
        assert_eq!(
            definitions[3].model_uri.as_deref(),
            Some("cross-encoder/ettin-reranker-17m-v1")
        );
        assert_eq!(
            gutts_model_by_id("model.cactus-needle3").unwrap().version,
            "3"
        );
        assert!(gutts_model_by_id("unknown").is_none());
        for model in definitions {
            let wire = serde_json::to_value(&model).unwrap();
            assert_eq!(
                serde_json::from_value::<ModelMetadata>(wire).unwrap(),
                model
            );
        }
    }
    #[test]
    fn parallel_users_share_one_load_and_one_unload() {
        let residency: Arc<ModelResidency<usize>> = Arc::new(ModelResidency::new());
        let entered = Arc::new(Barrier::new(2));
        let loads = Arc::new(AtomicUsize::new(0));
        let unloads = Arc::new(AtomicUsize::new(0));
        std::thread::scope(|scope| {
            for _ in 0..2 {
                let residency = Arc::clone(&residency);
                let entered = Arc::clone(&entered);
                let loads = Arc::clone(&loads);
                let unloads = Arc::clone(&unloads);
                scope.spawn(move || {
                    let outcome = residency.run(
                        || {
                            loads.fetch_add(1, Ordering::SeqCst);
                            Ok::<usize, &'static str>(42)
                        },
                        |_model| {
                            unloads.fetch_add(1, Ordering::SeqCst);
                            Ok(())
                        },
                        |model| {
                            assert_eq!(*model, 42);
                            entered.wait();
                            Ok(*model)
                        },
                    );
                    assert_eq!(outcome, Ok(42));
                });
            }
        });
        assert_eq!(loads.load(Ordering::SeqCst), 1);
        assert_eq!(unloads.load(Ordering::SeqCst), 1);
        assert_eq!(residency.active_users(), 0);
        assert!(!residency.is_loaded());
    }
    #[test]
    fn failure_is_not_cached_as_success() {
        let residency: ModelResidency<u64> = ModelResidency::new();
        let failed = residency.run(
            || Err::<u64, &str>("load-failed"),
            |_model| Ok(()),
            |_model| Ok(()),
        );
        assert_eq!(failed, Err("load-failed"));
        assert!(!residency.is_loaded());
        assert_eq!(
            residency.run(|| Ok::<u64, &str>(7), |_model| Ok(()), |model| Ok(*model)),
            Ok(7)
        );
    }
}
