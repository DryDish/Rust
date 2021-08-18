/*
Unit struct                         = doesn't have anything
struct UnitStruct;

Tuple struct / unnamed struct       = no variable names, only types
struct ImageSize(u32, u32);

Named struct                        = most commonly used, name and type
struct Person {
    name: String,
    age: u8,
    sex: char,
}
*/

struct Address {
    city: String,
    country: String,
    post_code: u16,
}

struct Customer {
    name: String,
    age: u16,
    sex: char,
    address: Address,
}

pub fn create_customer() {
    let mut customers: Vec<Customer> = Vec::new();
    let peter = Customer {
        name: String::from("Peter Piper Piperson"),
        age: 25,
        sex: 'm',
        address: Address {
            city: "Amsterdam".to_string(),
            country: "Holland".to_string(),
            post_code: 12345,
        },
    };

    let sam = Customer {
        name: String::from("Sam Anderson"),
        age: 19,
        sex: 'f',
        address: Address {
            city: "Copenhagen".to_string(),
            country: "Denmark".to_string(),
            post_code: 5541,
        },
    };

    customers.push(peter);
    customers.push(sam);

    print_customer_information(customers);
}

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
