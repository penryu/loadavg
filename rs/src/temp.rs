use std::env::consts::OS;
use std::fs;
use std::process::Command;

use glob::glob;

const LINUX_TEMP_PATH: &str = "/sys/class/thermal/thermal_zone*/temp";
const MACOS_TEMP_BIN: &str = "osx-cpu-temp";

fn read_linux() -> Option<f64> {
    let (sum, count) = glob(LINUX_TEMP_PATH)
        .ok()?
        .try_fold((0, 0), |(sum, count), zone| {
            let path = zone.ok()?;
            let filename: &str = path.to_str()?;
            let content = fs::read_to_string(filename).ok()?;
            let reading: i32 = content.trim().parse().ok()?;
            Some((sum + reading, count + 1))
        })?;

    let average = (sum as f64 / count as f64) / 1000.0 ;
    Some(average)
}

fn read_macos() -> Option<f64> {
    let output = Command::new(MACOS_TEMP_BIN).output().ok()?.stdout;
    let line = String::from_utf8(output).ok()?;
    let temp = line.split('°').next()?;
    Some(temp.parse::<f64>().ok()?)
}

pub fn read() -> Option<f64> {
    let temp = match OS {
        "linux" => read_linux(),
        "macos" => read_macos(),
        _ => None
    };
    temp.to_owned()
}
