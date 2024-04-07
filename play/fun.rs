fn main() {
    println!("Hello, world!");
    let a = 2;
    another_function(a);
    println!("post function. {}", a);
}

fn another_function(a: i32) {
    println!("Another function.");
    println!("param. {}", a);
}

// fn main() {
//     println!("Hello, world!");
//     let a = 2;
//     another_function(&a);
//     println!("post function. {}", a);
// }

// fn another_function(a: &i32) {
//     println!("Another function.");
//     println!("param. {}", a);
// }