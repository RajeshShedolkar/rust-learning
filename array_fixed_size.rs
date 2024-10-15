fn main() {
    // Define a fixed-size array of integers
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Access elements of the array
    println!("First element: {}", arr[0]);

    // Iterate through the array
    for element in arr.iter() {
        println!("Element: {}", element);
    }
}
