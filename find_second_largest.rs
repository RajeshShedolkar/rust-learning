fn find_second_largest(arr: &[i32]) -> Option<i32> {
    if arr.len() < 2 {
        return None;
    }

    let mut first = i32::MIN;
    let mut second = i32::MIN;

    for &num in arr.iter() {
        if num > first {
            second = first;
            first = num;
        } else if num > second && num < first {
            second = num;
        }
    }

    if second == i32::MIN {
        None
    } else {
        Some(second)
    }
}

fn main() {
    let arr = [7, 5, 6, 1, 4];
    match find_second_largest(&arr) {
        Some(val) => println!("The second largest element is: {}", val),
        None => println!("No second largest element found."),
    }
}
