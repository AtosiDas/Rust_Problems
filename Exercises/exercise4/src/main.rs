//Implement a function that checks whether a given number is prime or not.
use std::io;

fn main() {
    println!("Enter a number to check");
    let mut target = String::new();
    io::stdin() 
        .read_line(&mut target)
        .expect("Failed to read line");

    let target: u32 = target.trim().parse().expect("Falied to read");
    println!("The number is: {target}");
    let result = is_prime(target);
    println!("Is the value {target} prime? {result}");
}
fn is_prime(target: u32) -> bool {
    if target == 1 {
        false
    }
    else if target == 2 {
        true
    }
    else if target == 3{
        true
    }
    else{
        let mut count = 2;
        if target % 2 == 0{
            while count <= target / 2 {
                if target % count == 0{
                    return false;
                }
                count += 1;
            }
            return true;
        }
        else {
            while count <= target / 2 + 1{
                if target % count == 0{
                    return false;
                }
                count += 1;
            }
            return true;
        }
    }
}