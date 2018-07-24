use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("analysis - first element of slice: {}", slice[0]);
    println!("analysis - num elements: {}", slice.len());
}

fn main() {
    let fixed: [i32; 10] = [21, 344, 2, 78, 10, 22, 98, 41, 90, 47];
    let dyn = [10, 20, 30, 40];

    println!("fixed, first item: {}", fixed[0]);
    println!("dyn, last item: {}", dyn[dyn.len() - 1]);

    println!("fixed, memsize: {} bytes", mem::size_of_val(&fixed));
    println!("dyn, memsize: {} bytes", mem::size_of_val(&dyn));

    println!("Inspecting a slice");
    analyze_slice(&dyn[1..3]);
}