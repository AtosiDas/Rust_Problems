//Find the maximum subarray sum in Rust

fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = std::i32::MIN; // Initialize max_sum to the smallest possible value
    let mut current_sum = 0;

    for &num in nums {
        current_sum = current_sum.max(0) + num; // Update current_sum (if negative, reset to 0)
        max_sum = max_sum.max(current_sum); // Update max_sum if current_sum is greater
    }

    max_sum
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4, 8, -5];
    let max_sum = max_subarray_sum(&nums);
    println!("Maximum subarray sum: {}", max_sum);
}
