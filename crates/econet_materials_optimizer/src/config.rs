use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Optimizer configuration: sweep size, output shard path, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerConfig {
    /// Number of candidate recipes to sample per run.
    pub samples_per_run: usize,
    /// Path to the materials shard CSV.
    pub shard_path: String,
    /// Node identifier for Phoenix trays.
    pub nodeid: String,
    /// Region string, e.g., "Phoenix-AZ-ISO14851-2026".
    pub region: String,
    /// Date stamp for this run (YYYY-MM-DD).
    pub date: NaiveDate,
}

impl OptimizerConfig {
    pub fn default_for_phoenix() -> Self {
        Self {
            samples_per_run: 10_000,
            shard_path: "qpudatashards/particles/BioPackPhoenixTrays2026v1.csv".to_string(),
            nodeid: "BioPack-Phoenix-Trays-2026v1".to_string(),
            region: "Phoenix-AZ-ISO14851-2026".to_string(),
            date: chrono::Utc::now().date_naive(),
        }
    }
}
