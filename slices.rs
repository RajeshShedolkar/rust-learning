fn main() {
    let arr = [1, 2, 3, 4, 5];

    // Define a slice from the array
    let slice = &arr[1..4]; // This will include elements 2, 3, and 4

    println!("Slice: {:?}", slice);

    // Iterate through the slice
    for element in slice.iter() {
        println!("Element: {}", element);
    }
}
