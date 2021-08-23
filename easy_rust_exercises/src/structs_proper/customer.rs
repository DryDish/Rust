use core::panic;

use crate::structs_proper::address::Address;

#[derive(Debug)]
pub struct Customer {
    name: String,
    age: u16,
    sex: char,
    address: Address,
}

use std::fmt::{Display, Formatter, Result};

impl Display for Customer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, , {}, {})", self.name, self.age, self.sex, self.address)
    }
}

impl Customer {
    pub fn new() -> Self {
        Self {
            name: "Default".to_string(),
            age: 255,
            sex: 'd',
            address: Address::new(),
        }
    }
    pub fn from(name: &str, age: u16, sex: char, address: Address) -> Self {
        Self {
            name: name.to_string(),
            age: Customer::set_age(age),
            sex: Customer::set_sex(sex),
            address,
        }
    }

    fn set_age(age: u16) -> u16 {
        if age >= 16 && age < 150 {
            age
        } else {
            panic!("Age must be from 16 to 150")
        }
    }

    fn set_sex(letter: char) -> char {
        match letter {
            letter if letter == 'f' || letter == 'F' => 'f',
            letter if letter == 'm' || letter == 'M' => 'm',
            letter if letter == '?' || letter == 'x' => 'x',
            _ => panic!("The sex must be f/F, m/M or ?/x"),
        }
    }

    pub fn print_customer(&self) {
        let title = "customer-info";
        println!("{:-^80}", title);
        println!(
            "{separator: <2}{: <25}{: <5}{: <5}{: <10}{: <15}{: <10}{separator: >8}",
            "name",
            "age",
            "sex",
            "country",
            "city",
            "postcode",
            separator = "|"
        );
        println!("{0:<40}{0:>40}", "|");
        println!(
            "{separator: <2}{: <25}{: <5}{: <5}{: <10}{: <15}{: <10}{separator: >8}",
            self.name,
            self.age,
            self.sex,
            self.address.country,
            self.address.city,
            self.address.post_code,
            separator = "|"
        );
        println!("{:-^80}", "-");
    }
}

/*
fn print_customer_information(customers: Vec<Customer>) {
    let title = "customer-info";
    println!("{:-^80}", title);
    println!(
        "{separator: <2}{: <25}{: <5}{: <5}{: <10}{: <15}{: <10}{separator: >8}",
        "name",
        "age",
        "sex",
        "country",
        "city",
        "postcode",
        separator = "|"
    );
    println!("{0:<40}{0:>40}", "|");
    for customer in customers
    {
        println!(
            "{separator: <2}{: <25}{: <5}{: <5}{: <10}{: <15}{: <10}{separator: >8}",
            customer.name,
            customer.age,
            customer.sex,
            customer.address.country,
            customer.address.city,
            customer.address.post_code,
            separator = "|"
        );
    }

    println!("{:-^80}", "-");
}
*/
