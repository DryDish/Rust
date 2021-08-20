use std::fmt::Display;

/*
Option = Something or Nothing
Structure:
    enum Option<T> {
        None,
        Some(T),
    }
*/

// Returns Some(number) if array length is sufficient else returns None
fn take_third<T: Display>(arr: &Vec<T>) -> Option<&T> {
    if arr.len() >= 3 {
        Some(&arr[2])
    } else {
        None
    }
}

fn handle_option<T: Display>(vectors: Vec<Option<T>>) {
    for item in vectors {
        match item {
            Some(result) => println!("This is the result : {}", result),
            None => println!("Index 2 was not found :("),
        }
    }
}

pub fn option_test() {
    let a = vec!["this", "that"];
    let b = vec!["this", "that", "there"];
    let numbers_0 = vec![99, 100];
    let numbers_1 = vec![99, 100, 101, 102];
    let numbers_2 = vec![999, 1000, 1001, 1002, 1003];

    // Add all options to vector and pass vector to function to check
    let mut options_vector = Vec::new();
    options_vector.push(take_third(&numbers_0));
    options_vector.push(take_third(&numbers_1));
    options_vector.push(take_third(&numbers_2));

    handle_option(options_vector);

    // Same as before but more direct.
    // Attempting to unwrap None will crash, but we can check it with .is_some()
    let mother_vec = vec![a, b];
    for vec in mother_vec {
        let result = take_third(&vec);
        if result.is_some() {
            println!("This is the result : {}", result.unwrap());
        } else {
            println!("Index 2 was not found :(");
        }
    }
}

/*
Result = Ok or Error
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
*/

fn is_of_age(age: u8) -> Result<u8, String> {
    match age {
        age if age >= 18 => Ok(age),
        _ => Err("The age is too low.".to_string()),
    }
}

pub fn result_test() {
    let good_age = 21;
    let underage = 16;

    let ok_answer = is_of_age(good_age);
    let err_answer = is_of_age(underage);

    println!("The good result is  : {:?}", ok_answer.unwrap());
    println!("The error result is : {:?}", err_answer);

    let age_vec = vec![16, 17, 18, 19, 20, 21];
    let mut result_vec = Vec::new();
    for age in age_vec {
        result_vec.push(is_of_age(age));
    }
    println!("Begin of 'if let'...");
    for index in 0..=10 {
        // Only read Some() results and ignore None
        // Since we are iterating 10 times, and our vec only has 6, the remaining 4 are Err
        if let Some(_age) = result_vec.get(index) {
            // Only read Ok results and ignore any Err
            // We have multiple underage in our vec, they are ignored
            // Because we are only readying Ok() and underage are Err
            if let Ok(age) = result_vec[index] {
                println!("The age over 18 is: {:?}", age);
            }
        }
    }

    let my_vec = vec![2, 3, 4];

    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {}", number);
        }
    }
}
