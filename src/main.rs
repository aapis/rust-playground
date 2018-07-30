extern crate chrono;
extern crate colored;

use colored::*;

mod exercise {
    pub mod objects;
    pub mod arrays;
    pub mod loops;
}

mod objects {
    pub mod person;
}

fn main() {
    println!("{}", "Running object tests".red());
    exercise::objects::test();

    println!("{}", "Running array tests".red());
    exercise::arrays::test();

    println!("{}", "Running loop tests".red());
    exercise::loops::test();
}