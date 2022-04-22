use std::io::stdin;

fn main() {
    println!("Please enter you weight (KG)");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    let mars_weight = calculate_weight_on_mars(weight);
    println!("Your weight on the mars is {} Kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

// println! is a macro
// use cargo expand which is use to see the macro code

// variable ownership in rust

// 1. Each value in Rust is owned by a variable

// 2. When the owner goes out of scope, the value will be deallocated.

// There can be only One owner at a time.

// let mut input = String::new()
// input points to the location in heap where string value is saved
// String is a type of smart pointer, it automatically deallocated when goes out of scope
// let mut s = input
// now s and input both points to same memory location
// this give an error of memory corruption called double free.
//  As one goes out of scope result in losing both values
// thus this is solved by rust and now s has borrowed the input value and input no longer hold the pointer value address
