//! Exact portable score head for pinned GutTS Ettin cross-encoder models.
//!
//! The transformer backbone must provide its **real** last hidden state.
//! This module computes the exact Dense -> GELU -> LayerNorm -> Dense -> sigmoid
//! math of src/models/runtimes/ettin-head.ts, including f32 intermediate rounding.
use safetensors::{Dtype, SafeTensors};
use std::{fmt, path::Path};

#[derive(Clone, Debug, PartialEq)]
pub struct EttinHeadWeights {
    pub w1: Vec<f32>,
    pub ln_weight: Vec<f32>,
    pub ln_bias: Vec<f32>,
    pub w2: Vec<f32>,
    pub b2: Vec<f32>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EttinHeadError(pub String);
impl fmt::Display for EttinHeadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for EttinHeadError {}

fn load_tensor(bytes: &[u8], name: &str) -> Result<Vec<f32>, EttinHeadError> {
    let tensors = SafeTensors::deserialize(bytes).map_err(|e| EttinHeadError(e.to_string()))?;
    let tensor = tensors
        .tensor(name)
        .map_err(|e| EttinHeadError(e.to_string()))?;
    if tensor.dtype() != Dtype::F32 || !tensor.data().len().is_multiple_of(4) {
        return Err(EttinHeadError(format!(
            "{name}: expected aligned float32 weights"
        )));
    }
    let (data, remainder) = tensor.data().as_chunks::<4>();
    if !remainder.is_empty() {
        return Err(EttinHeadError(format!("{name}: invalid F32 data length")));
    }
    let values = data
        .iter()
        .map(|chunk| f32::from_le_bytes(*chunk))
        .collect::<Vec<_>>();
    if values.iter().any(|v| !v.is_finite()) {
        return Err(EttinHeadError(format!("{name}: nonfinite model weights")));
    }
    Ok(values)
}
impl EttinHeadWeights {
    /// Load genuine `2_Dense/model.safetensors`,
    /// `3_LayerNorm/model.safetensors`, and `4_Dense/model.safetensors`
    /// from an authorized download of the corresponding exact Ettin model.
    pub fn from_safetensors(
        dense: &[u8],
        layer_norm: &[u8],
        output: &[u8],
    ) -> Result<Self, EttinHeadError> {
        Ok(Self {
            w1: load_tensor(dense, "linear.weight")?,
            ln_weight: load_tensor(layer_norm, "norm.weight")?,
            ln_bias: load_tensor(layer_norm, "norm.bias")?,
            w2: load_tensor(output, "linear.weight")?,
            b2: load_tensor(output, "linear.bias")?,
        })
    }
    pub fn from_directory(path: impl AsRef<Path>) -> Result<Self, EttinHeadError> {
        let path = path.as_ref();
        let read = |subpath: &str| {
            std::fs::read(path.join(subpath)).map_err(|e| EttinHeadError(e.to_string()))
        };
        Self::from_safetensors(
            &read("2_Dense/model.safetensors")?,
            &read("3_LayerNorm/model.safetensors")?,
            &read("4_Dense/model.safetensors")?,
        )
    }
    pub fn validate(&self, hidden_size: usize) -> Result<(), EttinHeadError> {
        if hidden_size == 0
            || self.w1.len()
                != hidden_size
                    .checked_mul(hidden_size)
                    .ok_or_else(|| EttinHeadError("hidden size multiplication overflow".into()))?
            || self.ln_weight.len() != hidden_size
            || self.ln_bias.len() != hidden_size
            || self.w2.len() != hidden_size
            || self.b2.len() != 1
        {
            return Err(EttinHeadError(
                "Ettin head tensors do not match hidden size".into(),
            ));
        }
        if self
            .w1
            .iter()
            .chain(&self.ln_weight)
            .chain(&self.ln_bias)
            .chain(&self.w2)
            .chain(&self.b2)
            .any(|v| !v.is_finite())
        {
            return Err(EttinHeadError("Ettin head has nonfinite values".into()));
        }
        Ok(())
    }
}
pub fn compute_ettin_head_scores(
    dims: [usize; 3],
    hidden: &[f32],
    head: &EttinHeadWeights,
) -> Result<Vec<f64>, EttinHeadError> {
    let [batch, seq, size] = dims;
    if seq == 0 || size == 0 || batch == 0 {
        return Err(EttinHeadError(
            "Ettin hidden tensor shape must be positive".into(),
        ));
    }
    let required = batch
        .checked_mul(seq)
        .and_then(|n| n.checked_mul(size))
        .ok_or_else(|| EttinHeadError("Ettin hidden tensor shape overflow".into()))?;
    if required != hidden.len() || hidden.iter().any(|v| !v.is_finite()) {
        return Err(EttinHeadError(
            "Ettin hidden tensor length/value invalid".into(),
        ));
    }
    head.validate(size)?;
    let mut scores = Vec::with_capacity(batch);
    let c = (2.0_f64 / std::f64::consts::PI).sqrt();
    for b in 0..batch {
        let cls = &hidden[b * seq * size..(b * seq + 1) * size];
        let mut y = vec![0.0_f32; size];
        for (i, slot) in y.iter_mut().enumerate() {
            let sum = (0..size)
                .map(|j| f64::from(head.w1[i * size + j]) * f64::from(cls[j]))
                .sum::<f64>();
            *slot = (0.5 * sum * (1.0 + (c * (sum + 0.044715 * sum * sum * sum)).tanh())) as f32;
        }
        let mean = y.iter().map(|v| f64::from(*v)).sum::<f64>() / (size as f64);
        let variance = y
            .iter()
            .map(|v| {
                let diff = f64::from(*v) - mean;
                diff * diff
            })
            .sum::<f64>()
            / (size as f64);
        let inv_std = 1.0 / (variance + 1e-5).sqrt();
        for (i, slot) in y.iter_mut().enumerate() {
            *slot = (((f64::from(*slot) - mean) * inv_std) * f64::from(head.ln_weight[i])
                + f64::from(head.ln_bias[i])) as f32;
        }
        let logit = head
            .w2
            .iter()
            .zip(&y)
            .fold(f64::from(head.b2[0]), |sum, (w, v)| {
                sum + f64::from(*w) * f64::from(*v)
            });
        let score = 1.0 / (1.0 + (-logit).exp());
        if !score.is_finite() {
            return Err(EttinHeadError("nonfinite Ettin score".into()));
        }
        scores.push(score);
    }
    Ok(scores)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn weights() -> EttinHeadWeights {
        EttinHeadWeights {
            w1: vec![1.0, 0.2, -0.1, 0.9],
            ln_weight: vec![1.0, 1.0],
            ln_bias: vec![0.0, 0.0],
            w2: vec![0.8, -0.7],
            b2: vec![0.2],
        }
    }
    #[test]
    fn score_is_finite_and_batch_order_is_preserved() {
        let scores = compute_ettin_head_scores(
            [2, 2, 2],
            &[0.5, 0.3, 0.0, 0.0, -0.5, 0.7, 0.0, 0.0],
            &weights(),
        )
        .unwrap();
        assert_eq!(scores.len(), 2);
        assert!(scores.iter().all(|score| (0.0..=1.0).contains(score)));
        assert_ne!(scores[0], scores[1]);
    }
    #[test]
    fn no_fake_scores_for_malformed_weights_or_shape() {
        assert!(compute_ettin_head_scores([1, 1, 3], &[1.0, 1.0, 1.0], &weights()).is_err());
        assert!(compute_ettin_head_scores([1, 0, 2], &[], &weights()).is_err());
        assert!(compute_ettin_head_scores([1, 1, 2], &[1.0, f32::NAN], &weights()).is_err());
        let mut w = weights();
        w.w2.clear();
        assert!(compute_ettin_head_scores([1, 1, 2], &[1.0, 0.0], &w).is_err());
    }
}
