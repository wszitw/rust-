use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let parts: Vec<&str> = line.split_whitespace().collect();
    let a: u64 = parts[0].parse().unwrap();
    let b: u64 = parts[1].parse().unwrap();
    println!("{}", a + b)
}