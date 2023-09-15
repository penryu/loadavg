use libc::getloadavg;
use std::fmt;

/// Holds load readings.
#[derive(Debug, Default)]
pub struct LoadAvg(f64, f64, f64);

impl LoadAvg {
    pub fn new() -> Self {
        let mut loadavg = LoadAvg::default();
        let _ = unsafe { getloadavg(std::ptr::addr_of_mut!(loadavg.0), 3) };
        loadavg
    }
}

impl fmt::Display for LoadAvg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let LoadAvg(x, y, z) = self;
        write!(f, "{x:0.2} {y:0.2} {z:0.2}")
    }
}
