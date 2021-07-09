use std::env::consts::OS;
use std::fs;
use std::process::Command;

use glob::glob;

const LINUX_TEMP_PATH: &str = "/sys/class/thermal/thermal_zone*/temp";
const MACOS_TEMP_BIN: &str = "osx-cpu-temp";

fn read_linux() -> Option<String> {
    let mut sum = 0;
    let mut count = 0;

    for zone in glob(LINUX_TEMP_PATH).unwrap() {
        let path = zone.unwrap();
        let filename: &str = path.to_str()?;
        let content = fs::read_to_string(filename).unwrap();
        let reading: i32 = content.trim().parse::<i32>().unwrap();
        sum += reading;
        count += 1;
    }

    let average = (sum as f64 / count as f64) / 1000.0 ;
    Some(format!("{:.1}°C", average))
}

fn read_macos() -> Option<String> {
    let output = Command::new(MACOS_TEMP_BIN).output().unwrap().stdout;
    let reading = String::from_utf8(output).unwrap().trim().to_owned();
    Some(reading)
}

pub fn read() -> Option<String> {
    let temp = match OS {
        "linux" => read_linux(),
        "macos" => read_macos(),
        _ => None
    };
    temp.to_owned()
}
