extern "C" {
    fn getloadavg(loadavg: *mut f64, count: i32) -> i32;
}

pub fn read() -> String {
    let mut loadavg = [0f64; 3];
    let count: i32;

    unsafe {
        count = getloadavg(&mut loadavg as *mut f64, 3);
    }

    assert_eq!(count, 3);
    let output = format!("{:0.2} {:0.2} {:0.2}",
        loadavg[0], loadavg[1], loadavg[2]);

    output
}
