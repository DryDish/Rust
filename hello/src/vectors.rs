// Vectors are resizable arrays
use std::{mem, str};


pub fn run()
{
    // Must be correct data type and exact length
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // Debug print
    println!("Numbers vector : {:?}", numbers);

    // Retrieve elements by index:
    println!("Retrieving index 2: {}", numbers[2]);


    // To make Arrays mutable add mut
    let mut more_numbers: Vec<&str> = vec!["one", "two", "three", "four", "five"];

    println!("Original: {:?}", more_numbers);

    more_numbers[2] = "changed";

    println!("Altered:  {:?} Vector length: {}", more_numbers, more_numbers.len());

    // Checking memory size
    println!("Vector occupies {} bytes of memory", mem::size_of_val(&more_numbers));

    // Slices [start index...end index -1]
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);

    // Adding to vector
    more_numbers.push("six");
    println!("Pushed vector: {:?}", more_numbers);

    // Remove last
    more_numbers.pop();
    println!("Popped vector: {:?}", more_numbers);

    // Remove from index
    more_numbers.remove(2);
    println!("removed index 2 from vector: {:?}", more_numbers);

    // for each loop
    for x in numbers.iter_mut(){
        *x = *x * 2;
        // Alternative syntax: 
        //*x *= 2;
    }
    println!("Altered vector:  {:?}" , numbers);

}
