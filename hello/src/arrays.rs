// Fixed list where elements are the same data type
// Arrays are stack allocated
use std::mem;


pub fn run()
{
    // Must be correct data type and exact length
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // Debug print
    println!("Numbers array: {:?}", numbers);

    // Retrieve elements by index:
    println!("Retrieving index 2: {}", numbers[2]);


    // To make Arrays mutable add mut
    let mut more_numbers: [&str; 5] = ["one", "two", "three", "four", "five"];

    println!("Original: {:?}", more_numbers);

    more_numbers[2] = "changed";

    println!("Altered:  {:?} Array length: {}", more_numbers, more_numbers.len());

    // Checking memory size
    println!("Array occupies {} bytes of memory", mem::size_of_val(&more_numbers));

    // Slices [start index...end index -1]
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice)

}
