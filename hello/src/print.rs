pub fn run()
{
    // print to console
    println!("Hello from print.rs file");
    println!("{} is from {}", "Bob", "New York");
    // index
    println!("{0} is from {1} and {2} is also from {1}", "Bob", "New York", "Alice");
    // Named arguments
    println!("{name} likes to {verb} {activity}", name= "Bob", verb="run", activity="code");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 =  {}", 10 + 10);
}