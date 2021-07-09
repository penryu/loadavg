mod loadavg;
mod temp;

fn main() {
    let loadavg = loadavg::read();

    match temp::read() {
        Some(temp) => println!("{} / {:.1}°C", loadavg, temp),
        None => println!("{}", loadavg),
    };
}
