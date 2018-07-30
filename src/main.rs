extern crate chrono;
extern crate colored;

mod exercise {
    pub mod objects;
    pub mod arrays;
}

mod objects {
    pub mod person;
}

fn main() {
    println!("Running object tests");
    exercise::objects::test();

    println!("Running array tests");
    exercise::arrays::test();
}