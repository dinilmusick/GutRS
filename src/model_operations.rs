//! Explicit native adapters for the five public GutTS model families.
//!
//! These wrappers implement load/use/unload semantics and pass inference
//! directly to the host's concrete engine. No model answer is synthesized.
//! Consumers must supply real Cactus/Transformers/ONNX backends; this module
//! does not falsely claim a bundled inference engine.
use crate::api::{gutts_api_by_id, ApiDefinition, ApiError};
use crate::models::{gutts_model_by_id, ModelLibrary, ModelMetadata, ModelRuntime};
use serde_json::Value;
use std::sync::{atomic::{AtomicBool,Ordering},Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelRegistryError;
impl std::fmt::Display for ModelRegistryError {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
        write!(f,"Pinned GutTS model metadata is missing")
    }
}
impl std::error::Error for ModelRegistryError {}

fn metadata(id:&str)->Result<ModelMetadata,ModelRegistryError>{
    gutts_model_by_id(id).ok_or(ModelRegistryError)
}
fn api(id:&str)->Result<&'static ApiDefinition,ApiError>{
    gutts_api_by_id(id)?.ok_or_else(||ApiError::UnknownApi(id.into()))
}

/// A host-side cooperative cancellation flag corresponding to a model loader's
/// AbortSignal. Async cancellation and enforcement remain the host's duty.
#[derive(Clone, Default, Debug)]
pub struct ModelLoadContext {
    pub cancelled:Arc<AtomicBool>,
}
impl ModelLoadContext{
    pub fn cancel(&self){self.cancelled.store(true,Ordering::SeqCst);}
    pub fn is_cancelled(&self)->bool{self.cancelled.load(Ordering::SeqCst)}
}
/// This lease exposes the real model until explicit release. Release consumes
/// the lease; double-release is impossible at the Rust type boundary.
pub struct ModelLease<M,F>{
    pub model:M,
    release:F,
}
impl<M,F> ModelLease<M,F>{
    pub fn new(model:M,release:F)->Self{Self{model,release}}
    pub fn release<E>(self)->Result<(),E> where F:FnOnce(M)->Result<(),E>{
        (self.release)(self.model)
    }
}
pub type ModelLoader<M,E> = Box<dyn Fn() -> Result<M,E> + Send + Sync>;
pub type ModelUnloader<M,E> = Box<dyn Fn(Arc<M>) -> Result<(),E> + Send + Sync>;

pub trait Needle3Inference {
    type Error;
    fn complete(&self,prompt:&str,tools:Option<&Value>,schema:Option<&Value>)->Result<Value,Self::Error>;
    fn embed(&self,input:&str)->Result<Vec<f32>,Self::Error>;
}
pub trait SmolLM2Inference {
    type Error;
    fn generate(&self,prompt:&str,max_new_tokens:Option<u32>)->Result<String,Self::Error>;
}
pub trait WhistleInference {
    type Error;
    fn transcribe(&self,audio:&[u8],language:Option<&str>)->Result<String,Self::Error>;
}
pub trait EttinInference {
    type Error;
    fn rerank(&self,query:&str,candidates:&[String])->Result<Vec<f64>,Self::Error>;
}
pub trait MiniLMInference {
    type Error;
    fn similarity(&self,left:&str,right:&str)->Result<f64,Self::Error>;
}

