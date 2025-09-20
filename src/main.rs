use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).expect("Failed to read line");
    println!("{input}");

    let zero = String::from("0");
    //let nine = String::from("9");
    if input < zero {
        println!("invalid input. try again.");
    }
    
    
}