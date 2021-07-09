use std::fmt;

extern "C" {
    fn getloadavg(loadavg: *mut f64, count: i32) -> i32;
}

pub struct LoadAvg(f64, f64, f64);

impl fmt::Display for LoadAvg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0.2} {:0.2} {:0.2}", self.0, self.1, self.2)
    }
}

pub fn read() -> LoadAvg {
    let mut loadavg = [0f64; 3];
    let count: i32;

    unsafe {
        count = getloadavg(&mut loadavg as *mut f64, 3);
    }

    assert_eq!(count, 3);

    LoadAvg(loadavg[0], loadavg[1], loadavg[2])
}
