use std::collections::HashSet;

fn find_duplicates(arr: &[i32]){
    let mut seen = HashSet::new();
    let mut vec = Vec::new();
    
    for num in arr.iter(){
        println!("{:?}", num);
        if !seen.insert(num){
            vec.push(num);
        }
    }
    println!("{:?}, {:?}", vec, seen);
}

fn main() {
    let arr = [4, 3, 2, 7, 8, 2, 3, 1];
    find_duplicates(&arr);
    // println!("Duplicate elements: {:?}", duplicates);
}
