#![warn(clippy::all, clippy::pedantic)]

mod temp;

use std::fmt;

extern "C" {
    fn getloadavg(loadavg: *mut f64, count: i32) -> i32;
}

pub struct LoadAvg {
    one: f64,
    five: f64,
    fifteen: f64,
}

impl Default for LoadAvg {
    fn default() -> Self {
        LoadAvg {
            one: 0.0,
            five: 0.0,
            fifteen: 0.0,
        }
    }
}

impl fmt::Display for LoadAvg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let LoadAvg { one, five, fifteen } = self;
        write!(f, "{one:0.2} {five:0.2} {fifteen:0.2}")
    }
}

fn main() {
    let mut loadavg = LoadAvg::default();

    unsafe {
        let _ = getloadavg(std::ptr::addr_of_mut!(loadavg.one), 3);
    }

    match crate::temp::read() {
        Some(temp) => println!("{loadavg} / {temp:.1}°C"),
        None => println!("{loadavg}"),
    };
}
