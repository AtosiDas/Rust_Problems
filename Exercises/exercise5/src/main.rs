//Given a sorted array of integers, implement a function that returns the median of the array.

fn main() {
    let arr = [1,2,3,4,5,6,7,8,9,10];
    let median = find_median(&arr);
    println!("The median of the array is: {median}"); 
}

fn find_median(arr: &[u32]) -> f64 {
    if arr.len() % 2 != 0 {
        let index = arr.len() / 2;
        arr[index] as f64
    }
    else {
        let index = arr.len() / 2;
        let ave = (arr[index] as f64 + arr[index - 1] as f64) / 2.0;
        return ave;
    }
}