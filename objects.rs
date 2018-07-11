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
        let current_year: i32 = 2018;

        current_year - self.birth_year
    }
}

fn main() {
    let me = Person {
        name: "Ryan".to_string(),
        birth_year: 1988
    };

    println!("I am {} and I am {} years old", me.get_name(), me.get_age());
}