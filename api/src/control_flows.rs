pub fn if_control_flow(some_number: u8) {
    if some_number == 1 {
        println!("The number was one");
    } else if some_number == 2 {
        println!("The number was two");
    } else if some_number == 3 {
        println!("The number was three");
    } else {
        println!("The number was {}", some_number);
    }
}

pub fn match_control_flow(number: u8) {
    match number {
        // kinda like:
        1 => println!("The number was one"),        // If
        2 => println!("The number was two"),        // else if
        3 => println!("The number was three"),      // else if
        _ => println!("The number was {}", number), // else
    };
}

pub fn match_control_flow_advanced(weather: &str, temperature: i8) {
    match (weather, temperature) {
        // kinda like:
        ("cold", 8..=11) => println!("The weather is cold today"),                              // If
        ("cold", 4..=7) => println!("The weather is quite cold today"),                         // else if
        ("very cold", 1..=3) => println!("The weather is very cold today"),                     // else if
        ("very cold" | "freezing", -10..=0) => println!("The weather is sub zero today."),      // else if
        (weather, temperature) if temperature < -10 && weather != "" =>                 // else if
            println!("The weather is unthinkably cold"),  
        _ => println!(" The weather was specified incorrectly"),                                // else
    };
}

pub fn match_colors(r: u8, g:u8, b: u8) {
    match (r,g,b) {
        (r, g,b) if r & g & b == 255 => println!("This color is white"),
        (r, g,b) if r & g & b == 0 => println!("This color is black"),
        (r, g, _ ) if r & g == 255 => println!("This color is mostly yellow"),
        (r, _, b ) if r & b == 255 => println!("This color is mostly purple"),
        (_, g, b ) if g & b == 255 => println!("This color is mostly teal"),
        (r, _, _ ) if r == 255 => println!("This color is mostly red"),
        (_, g, _) if g == 255 => println!("This color is mostly green"),
        (_, _, b) if b == 255 => println!("This color is mostly blue"),
        _ => println!("this color has no traits maxed."),
    }
}