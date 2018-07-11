use std::string::String;

struct Person {
    name: String,
    age: i32
}

impl Person {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> i32 {
        self.age
    }
}

fn main() {
    let me = Person {
        name: "Ryan".to_string(),
        age: 29
    };

    println!("I am {} and I am {} years old", me.get_name(), me.get_age());
}