use anyhow::{bail, Context, Result};
use std::process::Command;

const MACOS_TEMP_BIN: &str = "osx-cpu-temp";

pub fn read_temp() -> Result<f64> {
    let output = Command::new(MACOS_TEMP_BIN).output()?.stdout;
    let line = String::from_utf8(output)?;
    let temp = line
        .split('°')
        .next()
        .context("Unrecognized input from {MACOS_TEMP_BIN}")?;

    match temp.parse::<f64>() {
        Ok(value) if value > 1.0 => Ok(value),
        Ok(_) | Err(_) => bail!("temperature unavailable"),
    }
}
