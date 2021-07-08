extern {
    fn getloadavg(loadavg: *mut f64, count: i32) -> i32;
}

fn main() {
    let mut readings = [0f64; 3];
    unsafe {
        let count = getloadavg(&mut readings as *mut f64, 3);
        assert_eq!(count, 3);
    }
    println!("{:0.2} {:0.2} {:0.2}", readings[0], readings[1], readings[2]);
}
