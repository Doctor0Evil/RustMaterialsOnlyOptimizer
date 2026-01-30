use crate::kernels::KernelOutputs;
use crate::region::RegionConfig;

/// Admissibility result for a recipe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admissibility {
    Admissible,
    Rejected,
}

/// Apply Phoenix hard gates on a recipe's kernel outputs.
///
/// Conditions:
/// - t90 <= 180 days.
/// - r_tox <= 0.10, r_micro <= 0.05, r_worm <= 0.10, r_bee <= 0.10.
/// - R_residual <= R_max (≈0.13 in Phoenix default).[web:0]
pub fn admissible(region: &RegionConfig, outputs: &KernelOutputs) -> Admissibility {
    if outputs.t90_days > region.target_t90_max_days {
        return Admissibility::Rejected;
    }

    if outputs.r_tox > 0.10
        || outputs.r_micro > 0.05
        || outputs.r_worm > 0.10
        || outputs.r_bee > 0.10
    {
        return Admissibility::Rejected;
    }

    if outputs.r_residual > region.r_max {
        return Admissibility::Rejected;
    }

    Admissibility::Admissible
}
