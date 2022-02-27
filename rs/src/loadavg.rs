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
        write!(f, "{:0.2} {:0.2} {:0.2}", one, five, fifteen)
    }
}

pub fn read() -> LoadAvg {
    let mut load = LoadAvg::default();

    unsafe {
        let _ = getloadavg(&mut load.one as *mut f64, 3);
    }

    load
}
