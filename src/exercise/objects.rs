use objects::person;

pub fn test() {
    let ryan = person::Person {
        name: "Ryan".to_string(),
        birth_year: 1988
    };

    ryan.print_info();

    let kevin = person::Person {
        name: "Gaspare Bertoni".to_string(),
        birth_year: 1777
    };

    kevin.print_info();
}