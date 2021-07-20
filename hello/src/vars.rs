// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run()
{
    let name = "Bob";
    let age = 28;

    // does not work since its immutable
    // age = 29;
    println!("My name is {} and i am {}", name, age);

    let name2 = "Alice";
    // make age2 mutable
    let mut age2 = 30;

    println!("My name is {} and i am {}", name2, age2);
    age2 = 31;
    println!("My name is {} and i am now {}", name2, age2);

    // Define constant
    // Must declare type when using const
    // Convention is for const to be capitalized
    const ID:i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Bob", 39);
    
    println!("{} is {}", my_name, my_age);

}