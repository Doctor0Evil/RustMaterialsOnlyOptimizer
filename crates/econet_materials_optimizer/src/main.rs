mod config;
mod sampler;
mod service;
mod writer;

use crate::config::OptimizerConfig;
use crate::service::run_once;
use anyhow::Result;
use clap::Parser;

/// Materials-only optimizer service for Phoenix trays.
///
/// Sweeps recipe space, calls econet_tray_kernels, and writes only
/// admissible recipes to BioPackPhoenixTrays2026v1.csv.[web:0]
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    /// Optional path to a JSON config file.
    #[arg(short, long)]
    config: Option<String>,

    /// Random seed for the sampler.
    #[arg(long, default_value_t = 42)]
    seed: u64,
}

fn load_config(path: Option<String>) -> Result<OptimizerConfig> {
    if let Some(p) = path {
        let text = std::fs::read_to_string(p)?;
        let cfg: OptimizerConfig = serde_json::from_str(&text)?;
        Ok(cfg)
    } else {
        Ok(OptimizerConfig::default_for_phoenix())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = load_config(cli.config)?;
    run_once(&config, cli.seed)?;
    Ok(())
}
