use std::collections::HashSet;

fn find_duplicates(arr: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();

    for &num in arr.iter() {
        if !seen.insert(num) {
            duplicates.push(num);
        }
    }

    duplicates
}

fn main() {
    let arr = [4, 3, 2, 7, 8, 2, 3, 1];
    let duplicates = find_duplicates(&arr);
    println!("Duplicate elements: {:?}", duplicates);
}
