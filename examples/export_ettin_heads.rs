use gutrs::ettin_head::{compute_ettin_head_scores, EttinHeadWeights};
use serde_json::{json, Value};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data: Value =
        serde_json::from_str(include_str!("../testdata/upstream-golden/ettin-head.json"))?;
    let mut output = Vec::new();
    for fixture in data["cases"].as_array().ok_or("cases missing")? {
        let head = &fixture["head"];
        let get = |key: &str| -> Result<Vec<f32>, Box<dyn std::error::Error>> {
            Ok(head[key]
                .as_array()
                .ok_or("invalid weight array")?
                .iter()
                .map(|x| x.as_f64().unwrap_or(0.0) as f32)
                .collect())
        };
        let weights = EttinHeadWeights {
            w1: get("w1")?,
            ln_weight: get("lnWeight")?,
            ln_bias: get("lnBias")?,
            w2: get("w2")?,
            b2: get("b2")?,
        };
        let dims_vec = fixture["dims"].as_array().ok_or("dims missing")?;
        let dims = [
            dims_vec[0].as_u64().ok_or("dim0")? as usize,
            dims_vec[1].as_u64().ok_or("dim1")? as usize,
            dims_vec[2].as_u64().ok_or("dim2")? as usize,
        ];
        let hidden = fixture["data"]
            .as_array()
            .ok_or("data missing")?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect::<Vec<_>>();
        output.push(json!(compute_ettin_head_scores(dims, &hidden, &weights)?));
    }
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}
