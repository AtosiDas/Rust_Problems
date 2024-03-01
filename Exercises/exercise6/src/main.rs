//Implement a function that finds the longest common prefix of a given set of strings.

fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return String::new(); // If the input vector is empty, return an empty string
    }

    let first_str = strs[0]; // Consider the first string as the initial prefix
    let mut prefix = String::new();

    'outer: for (i, ch) in first_str.chars().enumerate() {
        for s in strs.iter().skip(1) {
            if let Some(c) = s.chars().nth(i) {
                if c != ch {
                    break 'outer; // If a character mismatches, break out of the outer loop
                }
            } else {
                break 'outer; // If a string is shorter than the prefix, break out of the outer loop
            }
        }
        prefix.push(ch); // Append the character to the prefix
    }

    prefix
}

fn main() {
    let str1 = vec!["flower", "flow", "flown"];
    let str2 = vec!["dog", "racecar", "car"];

    println!("Longest common prefix of str1: {}", longest_common_prefix(str1));
    println!("Longest common prefix of str2: {}", longest_common_prefix(str2));
}
