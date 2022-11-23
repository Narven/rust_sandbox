// Primitives str = immutable fixed-length somewhere in memory
// String = Growable, heap-allocated data structure - use when you need to modify or own string data

pub fn run() {
    // str
    let hellow = "Hellow";

    // String
    let mut hello = String::from("Hello ");

    // get length
    println!("Length: {}", hello.len());

    // push on a char
    hello.push(' ');

    // push on a String
    hello.push_str(" World");

    println!("{}", hello);

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check if empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Contains (world): {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "Their"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
