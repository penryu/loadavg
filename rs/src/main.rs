#![warn(clippy::all, clippy::pedantic)]

mod loadavg;
mod temp;

fn main() {
    let loadavg = crate::loadavg::read();

    match crate::temp::read() {
        Some(temp) => println!("{} / {:.1}°C", loadavg, temp),
        None => println!("{}", loadavg),
    };
}
