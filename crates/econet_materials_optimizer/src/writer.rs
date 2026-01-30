use crate::config::OptimizerConfig;
use chrono::NaiveDate;
use econet_tray_kernels::kernels::KernelOutputs;
use econet_tray_kernels::materials::MaterialMix;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Single admissible row that will be written to the shard.
pub struct ShardRow<'a> {
    pub nodeid: &'a str,
    pub region: &'a str,
    pub materialmix: &'a MaterialMix,
    pub target_t90_days: f64,
    pub outputs: &'a KernelOutputs,
    pub ecoimpactscore: f64,
    pub energy_kwh_per_cycle: f64,
    pub waste_reduced_kg_per_cycle: f64,
    pub antsafety_class: &'a str,
    pub hexstamp: &'a str,
    pub date: NaiveDate,
}

/// Ensure header exists; if file is new or empty, write header.
fn ensure_header(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        let mut wtr = csv::Writer::from_path(path)?;
        wtr.write_record(&[
            "nodeid",
            "region",
            "materialmix",
            "target_t90_days",
            "measured_t90_days",
            "r_tox",
            "r_micro",
            "r_worm",
            "r_bee",
            "R_residual",
            "admissible_flag",
            "ecoimpactscore",
            "energy_kWh_per_cycle",
            "waste_reduced_kg_per_cycle",
            "antsafety_class",
            "hexstamp",
        ])?;
        wtr.flush()?;
        return Ok(());
    }

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    let bytes = reader.read_line(&mut first_line)?;
    if bytes == 0 {
        let mut wtr = csv::Writer::from_path(path)?;
        wtr.write_record(&[
            "nodeid",
            "region",
            "materialmix",
            "target_t90_days",
            "measured_t90_days",
            "r_tox",
            "r_micro",
            "r_worm",
            "r_bee",
            "R_residual",
            "admissible_flag",
            "ecoimpactscore",
            "energy_kWh_per_cycle",
            "waste_reduced_kg_per_cycle",
            "antsafety_class",
            "hexstamp",
        ])?;
        wtr.flush()?;
    }

    Ok(())
}

/// Append an admissible row to the shard CSV.
///
/// Callers must ensure the recipe has already passed `econet_tray_kernels::admissible`.
pub fn append_row(config: &OptimizerConfig, row: &ShardRow<'_>) -> anyhow::Result<()> {
    let path = Path::new(&config.shard_path);
    ensure_header(path)?;
    let file = OpenOptions::new().append(true).open(path)?;
    let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(file);

    let materialmix_json = serde_json::to_string(&row.materialmix)?;
    let admissible_flag = "true";

    wtr.write_record(&[
        row.nodeid,
        row.region,
        &materialmix_json,
        &format!("{:.3}", row.target_t90_days),
        &format!("{:.3}", row.outputs.t90_days),
        &format!("{:.5}", row.outputs.r_tox),
        &format!("{:.5}", row.outputs.r_micro),
        &format!("{:.5}", row.outputs.r_worm),
        &format!("{:.5}", row.outputs.r_bee),
        &format!("{:.5}", row.outputs.r_residual),
        admissible_flag,
        &format!("{:.5}", row.ecoimpactscore),
        &format!("{:.5}", row.energy_kwh_per_cycle),
        &format!("{:.5}", row.waste_reduced_kg_per_cycle),
        row.antsafety_class,
        row.hexstamp,
    ])?;
    wtr.flush()?;
    Ok(())
}
