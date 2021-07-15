// Time: O(N^2)
// Space: O(1)
// Usage: When the data is nearly sorted or when the input is relatively small.
// Reasons: Low overhead, efficient memory usage and stable by nature.

fn insertion_sort(arr: &mut [i32]) {
    let mut min_index: usize;
    let arr_size: usize = arr.len();

    for i in 0..(arr_size - 1) {
        min_index = i;

        for j in (i + 1)..arr_size {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        // Swap
        let aux: i32 = arr[i];
        arr[i] = arr[min_index];
        arr[min_index] = aux;
    }
}

fn print_arr(arr: &[i32]) {
    for i in 0..arr.len() {
        print!("{} ", arr[i]);
    }

    println!();
}

fn main() {
    let mut arr: [i32; 5] = [64, 25, 12, 22, 11];
    insertion_sort(&mut arr);
    print_arr(&arr);
}
