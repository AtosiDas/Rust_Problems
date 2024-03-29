//Merge two sorted arrays in Rust

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0); // Pointers for arrays arr1 and arr2 respectively

    // Merge elements from arr1 and arr2 into merged array
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    // Add remaining elements from arr1 (if any)
    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }

    // Add remaining elements from arr2 (if any)
    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    merged
}

fn main() {
    let arr1 = vec![1, 1, 3, 5, 7, 9, 11];
    let arr2 = vec![2, 3, 4, 6, 8, 9, 10];

    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged sorted arrays: {:?}", merged);
}
