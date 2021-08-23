#[derive(Debug)]
pub struct Address {
    pub city: String,
    pub country: String,
    pub post_code: u16,
}

use std::fmt::{Display, Formatter, Result};

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.city, self.country, self.post_code)
    }
}

impl Address {
    pub fn new() -> Self {
        Self {
            city: "Default".to_string(),
            country: "Default".to_string(),
            post_code: 12345
        }
    }

    pub fn from(city: &str, country: &str, post_code: u16)-> Self {
        Self {
            city: city.to_string(),
            country: country.to_string(),
            post_code
        }
    }
}