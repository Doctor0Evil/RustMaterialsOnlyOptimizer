use crate::materials::MaterialMix;
use crate::region::RegionConfig;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Outputs of the Phoenix materials kernels for a single recipe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelOutputs {
    pub t90_days: f64,
    pub r_tox: f64,
    pub r_micro: f64,
    pub r_worm: f64,
    pub r_bee: f64,
    pub r_residual: f64,
}

#[derive(Debug, Error)]
pub enum EvaluationError {
    #[error("material mix has no components")]
    EmptyMix,
}

/// Evaluate a recipe under the Phoenix region configuration.
///
/// This function encodes the single source of truth for kernel behavior.
pub fn eval_recipe(region: &RegionConfig, mix: &MaterialMix) -> Result<KernelOutputs, EvaluationError> {
    if mix.components.is_empty() {
        return Err(EvaluationError::EmptyMix);
    }

    // Placeholder kernel logic:
    // - t90 scales with fraction of recalcitrant components
    // - risk metrics and residual follow simple monotone mappings
    // In production, this is replaced by calibrated Monod/Q10 models and LC–MS bands.[web:0]
    let norm_mix = mix.clone().normalized();

    let recalcitrant_index: f64 = norm_mix
        .components
        .iter()
        .map(|c| match c.name.as_str() {
            "bagasse" | "starch" => 0.5 * c.fraction,
            "PLA" => 1.2 * c.fraction,
            "coating-hard" => 1.5 * c.fraction,
            _ => 1.0 * c.fraction,
        })
        .sum();

    let t90_days = 90.0 * recalcitrant_index.clamp(0.5, 2.0);

    let r_tox = (0.02 * recalcitrant_index).clamp(0.0, 0.10);
    let r_micro = (0.01 * recalcitrant_index).clamp(0.0, 0.05);
    let r_worm = (0.03 * recalcitrant_index).clamp(0.0, 0.10);
    let r_bee = (0.03 * recalcitrant_index).clamp(0.0, 0.10);

    let r_residual = region.w_tox * r_tox
        + region.w_micro * r_micro
        + region.w_worm * r_worm
        + region.w_bee * r_bee;

    Ok(KernelOutputs {
        t90_days,
        r_tox,
        r_micro,
        r_worm,
        r_bee,
        r_residual,
    })
}
