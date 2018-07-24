extern crate chrono;

use chrono::prelude::*;
use std::string::String;

struct Person {
    name: String,
    birth_year: i32
}

impl Person {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> i32 {
        let dt = Local::now();
        let current_year: i32 = dt.year();

        current_year - self.birth_year
    }

    pub fn print_info(&self) {
        println!("I am {} and I am {} years old", self.get_name(), self.get_age());
    }
}

pub fn test() {
    let ryan = Person {
        name: "Ryan".to_string(),
        birth_year: 1988
    };

    ryan.print_info();

    let kevin = Person {
        name: "Gaspare Bertoni".to_string(),
        birth_year: 1777
    };

    kevin.print_info();
}