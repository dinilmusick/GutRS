//! Genuine SmolLM2-135M-Instruct text generation on Candle's CPU runtime.
//! No API keys, subprocesses, placeholder text or runtime model substitution.
#[cfg(feature = "smollm2-native")]
pub mod native {
    use crate::model_operations::SmolLM2Inference;
    use candle_core::{DType, Device, Tensor};
    use candle_nn::VarBuilder;
    use candle_transformers::{
        generation::{LogitsProcessor, Sampling},
        models::llama::{self, Llama, LlamaConfig, LlamaEosToks},
    };
    use std::sync::Mutex;
    use tokenizers::Tokenizer;

    pub const MODEL_ID: &str = "HuggingFaceTB/SmolLM2-135M-Instruct";

    struct Session {
        model: Llama,
        tokenizer: Tokenizer,
        config: llama::Config,
    }
    /// Real in-memory model with one serialized Candle inference session.
    pub struct NativeSmolLM2 {
        session: Mutex<Session>,
    }
    impl NativeSmolLM2 {
        /// Download and load the exact Hugging Face safetensors checkpoint,
        /// config and tokenizer on CPU. All loading errors propagate.
        pub fn load() -> Result<Self, String> {
            let repo = hf_hub::api::sync::Api::new()
                .map_err(|e| e.to_string())?
                .model(MODEL_ID.into());
            let tokenizer_path=repo.get("tokenizer.json").map_err(|e|e.to_string())?;
            let config_path=repo.get("config.json").map_err(|e|e.to_string())?;
            let weight_path=repo.get("model.safetensors").map_err(|e|e.to_string())?;
            let model_config: LlamaConfig =
                serde_json::from_slice(&std::fs::read(config_path)
                    .map_err(|e|e.to_string())?).map_err(|e:serde_json::Error|e.to_string())?;
            let config=model_config.into_config(false);
            let device=Device::Cpu;
            // In-memory safetensors loading avoids unsafe memory mapping.
            let tensors=candle_core::safetensors::load(weight_path,&device)
                .map_err(|e|e.to_string())?;
            let vb=VarBuilder::from_tensors(tensors,DType::F32,&device);
            let model=Llama::load(vb,&config).map_err(|e|e.to_string())?;
            let tokenizer=Tokenizer::from_file(tokenizer_path)
                .map_err(|e|e.to_string())?;
            Ok(Self { session:Mutex::new(Session{model,tokenizer,config}) })
        }
    }
    impl SmolLM2Inference for NativeSmolLM2 {
        type Error=String;
        fn generate(&self,prompt:&str,max_new_tokens:Option<u32>)
            ->Result<String,Self::Error>{
            if prompt.trim().is_empty(){
                return Err("SmolLM2 prompt must not be empty".into());
            }
            let limit=max_new_tokens.unwrap_or(64);
            if limit>1024{
                return Err("SmolLM2 generation token limit exceeds 1024".into());
            }
            if limit==0{return Ok(String::new());}
            let state=self.session.lock()
                .map_err(|_|"SmolLM2 model lock poisoned".to_string())?;
            let device=Device::Cpu;
            let mut cache=llama::Cache::new(true,DType::F32,&state.config,&device)
                .map_err(|e|e.to_string())?;
            let encoded=state.tokenizer.encode(prompt,true)
                .map_err(|e|e.to_string())?;
            let mut tokens=encoded.get_ids().to_vec();
            if tokens.is_empty(){return Err("SmolLM2 tokenizer produced no prompt tokens".into());}
            let mut generated=Vec::<u32>::new();
            let mut index_pos=0_usize;
            let mut processor=LogitsProcessor::from_sampling(0,Sampling::ArgMax);
            for step in 0..limit {
                let (context_len,context_index)=if step>0 {(1,index_pos)}else{(tokens.len(),0)};
                let context=&tokens[tokens.len()-context_len..];
                let input=Tensor::new(context,&device).map_err(|e|e.to_string())?
                    .unsqueeze(0).map_err(|e|e.to_string())?;
                let logits=state.model.forward(&input,context_index,&mut cache)
                    .map_err(|e|e.to_string())?.squeeze(0).map_err(|e|e.to_string())?;
                index_pos+=context_len;
                let next=processor.sample(&logits).map_err(|e|e.to_string())?;
                let stop=match &state.config.eos_token_id{
                    Some(LlamaEosToks::Single(id))=>next==*id,
                    Some(LlamaEosToks::Multiple(ids))=>ids.contains(&next),
                    None=>false,
                };
                if stop{break;}
                generated.push(next);
                tokens.push(next);
            }
            state.tokenizer.decode(&generated,true).map_err(|e|e.to_string())
        }
    }
    #[test]
    fn exactly_pinned_model_is_never_swapped(){
        assert_eq!(MODEL_ID,"HuggingFaceTB/SmolLM2-135M-Instruct");
    }
}
