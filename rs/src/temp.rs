use std::fs;
use std::process::Command;

use glob::glob;

const LINUX_TEMP_PATH: &str = "/sys/class/thermal/thermal_zone*/temp";
const MACOS_TEMP_BIN: &str = "osx-cpu-temp";

fn read_temp_freebsd() -> Option<f64> {
    None
}

fn read_temp_linux() -> Option<f64> {
    let readings = glob(LINUX_TEMP_PATH)
        .ok()?
        .collect::<Result<Vec<_>, _>>()
        .ok()?
        .into_iter()
        .map(|path| fs::read_to_string(path).ok()?.trim().parse::<i32>().ok())
        .collect::<Option<Vec<_>>>()?;

    #[allow(clippy::cast_precision_loss)]
    let count = readings.len() as f64;
    let sum: i32 = readings.iter().sum();
    Some(f64::from(sum) / count / 1000.0)
}

fn read_temp_macos() -> Option<f64> {
    let output = Command::new(MACOS_TEMP_BIN).output().ok()?.stdout;
    let line = String::from_utf8(output).ok()?;
    let temp = line.split('°').next()?;
    temp.parse::<f64>().ok()
}

pub fn read() -> Option<f64> {
    match std::env::consts::OS {
        "freebsd" => read_temp_freebsd(),
        "linux" => read_temp_linux(),
        "macos" => read_temp_macos(),
        _ => None,
    }
}
