fn main() {
    let greetings = "Hello World!";
    println!("{}", greetings);
    greet("mars");
    strings();
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
}

// &str is the primitive type, an immutable reference to the String

// why we need &str and String
// we need $str if we dont want to change the string and we need String we want
// the ownership of the data

// most of the code contains to_owned() and String::from() as it is faster then
// to_string() method

// &string method can be used anywhere. It automatically turns into the &str
// if the method needs it.
