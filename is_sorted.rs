fn is_sorted(arr: &[i32]) -> bool {
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("Is the array sorted? {}", is_sorted(&arr));
}
