// mod control_flows;
// mod collections;
// mod structs;
// mod enums;
// mod structs_proper;
// mod generics;
// mod option_and_result;

// // Can also import all with * instead of {fn_name}
// use crate::collections::{array_test, vector_test, tuple_test, hashmap_test, btreemap_test}; 
// use crate::control_flows::{if_control_flow, match_control_flow, match_control_flow_advanced, match_colors};
// use crate::structs::*;
// use crate::enums::*;
// use crate::structs_proper::proper_structs_test;
// use crate::generics::generics_test;
// use crate::option_and_result::{option_test, result_test};


fn main() {
    // println!("Hello, world!, number: {}", number());
    // let result = multiply(12, 2);
    // println!("{}", result);
    // let test_str = make_str("ONE", "TWO");
    // println!("test str: {:?}", test_str);
    // let country = String::from("Austria");
    // print_country(&country); // We print "Austria"
    // print_country(&country);
    // array_test();
    // vector_test();
    // tuple_test();
    // if_control_flow(3);
    // match_control_flow(1);
    // match_control_flow_advanced("very cold", 2);
    // match_control_flow_advanced("freezing", -6);
    // match_control_flow_advanced("asd", -11);
    // match_colors(255, 15, 5);
    // match_colors(12, 255, 255);
    // match_colors(255, 255, 1);
    // match_colors(255, 255, 255);
    // create_customer();
    // enums_test();
    // proper_structs_test();
    // generics_test();
    // option_test();
    // result_test();
    // hashmap_test();
    // btreemap_test();
    let first = std::thread::spawn(|| {
        println!("hello!");
    });
    let second = std::thread::spawn(|| {
        println!("hello!");
    });
    let third = std::thread::spawn(|| {
        println!("hello!");
    });
    
}

fn number() -> i32 {
    8
}

fn multiply(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
    result
}

fn make_str(original: &str, second: &str) -> String {
    let new_str = format!(
        "This is the first {} and this is the second {}",
        original, second
    );
    new_str
}

fn print_country(country_name: &String) {
    println!("{}", country_name);
}
