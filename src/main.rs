use std::io;

fn main() {
    println!("Please enter your name");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error reading input");
    println!("Hello, {input}");
}
