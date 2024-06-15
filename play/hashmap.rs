use std::collections::HashMap;

fn main() {
    // Creating a HashMap
    let mut my_map = HashMap::new();
    
    // Adding key-value pairs
    my_map.insert("key1", "value1");
    my_map.insert("key2", "value2");
    my_map.insert("key3", "value3");
    
    // Accessing a value
    if let Some(value) = my_map.get("key1") {
        println!("The value for 'key1' is {}", value);
    } else {
        println!("'key1' not found");
    }
    
    // Adding a new key-value pair
    my_map.insert("key4", "value4");
    
    // Iterating over the HashMap
    for (key, value) in &my_map {
        println!("{}: {}", key, value);
    }
}
