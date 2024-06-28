use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return Some((j, i));
        }
        map.insert(num, i);
    }
    None
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    match two_sum(nums, target) {
        Some((index1, index2)) => println!("Indices: {}, {}", index1, index2),
        None => println!("No solution found!"),
    }
}
