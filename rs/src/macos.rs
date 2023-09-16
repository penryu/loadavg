use anyhow::{bail, Context, Result};
use std::process::Command;

const MACOS_TEMP_BIN: &str = "osx-cpu-temp";

pub fn temperature() -> Result<f64> {
    let output = Command::new(MACOS_TEMP_BIN).output()?.stdout;

    let line = String::from_utf8(output)?;

    let temp: f64 = line
        .split('°')
        .next()
        .context("Unrecognized input from {MACOS_TEMP_BIN}")?
        .parse()
        .context("unable to read temperature")?;

    if temp < 0.1 {
        bail!("temperature unavailable");
    }

    Ok(temp)
}