pub struct Needle3<M,R:ModelRuntime<M>>{
    pub library:ModelLibrary<M>,
    runtime:R,
}
impl<M,R> Needle3<M,R> where M:Needle3Inference<Error=R::Error>,R:ModelRuntime<M>{
    pub fn new(runtime:R)->Result<Self,ModelRegistryError>{
        Ok(Self{library:ModelLibrary::new(metadata("model.cactus-needle3")?),runtime})
    }
    pub fn complete(&self,prompt:&str,tools:Option<&Value>,schema:Option<&Value>)
        ->Result<Value,R::Error>{
        self.library.with_model(&self.runtime,|model|model.complete(prompt,tools,schema))
    }
    pub fn embed(&self,input:&str)->Result<Vec<f32>,R::Error>{
        self.library.with_model(&self.runtime,|model|model.embed(input))
    }
}
pub struct SmolLM2<M,R:ModelRuntime<M>>{
    pub library:ModelLibrary<M>,
    runtime:R,
}
impl<M,R> SmolLM2<M,R> where M:SmolLM2Inference<Error=R::Error>,R:ModelRuntime<M>{
    pub fn new(runtime:R)->Result<Self,ModelRegistryError>{
        Ok(Self{library:ModelLibrary::new(metadata("model.smollm2-135m-instruct")?),runtime})
    }
    pub fn generate(&self,prompt:&str,max_new_tokens:Option<u32>)->Result<String,R::Error>{
        self.library.with_model(&self.runtime,|model|model.generate(prompt,max_new_tokens))
    }
}
pub struct Whistle<M,R:ModelRuntime<M>>{
    pub library:ModelLibrary<M>,
    runtime:R,
}
impl<M,R> Whistle<M,R> where M:WhistleInference<Error=R::Error>,R:ModelRuntime<M>{
    pub fn new(runtime:R)->Result<Self,ModelRegistryError>{
        Ok(Self{library:ModelLibrary::new(metadata("model.cactus-whistle")?),runtime})
    }
    pub fn transcribe(&self,audio:&[u8],language:Option<&str>)->Result<String,R::Error>{
        self.library.with_model(&self.runtime,|model|model.transcribe(audio,language))
    }
}
pub struct MiniLM<M,R:ModelRuntime<M>>{
    pub library:ModelLibrary<M>,
    runtime:R,
}
impl<M,R> MiniLM<M,R> where M:MiniLMInference<Error=R::Error>,R:ModelRuntime<M>{
    pub fn new(runtime:R)->Result<Self,ModelRegistryError>{
        Ok(Self{library:ModelLibrary::new(metadata("model.minilm-l6-v2")?),runtime})
    }
    pub fn similarity(&self,left:&str,right:&str)->Result<f64,R::Error>{
        self.library.with_model(&self.runtime,|model|model.similarity(left,right))
    }
}
pub struct EttinReranker<M,R:ModelRuntime<M>>{
    pub library:ModelLibrary<M>,
    runtime:R,
}
impl<M,R> EttinReranker<M,R> where M:EttinInference<Error=R::Error>,R:ModelRuntime<M>{
    pub fn new(runtime:R)->Result<Self,ModelRegistryError>{
        Self::with_size(17,runtime)
    }
    pub fn with_size(size:u16,runtime:R)->Result<Self,ModelRegistryError>{
        if !matches!(size,17|32|68|150){return Err(ModelRegistryError);}
        let id=format!("model.ettin-reranker-{size}m");
        Ok(Self{library:ModelLibrary::new(metadata(&id)?),runtime})
    }
    pub fn rerank(&self,query:&str,candidates:&[String])->Result<Vec<f64>,R::Error>{
        self.library.with_model(&self.runtime,|model|model.rerank(query,candidates))
    }
}
macro_rules! ettin_wrapper {
    ($name:ident,$size:expr) => {
        pub struct $name<M,R:ModelRuntime<M>>(pub EttinReranker<M,R>);
        impl<M,R> $name<M,R>
        where M:EttinInference<Error=R::Error>,R:ModelRuntime<M>{
            pub fn new(runtime:R)->Result<Self,ModelRegistryError>{
                Ok(Self(EttinReranker::with_size($size,runtime)?))
            }
            pub fn rerank(&self,query:&str,candidates:&[String])->Result<Vec<f64>,R::Error>{
                self.0.rerank(query,candidates)
            }
        }
    };
}
ettin_wrapper!(EttinReranker17M,17);
ettin_wrapper!(EttinReranker32M,32);
ettin_wrapper!(EttinReranker68M,68);
ettin_wrapper!(EttinReranker150M,150);

pub fn needle3_metadata()->Result<ModelMetadata,ModelRegistryError>{metadata("model.cactus-needle3")}
pub fn smollm2_metadata()->Result<ModelMetadata,ModelRegistryError>{metadata("model.smollm2-135m-instruct")}
pub fn whistle_metadata()->Result<ModelMetadata,ModelRegistryError>{metadata("model.cactus-whistle")}
pub fn minilm_l6_metadata()->Result<ModelMetadata,ModelRegistryError>{metadata("model.minilm-l6-v2")}
pub fn ettin_metadata(size:u16)->Result<ModelMetadata,ModelRegistryError>{
    if !matches!(size,17|32|68|150){return Err(ModelRegistryError);}
    metadata(&format!("model.ettin-reranker-{size}m"))
}
pub fn needle3_complete_api()->Result<&'static ApiDefinition,ApiError>{api("model.cactus-needle3.complete")}
pub fn smollm2_generate_api()->Result<&'static ApiDefinition,ApiError>{api("model.smollm2-135m-instruct.generate")}
pub fn whistle_transcribe_api()->Result<&'static ApiDefinition,ApiError>{api("model.cactus-whistle.transcribe")}
pub fn ettin_rerank_api()->Result<&'static ApiDefinition,ApiError>{api("model.ettin-reranker.rerank")}
pub fn minilm_similarity_api()->Result<&'static ApiDefinition,ApiError>{api("model.minilm-l6-v2.similarity")}

