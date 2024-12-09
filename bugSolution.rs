fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe method 1: Match statement
    match vec.get(0) {
        Some(first) => println!("First element: {}", first),
        None => println!("Vector is empty"),
    }

    // Safe method 2: Using get() and checking for None
    if let Some(first) = vec.get(0) {
        println!("First element: {}", first);
    } else {
        println!("Vector is empty");
    }
} 