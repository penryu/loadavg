use anyhow::Result;
use glob::glob;
use std::fs::read_to_string;

const SYS_TEMP_PATH: &str = "/sys/class/thermal/thermal_zone*/temp";

pub fn temperature() -> Result<f64> {
    let readings = glob(SYS_TEMP_PATH)?
        .flatten()
        .flat_map(read_to_string)
        .flat_map(|s| s.trim().parse::<f64>())
        .collect::<Vec<_>>();

    #[allow(clippy::cast_precision_loss)]
    let count: f64 = readings.len() as f64;
    let sum: f64 = readings.into_iter().sum();
    let avg = sum / count;
    Ok(avg / 1000.0)
}
