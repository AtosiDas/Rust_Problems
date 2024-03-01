//Implement a function that returns the kth smallest element in a given array.

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    i
}

fn quickselect(arr: &mut [i32], low: usize, high: usize, k: usize) -> i32 {
    let pivot_index = partition(arr, low, high);

    if pivot_index == k {
        arr[pivot_index]
    } else if pivot_index < k {
        quickselect(arr, pivot_index + 1, high, k)
    } else {
        quickselect(arr, low, pivot_index - 1, k)
    }
}

fn kth_smallest(arr: &mut [i32], k: usize) -> i32 {
    let n = arr.len();
    quickselect(arr, 0, n - 1, k - 1)
}

fn main() {
    let mut arr = [17, 10, 4, 3, 20, 15];
    let k = 4;
    let kth_smallest_element = kth_smallest(&mut arr, k);
    println!("The {}th smallest element is {}", k, kth_smallest_element);
}
