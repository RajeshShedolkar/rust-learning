fn main() {
    let s = String::from("hello"); // s is the owner of the String data

    let len = calculate_length(&s); // borrowing s

    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to the String data
    s.len()
}
