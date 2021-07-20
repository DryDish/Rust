// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run()
{
    // type str [immutable]
    let hello = "Hello there";
    // type String
    let mut other_hello = String::from("Hello there!");

    println!("str: {}, String: {}", hello, other_hello);
    
    // Length
    println!("Length of str: {}", hello.len());

    // Add to String
    // Add char
    other_hello.push(' ');
    other_hello.push('G');
    other_hello.push_str("eneral Kenobi!");

    println!("Modified str: {}, Modified String: {}", hello, other_hello);

    // String methods
    println!("Capacity in bytes: {}", other_hello.capacity());

    println!("Checking if its empty: {}", other_hello.is_empty());

    println!("Does it contain a substring?: {}", other_hello.contains("Kenobi"));

    // Replace returns a new string.
    println!("Replace Kenobi with Peter: {}", other_hello.replace("Kenobi", "Peter"));

    // Loop through string by whitespace
    for word in other_hello.split_whitespace()
    {
        println!("{}", word);
    }

    // Create a string with capacity
    let mut custom_string = String::with_capacity(10);
    custom_string.push('a');
    custom_string.push('b');
    println!("Custom String: {}", custom_string);


    // Assertions:
    
    // No error because it passed, passes silently
    assert_eq!(2, custom_string.len(), "Checking that custom_string len is 2");
    assert_eq!(10, custom_string.capacity(), "Checking that capacity is 10");

    // This will crash since the check fails
    // assert_eq!(11, custom_string.capacity(), "checking that capacity is 11");
}