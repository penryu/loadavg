#![warn(clippy::pedantic)]

use cfg_if::cfg_if;

mod common;
use common::LoadAvg;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        use linux::temperature;
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        use macos::temperature;
    }
}

fn main() {
    let mut output = LoadAvg::new().to_string();

    if let Ok(temp) = crate::temperature() {
        output.push_str(&format!(" / {temp:.1}°C"));
    }

    println!("{output}");
}
