extern crate chrono;
extern crate colored;

use colored::*;

mod exercise {
    pub mod objects;
    pub mod arrays;
    pub mod loops;
    pub mod threads;
}

mod objects {
    pub mod person;
}

fn main() {
    use exercise::*;

    println!("{}", "Running object tests".red());
    objects::test();

    println!("{}", "Running array tests".red());
    arrays::test();

    println!("{}", "Running loop tests".red());
    loops::test();

    println!("{}", "Running thread tests".red());
    threads::test();
}