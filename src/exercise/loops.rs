fn while_loop() {
    let mut to = 7;
    while to != 0 {
        println!("{}", to);
        to = to - 1;
    }
}

fn for_loop() {
    let arr = [27, 39, 81, 44];

    for item in arr.iter() {
        println!("at {}", item);
    }
}

pub fn test() {
    while_loop();

    for_loop();
}