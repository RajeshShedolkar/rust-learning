fn rotate_array(arr: &mut [i32], k: usize) {
    let len = arr.len();
    let k = k % len; // Handle cases where k > len
    arr.reverse();
    arr[..k].reverse();
    arr[k..].reverse();
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    rotate_array(&mut arr, k);
    println!("Array after rotation: {:?}", arr);
}
