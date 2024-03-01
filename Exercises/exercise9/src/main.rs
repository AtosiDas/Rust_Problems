//Reverse a string in Rust
use std::io;

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

fn main() {
    println!("Enter a string!");
    let mut original_string = String::new();
    io::stdin() 
        .read_line(&mut original_string)
        .expect("Failed to read line");

    let reversed_string = reverse_string(&original_string);
    println!("Original string: {}", original_string);
    println!("Reversed string: {}", reversed_string);
}
