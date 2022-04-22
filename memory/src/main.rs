
fn main() {
    let a = 2;
    let result = stack_only(a);
    println!("{}", result);
    // println! is a macro 
    // a macro is a code used to write more code
    
}

fn stack_only(b: i32) -> i32 {
    let c = 3;
    return b + c + stack_and_heap(); 
}

fn stack_and_heap() -> i32 {
    let d = 5;
    let e = Box::new(7);
    // smart pointer that does'nt need to be reallocatable once goes out of scope
    // Box::new() points to the location of the value in heap
    return d+ *e;
}

