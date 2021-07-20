
pub fn run()
{
    let person: (&str, &str, i32) = ("Bob", "Male", 22);
    println!("{} is {} and is {} years old.", person.0, person.1, person.2);

}