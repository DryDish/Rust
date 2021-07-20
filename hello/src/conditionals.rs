// Conditionals used to check the condition of something
//  and act upon the result

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_is_18 = true;

    if age >= 18 || knows_is_18 {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to be 18+");
    } else {
        println!("I need to check your ID");
    }

    // Shorthand if
    let is_of_age = if age >= 18 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
