mod customer;
pub mod address;

use crate::structs_proper::customer::Customer;
use crate::structs_proper::address::Address;

struct TestingStuff {
    name: String,
    number: u8,
    arr: [u8; 2],
}

pub fn proper_structs_test()
{
    let customer_1 = Customer::new();

    let address_2 = Address::from("Copenhagen", "Africa", 9876);
    let customer_2 = Customer::from("Kieth", 18, '?', address_2);
    println!("{:?}", customer_1);
    println!("{:?}", customer_2);
    customer_2.print_customer();

    // Created a variable from the struct
    let destructuring_var = TestingStuff {
        name: "Peter".to_string(),
        number: 12,
        arr: [5, 2],
    };    
    
    // Destructuring of the variable into a b and c
    let TestingStuff {
         name: a, 
         number: b,
         arr: c 
    } = destructuring_var;
         
    println!("Testing_stuff variables: name: {} number: {} and arr: {:?}", a, b, c);

    let address_3 = Address::from("city", "country", 1111);
    let customer_3 = Customer::from("Customer Name", 20, 'x', address_3);
    println!("Customer printed with Display trait: {}", customer_3);
    
}

