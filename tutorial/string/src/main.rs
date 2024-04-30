fn main() {
    // println!("Hello, world!");
    // let x = "rajesh shedolkar";
    let s = String::from("Rajesh Shedolkar");
    let y = s.chars().nth(78);

    match y {
        Some(z) => {
            print!("char: {}\n", z);
        },
        None => {
            print!("No value \n");
        }
    }
    // print!("{}", y);
}
