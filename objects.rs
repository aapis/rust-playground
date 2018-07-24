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

    fn print_info(&self) {
        println!("I am {} and I am {} years old", self.get_name(), self.get_age());
    }
}

fn main() {
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