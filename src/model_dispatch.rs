//! All pinned model API IDs dispatch through a real caller-provided host.
//! No model data, text, scores or transcripts are fabricated by the crate.
use crate::api::{execute_gutts_api,gutts_api_by_id,ApiError};
use serde_json::{json,Value};
use base64::Engine;

pub trait GuttsModelDispatch {
    type Error:std::fmt::Display;
    fn generate(&self,prompt:&str,max_new_tokens:Option<u32>)->Result<String,Self::Error>;
    fn complete(&self,prompt:&str,tools:Option<&Value>,schema:Option<&Value>)->Result<Value,Self::Error>;
    fn transcribe(&self,audio:&[u8],language:Option<&str>)->Result<String,Self::Error>;
    fn rerank(&self,query:&str,candidates:&[String])->Result<Vec<f64>,Self::Error>;
    fn similarity(&self,left:&str,right:&str)->Result<f64,Self::Error>;
}
fn str_field<'a>(input:&'a Value,key:&str)->Result<&'a str,ApiError>{
    input.get(key).and_then(Value::as_str).ok_or_else(||ApiError::InvalidArguments(
        format!("missing string argument '{key}'")))
}
fn optional_field<'a>(input:&'a Value,key:&str,kind:&str)->Result<Option<&'a Value>,ApiError>{
    match input.get(key) {
        None=>Ok(None),
        Some(v) if (kind=="array"&&v.is_array())||(kind=="object"&&v.is_object())=>Ok(Some(v)),
        _=>Err(ApiError::InvalidArguments(format!("'{key}' must be {kind}"))),
    }
}
pub fn execute_gutts_api_with_models<M:GuttsModelDispatch>(
    api_id:&str,input:&Value,host:&M
)->Result<Value,ApiError>{
    gutts_api_by_id(api_id)?.ok_or_else(||ApiError::UnknownApi(api_id.into()))?;
    if !input.is_object(){return Err(ApiError::InvalidArguments("API input must be object".into()));}
    let failed=|e:M::Error|ApiError::Operation(e.to_string());
    match api_id {
        "model.smollm2-135m-instruct.generate"=>{
            let max=match input.get("maxNewTokens"){
                None=>None,
                Some(v)=>Some(v.as_u64().and_then(|v|u32::try_from(v).ok())
                    .ok_or_else(||ApiError::InvalidArguments("maxNewTokens must be nonnegative u32".into()))?),
            };
            Ok(json!(host.generate(str_field(input,"prompt")?,max).map_err(failed)?))
        }
        "model.cactus-needle3.complete"=>Ok(host.complete(
            str_field(input,"prompt")?,
            optional_field(input,"tools","array")?,
            optional_field(input,"schema","object")?,
        ).map_err(failed)?),
        "model.cactus-whistle.transcribe"=>{
            let audio=base64::engine::general_purpose::STANDARD.decode(str_field(input,"audio")?)
                .map_err(|e|ApiError::InvalidArguments(format!("audio must be RFC4648 base64: {e}")))?;
            let language=match input.get("language"){
                None=>None,
                Some(v)=>Some(v.as_str().ok_or_else(||ApiError::InvalidArguments("language must be string".into()))?),
            };
            Ok(json!(host.transcribe(&audio,language).map_err(failed)?))
        }
        "model.ettin-reranker.rerank"=>{
            let candidates=input.get("candidates").and_then(Value::as_array)
                .ok_or_else(||ApiError::InvalidArguments("candidates must be array".into()))?
                .iter().map(|v|v.as_str().map(str::to_owned)
                    .ok_or_else(||ApiError::InvalidArguments("candidate must be string".into())))
                .collect::<Result<Vec<_>,_>>()?;
            let scores=host.rerank(str_field(input,"query")?,&candidates).map_err(failed)?;
            if scores.len()!=candidates.len()||scores.iter().any(|v|!v.is_finite()){
                return Err(ApiError::Operation("invalid reranker scores".into()));
            }
            Ok(json!(scores))
        }
        "model.minilm-l6-v2.similarity"=>{
            let score=host.similarity(str_field(input,"left")?,str_field(input,"right")?)
                .map_err(failed)?;
            if !score.is_finite(){return Err(ApiError::Operation("nonfinite similarity".into()));}
            Ok(json!(score))
        }
        _=>execute_gutts_api(api_id,input),
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    struct Host;
    impl GuttsModelDispatch for Host{
        type Error=&'static str;
        fn generate(&self,prompt:&str,_:Option<u32>)->Result<String,Self::Error>{Ok(prompt.into())}
        fn complete(&self,prompt:&str,_:Option<&Value>,_:Option<&Value>)->Result<Value,Self::Error>{Ok(json!({"text":prompt}))}
        fn transcribe(&self,audio:&[u8],_:Option<&str>)->Result<String,Self::Error>{Ok(audio.len().to_string())}
        fn rerank(&self,_:&str,candidates:&[String])->Result<Vec<f64>,Self::Error>{Ok(vec![0.5;candidates.len()])}
        fn similarity(&self,_:&str,_:&str)->Result<f64,Self::Error>{Ok(0.75)}
    }
    #[test]
    fn all_five_models_use_supplied_host(){
        let h=Host;
        assert_eq!(execute_gutts_api_with_models("model.smollm2-135m-instruct.generate",&json!({"prompt":"hello"}),&h).unwrap(),json!("hello"));
        assert_eq!(execute_gutts_api_with_models("model.cactus-needle3.complete",&json!({"prompt":"x"}),&h).unwrap(),json!({"text":"x"}));
        assert_eq!(execute_gutts_api_with_models("model.cactus-whistle.transcribe",&json!({"audio":"AQID"}),&h).unwrap(),json!("3"));
        assert_eq!(execute_gutts_api_with_models("model.ettin-reranker.rerank",&json!({"query":"a","candidates":["a","b"]}),&h).unwrap(),json!([0.5,0.5]));
        assert_eq!(execute_gutts_api_with_models("model.minilm-l6-v2.similarity",&json!({"left":"a","right":"b"}),&h).unwrap(),json!(0.75));
        assert_eq!(execute_gutts_api_with_models("gutts.arrays.last",&json!({"values":[1,2]}),&h).unwrap(),json!(2));
    }
    #[test]
    fn input_validation_rejects_invalid_audio_and_tools(){
        let h=Host;
        assert!(matches!(execute_gutts_api_with_models("model.cactus-whistle.transcribe",&json!({"audio":"bad%%%"}),&h),Err(ApiError::InvalidArguments(_))));
        assert!(matches!(execute_gutts_api_with_models("model.cactus-needle3.complete",&json!({"prompt":"x","tools":{}}),&h),Err(ApiError::InvalidArguments(_))));
        assert!(matches!(execute_gutts_api_with_models("model.minilm-l6-v2.similarity",&json!({"left":"x"}),&h),Err(ApiError::InvalidArguments(_))));
        assert!(matches!(execute_gutts_api_with_models("model.smollm2-135m-instruct.generate",&json!({"prompt":"x","maxNewTokens":-1}),&h),Err(ApiError::InvalidArguments(_))));
    }
}