#[cfg(test)]
mod tests{
    use super::*;
    use std::sync::atomic::AtomicUsize;
    struct FunctionalModel;
    impl Needle3Inference for FunctionalModel {
        type Error=&'static str;
        fn complete(&self,prompt:&str,_:Option<&Value>,_:Option<&Value>)->Result<Value,Self::Error>{
            Ok(serde_json::json!({"echo":prompt}))
        }
        fn embed(&self,_:&str)->Result<Vec<f32>,Self::Error>{Ok(vec![1.0,2.0])}
    }
    impl MiniLMInference for FunctionalModel{
        type Error=&'static str;
        fn similarity(&self,_:&str,_:&str)->Result<f64,Self::Error>{Ok(0.625)}
    }
    impl SmolLM2Inference for FunctionalModel{
        type Error=&'static str;
        fn generate(&self,prompt:&str,_:Option<u32>)->Result<String,Self::Error>{Ok(prompt.to_owned())}
    }
    impl WhistleInference for FunctionalModel{
        type Error=&'static str;
        fn transcribe(&self,audio:&[u8],_:Option<&str>)->Result<String,Self::Error>{Ok(format!("{} bytes",audio.len()))}
    }
    impl EttinInference for FunctionalModel{
        type Error=&'static str;
        fn rerank(&self,_:&str,candidates:&[String])->Result<Vec<f64>,Self::Error>{Ok(vec![0.5;candidates.len()])}
    }
    struct Host {
        loads:Arc<AtomicUsize>,
        unloads:Arc<AtomicUsize>,
    }
    impl ModelRuntime<FunctionalModel> for Host{
        type Error=&'static str;
        fn load(&self,_:&ModelMetadata)->Result<FunctionalModel,Self::Error>{
            self.loads.fetch_add(1,Ordering::SeqCst);
            Ok(FunctionalModel)
        }
        fn unload(&self,_:Arc<FunctionalModel>)->Result<(),Self::Error>{
            self.unloads.fetch_add(1,Ordering::SeqCst);
            Ok(())
        }
    }
    fn host()->Host{Host{loads:Arc::new(AtomicUsize::new(0)),unloads:Arc::new(AtomicUsize::new(0))}}
    #[test]
    fn models_delegate_to_host_and_release_after_operation(){
        let needle=Needle3::new(host()).unwrap();
        assert_eq!(needle.complete("hello",None,None).unwrap(),serde_json::json!({"echo":"hello"}));
        assert_eq!(needle.embed("x").unwrap(),vec![1.0,2.0]);
        assert_eq!(needle.library.residency.active_users(),0);
        let lm=SmolLM2::new(host()).unwrap();
        assert_eq!(lm.generate("native",Some(3)).unwrap(),"native");
        let whistle=Whistle::new(host()).unwrap();
        assert_eq!(whistle.transcribe(&[1,2,3],Some("en")).unwrap(),"3 bytes");
        let mini=MiniLM::new(host()).unwrap();
        assert_eq!(mini.similarity("a","b").unwrap(),0.625);
        let ettin=EttinReranker32M::new(host()).unwrap();
        assert_eq!(ettin.rerank("query",&["a".into(),"b".into()]).unwrap(),vec![0.5,0.5]);
        assert_eq!(ettin.0.library.metadata.id,"model.ettin-reranker-32m");
    }
    #[test]
    fn registries_never_fabricate_missing_models_or_apis(){
        assert!(EttinReranker::<FunctionalModel,Host>::with_size(42,host()).is_err());
        assert_eq!(needle3_complete_api().unwrap().name,"Needle3.complete");
        assert_eq!(ettin_rerank_api().unwrap().name,"EttinReranker.rerank");
        assert_eq!(minilm_l6_metadata().unwrap().version,"L6-v2");
        let context=ModelLoadContext::default();
        assert!(!context.is_cancelled());
        context.cancel();
        assert!(context.is_cancelled());
        let lease=ModelLease::new(12,|value|Ok::<(),&'static str>(if value==12{()}else{return Err("bad")}));
        assert!(lease.release().is_ok());
    }
}
