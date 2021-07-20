/*
    Primitive Types--
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
    Floats: f32, f64
    Boolean (bool)
    Characters (char)
    Tuples
    Array
    Rust is a statically typed language, which means that it must know the types of all variables at compile time,
     however, the compiler can usually infer what type we want to use based on the value and how we use it.

*/
pub fn run() {
    // Defaults is i32
    let x = 32;

    // Defaults is f64
    let y = 2.5;

    // Explicitly declare type
    let z: i64 = 1234567890123;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;
    // Same thing as -> let is_active = true;

    println!("{:?}", (x, y, z, is_active));

    let is_greater = 10 > 5;

    println!("is it greater?: {}", is_greater);
    
    // Char
    let some_char = 's';
    let face = '\u{1F600}';

    println!("Some char: {}, face?: {}", some_char, face);

}
