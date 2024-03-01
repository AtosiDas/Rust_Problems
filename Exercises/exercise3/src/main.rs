//Given a string of words, implement a function that returns the shortest word in the string.

fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace() // Split the string into words
        .min_by_key(|word| word.len()) // Find the word with the minimum length
}

fn main() {
    let input = "Hello, I am Atosi.";
    match shortest_word(input) {
        Some(shortest) => println!("The shortest word is: {}", shortest),
        None => println!("No words found in the input string"),
    }
}
