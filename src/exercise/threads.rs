use std::thread;
use colored::*;

static NTHREADS: i32 = 10;

pub fn test() {
    let mut threads = vec![];

    for i in 0..NTHREADS {
        threads.push(thread::spawn(move || {
            println!("{} {}", "this is thread".blue(), i);
        }));
    }

    for thread in threads {
        let th = thread.join();
        println!("{:?}", th);
    }
}