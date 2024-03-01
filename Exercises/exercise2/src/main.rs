use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut result: Option<usize> = None;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            result = Some(mid);
            right = mid - 1; // Move left to find earlier occurrences
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

fn main() {
    let arr = vec![1, 2, 2, 3, 3, 3, 4, 5, 5, 6];
    println!("Enter a number to find");
    let mut target = String::new();
    io::stdin() 
        .read_line(&mut target)
        .expect("Failed to read line");

    let target: i32 = target.trim().parse().expect("Falied to read");
    match find_first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} does not exist in the array", target),
    }
}
