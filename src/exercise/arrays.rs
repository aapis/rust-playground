use std::mem;
use colored::*;

fn analyze_slice(slice: &[i32]) {
    println!("analysis - first element of slice: {}", slice[0]);
    println!("analysis - num elements: {}", slice.len());
}

pub fn test() {
    let fixed: [i32; 10] = [21, 344, 2, 78, 10, 22, 98, 41, 90, 47];
    let dyn = [10, 20, 30, 40];

    println!("{} {}", "fixed, first item: ".on_blue(), fixed[0]);
    println!("{} {}", "dyn, last item: ".on_blue(), dyn[dyn.len() - 1]);

    println!("{} {} bytes","fixed, memsize: ".on_blue(), mem::size_of_val(&fixed));
    println!("{} {} bytes","dyn, memsize: ".on_blue(), mem::size_of_val(&dyn));

    println!("{}", "Inspecting a slice".on_blue());
    analyze_slice(&dyn[1..3]);
}