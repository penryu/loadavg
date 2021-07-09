mod loadavg;
mod temp;

fn main() {
    let mut output = loadavg::read();

    if let Some(temp) = temp::read() {
        output += &format!(" / {}", temp);
    }

    println!("{}", output);
}
