use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Thing {
    object: String,
    size: u16,
}

pub fn generics_test() {
    let something = "This is a thing";
    let something_else: [u8; 4] = [1, 2, 3, 4];
    let a_tuple = ("this", "other".to_string(), 1234);
    let thing_thong = Thing {
        object: "Horse".to_string(),
        size: 200,
    };

    print_whatever_and_return(&something);
    print_whatever_and_return(&something_else);
    print_whatever_and_return(&a_tuple);
    print_whatever_and_return(&thing_thong);
}

fn print_whatever_and_return<T: Debug>(printable: &T) {
    println!("Here is your thing printed: {:?}", &printable);
    compare_and_print("Text im passing", 10, 5);
}

// PartialOrd is the required trait for comparisons with < > and =
fn compare_and_print<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) 
/* Alternative syntax: 
*   fn compare_and_print<T, U>(statement: T, num_1: U, num_2: U)
*   where
*       T: Display,
*       U: Display + PartialOrd,
*   { //code here }
*/
{
    println!(
        "{statement}. Is {a} > than {b}? It is {result}",
        statement = statement,
        a = num_1,
        b = num_2,
        result = num_1 > num_2
    );
}
