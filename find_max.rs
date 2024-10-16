fn find_max(arr: &[i32]) -> i32 {
    let mut max = arr[0]; // Assume the first element is the max
    for &num in arr.iter() {
        if num > max {
            max = num;
        }
    }
    max
}

fn main() {
    let arr = [3, 5, 7, 2, 8];
    println!("The maximum element is: {}", find_max(&arr));
}
