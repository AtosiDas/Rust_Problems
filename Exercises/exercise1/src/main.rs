//Implement a function that checks whether a given string is a palindrome or not.

use std::io;

fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase(); // Convert the input string to lowercase
    let input = input.chars().filter(|c| c.is_alphanumeric()).collect::<String>(); // Filter out non-alphanumeric characters
    let reversed = input.chars().rev().collect::<String>(); // Reverse the string
    input == reversed // Check if the original string is equal to the reversed string
}

fn main() {
    println!("Enter a string");
    let mut str1 = String::new();
    io::stdin() 
        .read_line(&mut str1)
        .expect("Failed to read line");

    //println!("The string is: {str1}");
    let check = is_palindrome(&str1);
    println!("Is {str1} a palindrome? {check}");
}
