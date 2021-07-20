use std::env;

pub fn run()
{
    // Collect args from command line
    let args: Vec<String> = env::args().collect();
    // args[0] is executable path
    let mut command: String = String::new();

    // Check that there is more than the base path
    if args.len() > 1
    {
        command = args[1].clone();
    }
    
    let name = "Peter";
    let status = "100%";

    
    if command == "hello"
    {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is: {}", status);
    } else if command != "" {
        println!("Command is not valid");
    }

}