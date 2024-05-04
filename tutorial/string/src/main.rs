fn main() {
    // println!("Hello, world!");
    // let x = "rajesh shedolkar";
    let s = String::from("Rajesh Shedolkar");
    let y = s.chars().nth(7);

    // This works if not out of index
    // print!("by unwrapping char: {}\n", y.unwrap());

    match y {
        Some(z) => {
            print!("char: {}\n", z);
        },
        None => {
            print!("No value \n");
        }
    }
    // print!("{}", y);

    let is_even = false;
    if is_even{
        print!("even number");
    } else if !is_even{
        print!("Odd number");
    }
    

    for i in 0..10{
        print!("\n{}", i);
    }
    

    // iterate over arrays, map, strings 
    let str = String::from("Hello World Rust");
    print!("first word: {}", first_word(str));


    let num = String::from("Rajesh Shedolkar");
    // onwner_ship(num);
    onwner_ship_with_ref(&num);
    print!("num after function call: {}", num);

    let mut s = String::from("Mutable string ");
    print!("s address: {:p}", s.as_ptr());
    // print!("Before string: {}", s);
    mut_ref_fn(&mut s);
    // print!("Before string: {}", s);
    let s1 = &mut s;
    print!("s1 address: {:p}, {:p}", s1.as_ptr(), &s1);

    print!("\n");
}


fn onwner_ship(num: String){
    print!( "num in ownership function: {}", num);
}

fn onwner_ship_with_ref(num: &String){
    print!( "num in ownership function: {}", num);
}



fn first_word(str: String) -> String{
    let mut ans = String::from("");
    for c in str.chars() {
        print!("\n{}", c);
        ans.push(c);
        if c == ' '{
            break;
        }
    }

    for (i, c) in str.chars().enumerate(){
        print!("({}, {}), ", i, c);
    }

    return ans;
}

fn mut_ref_fn(s1: &mut String){
    s1.push_str("additional text");
}