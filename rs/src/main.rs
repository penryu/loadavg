#![warn(clippy::pedantic)]

use cfg_if::cfg_if;

mod common;
use common::LoadAvg;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        use linux as curr_os;
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        use macos as curr_os;
    } else {
        mod unsupported;
        use unsupported as curr_os;
    }
}

fn main() {
    let mut output = LoadAvg::new().to_string();

    if let Ok(temp) = curr_os::temperature() {
        output.push_str(&format!(" / {temp:.1}°C"));
    }

    println!("{output}");
}
