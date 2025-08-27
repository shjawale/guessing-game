use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input);
    println!("{input}");
    
    
}