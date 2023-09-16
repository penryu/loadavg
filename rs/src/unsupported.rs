use anyhow::{bail, Result};
use std::env::consts::OS as current_OS;

pub fn temperature() -> Result<f64> {
    bail!("temperature not implemented for platform {current_OS}");
}
