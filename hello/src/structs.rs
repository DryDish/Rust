// Used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct ColorTuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        // No semicolon = return
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) 
    {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String)
    {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;
    println!("Color of c: {} {} {}", c.red, c.green, c.blue);

    let mut t = ColorTuple(255, 0, 0);
    t.0 = 200;
    println!("Color of t: {} {} {}", t.0, t.1, t.2);

    let mut p = Person::new("John", "Peterson");

    println!("Person, name, last name {} {}", p.first_name, p.last_name);
    println!("Person full name: {}", p.full_name());

    p.set_last_name("Doe");
    println!("Person full name: {}", p.full_name());

    println!("Person tuple: {:?}", p.to_tuple());
}
