fn main() {
    println!("Hello, world!");
    let a = 2;
    fun1(a);
    println!("post function fun1: {}", a);
    fun2(a);
    println!("post function fun2. {}", a);
}

fn fun1(a: i32) {
    println!("Another function.");
    println!("param. {}", a);
}

fn fun2(a: &i32) {
    println!("Another function.");
    println!("param. {}", a);
}
