use crate::config::OptimizerConfig;
use crate::sampler::RecipeSampler;
use crate::writer::{append_row, ShardRow};
use anyhow::Result;
use econet_tray_kernels::admissibility::{admissible, Admissibility};
use econet_tray_kernels::kernels::eval_recipe;
use econet_tray_kernels::RegionConfig;

/// Run a single sweep over the recipe space and write admissible rows.
pub fn run_once(config: &OptimizerConfig, seed: u64) -> Result<()> {
    let region = RegionConfig::phoenix_default();
    let mut sampler = RecipeSampler::new(seed);

    for i in 0..config.samples_per_run {
        let mix = sampler.sample_recipe("phoenix_recipe", i);
        let outputs = match eval_recipe(&region, &mix) {
            Ok(o) => o,
            Err(_) => continue,
        };

        let status = admissible(&region, &outputs);
        if status != Admissibility::Admissible {
            continue;
        }

        // Eco-impact placeholders: in production, derived from CEIM/EcoNet bridge.[web:0]
        let ecoimpactscore = 1.0;
        let energy_kwh_per_cycle = 0.5;
        let waste_reduced_kg_per_cycle = 0.2;
        let antsafety_class = "A";
        let hexstamp = "0x5fd1c8e0b47aa2199c33ee4417aa99dd22cc7711";

        let row = ShardRow {
            nodeid: &config.nodeid,
            region: &config.region,
            materialmix: &mix,
            target_t90_days: region.target_t90_max_days,
            outputs: &outputs,
            ecoimpactscore,
            energy_kwh_per_cycle,
            waste_reduced_kg_per_cycle,
            antsafety_class,
            hexstamp,
            date: config.date,
        };

        append_row(config, &row)?;
    }

    Ok(())
}
