use std::{collections::HashMap, fs::read_to_string};
fn main() {
    let greetings = "Hello World!";
    println!("{}", greetings);

    greet("mars");

    strings();

    assignments();

    temp();

    borrowing();

    everthing_return_something();

    add_numbers(5, 6);
}

fn greet(target: &str) {
    println!("hello ,{}", target);
}

/* --------------------------------- Strings -------------------------------- */

fn strings() {
    let new_string = "Hello world!".to_string();

    let apple = "apple".to_owned();

    let banana = String::from("banana");

    let mut mew = String::new();

    mew.push_str("Hello");

    mew.push_str(" world!");

    println!("{},{},{}", apple, banana, mew);

    let mew_two = format!("Hello! {}", banana);

    println!("{},{}", new_string, mew_two);

    let going_back_str: &str = &mew;
    let going_back_str_two = &mew_two;

    greet(going_back_str_two);
    greet(going_back_str);

    print_type_of(&"Hi!");
    print_type_of(&String::new());
}

fn print_type_of<T>(_: &T) {
    println!("Type of {}", std::any::type_name::<T>());
}

// &str is the primitive type, an immutable reference to the String

// why we need &str and String
// we need $str if we dont want to change the string and we need String we want
// the ownership of the data

// most of the code contains to_owned() and String::from() as it is faster then
// to_string() method

// &string method can be used anywhere. It automatically turns into the &str
// if the method needs it.

// &str are the pointer to the substring in other string data

/* --------------------------------- Variable Assignments & Mutability ----------------------------------------- */

fn assignments() {
    let mut mutable = 1;

    println!("{}", mutable);

    mutable = 3;

    println!("{}", mutable);

    let twich = "somesome";

    println!("{}", twich);

    let twich = 123;

    println!("{}", twich);
}

// re-assignment value to a variable must of the same time.

// we can assign a different type to a variable by re declaring it using the let keyword

fn temp() {
    let source = read_to_string("./README.md").unwrap();

    let mut files = HashMap::new();

    files.insert("README", source.clone()); // secure the ownership of the variable

    files.insert("READMEW2", source); // copy traits issue if we pass source again here
                                      // should be send as copy in a first place
}

fn borrowing() {
    let source = read_to_string("./README.md").unwrap();

    let mut files = HashMap::new();

    files.insert("README", source.clone());

    files.insert("README2", source);

    let files_ref = &files;

    print_borrowed_files(files_ref);

    let files_ref2 = &mut files;

    print_borrowed_files(files_ref2);
}

fn print_borrowed_files(map: &HashMap<&str, String>) {
    println!("{:?}", map);
}

// :? in the println! is for the debugging format. It is used to ouput the no human
// in the readable form

// to use the mutable reference first reference must end before the second

/* --------------------------------- Style Guides ----------------------------------------- */

// varibales functions and modules are in snake case
// constants are in capital camel case
// contants are in Pascal case

// if ( x > y) {} can be written as if x > y { } // this style is prefered

fn everthing_return_something() {
    let apples = 6;

    let message = if apples == 6 {
        "pile of apple"
    } else if apples > 6 {
        "more then a pile"
    } else {
        "less then a pile"
    };

    println!("{}", message);
}

// what if we add ; it will return the () that is called unit type
// no concept of null and undefined in the rust

// implicit return types

fn add_numbers(left: i64, right: i64) -> i64 {
    return left + right;
}
