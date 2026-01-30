use serde::{Deserialize, Serialize};

/// Region-specific configuration for Phoenix-first validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionConfig {
    /// Human-readable region identifier (e.g., "Phoenix-AZ-ISO14851").
    pub region_id: String,
    /// Target t90 window in days (gold band ~90, max 180).
    pub target_t90_min_days: f64,
    pub target_t90_max_days: f64,
    /// Maximum allowed global residual R.
    pub r_max: f64,
    /// Risk weights for residual computation.
    pub w_tox: f64,
    pub w_micro: f64,
    pub w_worm: f64,
    pub w_bee: f64,
}

impl RegionConfig {
    /// Returns the canonical Phoenix configuration.
    pub fn phoenix_default() -> Self {
        Self {
            region_id: "Phoenix-AZ-ISO14851-2026".to_string(),
            target_t90_min_days: 0.0,
            target_t90_max_days: 180.0,
            r_max: 0.13,
            w_tox: 0.25,
            w_micro: 0.25,
            w_worm: 0.25,
            w_bee: 0.25,
        }
    }
}
