//! Real Cactus Needle3/Whistle native engine entrypoints.
//!
//! `cactus-native` links the *actual* Cactus Compute engine through the
//! safe cactus-rs v0.2.1 crate. The engine requires separately provided model
//! archives and supports only a pinned subset of host targets.
//! This is not a fabricated model or a JavaScript shim.
#[cfg(all(
    feature = "cactus-native",
    any(
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "aarch64"),
    )
))]
pub mod native {
    //! Actual CPU inference and transcription engines. The caller loads real
    //! weights from `needle3.cact` or `whistle.cact` using the builders.
    pub use cactus_rs::needle::{
        Needle as NodeCactusNeedle3Runtime, Tool, Weights as Needle3Weights,
    };
    pub use cactus_rs::whistle::{
        Language, TranscribeOptions, Weights as WhistleWeights, Whistle as NodeCactusWhistleRuntime,
    };

    use crate::model_operations::{Needle3Inference, WhistleInference};
    use serde_json::{json, Value};
    use std::sync::Mutex;

    #[derive(Debug)]
    pub struct CactusNativeError(pub String);
    impl std::fmt::Display for CactusNativeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl std::error::Error for CactusNativeError {}

    /// Holds a real process-global Cactus Needle3 engine. A conversation is
    /// preserved across calls; dynamic tool declarations must be installed
    /// by the caller's NeedleBuilder before constructing this adapter.
    pub struct CactusNeedle3 {
        engine: Mutex<NodeCactusNeedle3Runtime>,
    }
    impl CactusNeedle3 {
        pub fn new(engine: NodeCactusNeedle3Runtime) -> Self {
            Self {
                engine: Mutex::new(engine),
            }
        }
    }
    impl Needle3Inference for CactusNeedle3 {
        type Error = CactusNativeError;
        fn complete(
            &self,
            prompt: &str,
            tools: Option<&Value>,
            schema: Option<&Value>,
        ) -> Result<Value, Self::Error> {
            if tools.is_some() || schema.is_some() {
                return Err(CactusNativeError(
                    "dynamic tools/schema require preconfigured NeedleBuilder".into(),
                ));
            }
            let mut engine = self
                .engine
                .lock()
                .map_err(|_| CactusNativeError("Needle3 engine lock poisoned".into()))?;
            let answer = engine
                .complete(prompt)
                .map_err(|e| CactusNativeError(e.to_string()))?;
            // Completion implements Deserialize but intentionally not Serialize.
            // Map its published getters to a stable JSON response without
            // pretending the raw engine JSON envelope is available.
            let kind = if answer.kind().is_call() {
                "call"
            } else if answer.kind().is_respond() {
                "respond"
            } else {
                "unknown"
            };
            Ok(json!({
                "type":kind,
                "function_calls":answer.calls(),
                "reasoning":answer.reasoning(),
                "confidence":answer.confidence(),
            }))
        }
        fn embed(&self, input: &str) -> Result<Vec<f32>, Self::Error> {
            let mut engine = self
                .engine
                .lock()
                .map_err(|_| CactusNativeError("Needle3 engine lock poisoned".into()))?;
            engine
                .embed(input)
                .map_err(|e| CactusNativeError(e.to_string()))
        }
    }

    /// Actual Whistle accepts 16 kHz mono PCM f32, not compressed audio.
    /// For the native model inference interface the input is explicitly
    /// 16-bit little-endian signed PCM at 16 kHz.
    pub fn decode_pcm16_mono(bytes: &[u8]) -> Result<Vec<f32>, CactusNativeError> {
        if !bytes.len().is_multiple_of(2) {
            return Err(CactusNativeError(
                "PCM16 audio has an odd byte length".into(),
            ));
        }
        if bytes.len() > 960_000 {
            return Err(CactusNativeError(
                "Whistle clips cannot exceed 30 seconds".into(),
            ));
        }
        Ok(bytes
            .as_chunks::<2>()
            .0
            .iter()
            .map(|b| f32::from(i16::from_le_bytes([b[0], b[1]])) / 32768.0)
            .collect())
    }
    fn language_code(code: &str) -> Result<Language, CactusNativeError> {
        match code {
            "en" => Ok(Language::En),
            "de" => Ok(Language::De),
            "fr" => Ok(Language::Fr),
            "es" => Ok(Language::Es),
            "it" => Ok(Language::It),
            "nl" => Ok(Language::Nl),
            "pl" => Ok(Language::Pl),
            _ => Err(CactusNativeError("Whistle language is unsupported".into())),
        }
    }
    pub struct CactusWhistle {
        engine: Mutex<NodeCactusWhistleRuntime>,
    }
    impl CactusWhistle {
        pub fn new(engine: NodeCactusWhistleRuntime) -> Self {
            Self {
                engine: Mutex::new(engine),
            }
        }
    }
    impl WhistleInference for CactusWhistle {
        type Error = CactusNativeError;
        fn transcribe(&self, audio: &[u8], language: Option<&str>) -> Result<String, Self::Error> {
            let pcm = decode_pcm16_mono(audio)?;
            let options = if let Some(code) = language {
                TranscribeOptions::new().with_language(language_code(code)?)
            } else {
                TranscribeOptions::new()
            };
            let mut engine = self
                .engine
                .lock()
                .map_err(|_| CactusNativeError("Whistle engine lock poisoned".into()))?;
            engine
                .transcribe_with_options(&pcm, &options)
                .map(|transcript| transcript.text().to_owned())
                .map_err(|e| CactusNativeError(e.to_string()))
        }
    }
    #[test]
    fn pcm_audio_and_language_are_validated_before_engine() {
        assert_eq!(decode_pcm16_mono(&[0, 0, 255, 127]).unwrap().len(), 2);
        assert!(decode_pcm16_mono(&[0, 0, 0]).is_err());
        assert!(language_code("xx").is_err());
        assert_eq!(language_code("pl").unwrap(), Language::Pl);
    }

    #[test]
    fn actual_cactus_engine_types_are_public() {
        assert!(std::any::type_name::<NodeCactusNeedle3Runtime>().contains("Needle"));
        assert!(std::any::type_name::<NodeCactusWhistleRuntime>().contains("Whistle"));
        // No model archives are downloaded/loaded by this type-level test.
    }
}
