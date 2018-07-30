use chrono::prelude::*;
use std::string::String;
use colored::*;

pub struct Person {
    pub name: String,
    pub birth_year: i32
}

impl Person {
    fn get_age(&self) -> i32 {
        let dt = Local::now();
        let current_year: i32 = dt.year();

        current_year - self.birth_year
    }

    pub fn print_info(&self) {
        println!("I am {} and I am {} years old", self.name.green(), self.get_age());
    }
}
