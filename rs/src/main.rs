#![warn(clippy::pedantic)]

use libc::getloadavg;
use std::fmt;

/// Holds load readings.
#[derive(Debug, Default)]
pub struct LoadAvg(f64, f64, f64);

impl fmt::Display for LoadAvg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let LoadAvg(x, y, z) = self;
        write!(f, "{x:0.2} {y:0.2} {z:0.2}")
    }
}

fn main() {
    let mut loadavg = LoadAvg::default();

    let _ = unsafe { getloadavg(std::ptr::addr_of_mut!(loadavg.0), 3) };

    if let Ok(temp) = crate::temperature::read() {
        println!("{loadavg} / {temp:.1}°C");
    } else {
        println!("{loadavg}");
    }
}

#[cfg(target_os = "linux")]
mod temperature {
    use anyhow::Result;
    use glob::glob;
    use std::fs::read_to_string;

    const SYS_PATH_TEMP: &str = "/sys/class/thermal/thermal_zone*/temp";

    pub fn read() -> Result<f64> {
        let readings = glob(SYS_PATH_TEMP)?
            .flatten()
            .flat_map(read_to_string)
            .flat_map(|s| s.trim().parse::<f64>())
            .collect::<Vec<_>>();

        let count: f64 = readings.len() as f64;
        let sum: f64 = readings.into_iter().sum();
        let avg = sum / count;
        Ok(avg / 1000.0)
    }
}

#[cfg(target_os = "macos")]
mod temperature {
    use anyhow::{bail, Context, Result};
    use std::process::Command;

    const MACOS_TEMP_BIN: &str = "osx-cpu-temp";

    pub fn read() -> Result<f64> {
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
}
