use std::{collections::HashMap, fmt::Display, fmt::Result, fs::read_to_string};

mod sub_module;

fn main() {
    let greetings = "Hello World!";
    println!("{}", greetings);
    caller();

    sub_module::print(sub_module::submodule::MSG);
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

/* --------------------------------- HashMap ----------------------------------------- */

fn hash_mapp() {
    let mut map = HashMap::new();

    map.insert("Key1", "Hello");
    map.insert("Key2", "World");

    println!("{:?}", map.get("Key1"));
    println!("{:?}", map.get("Key2"));
    println!("{:?}", map.get("Key3"));

    let value = map.get("Key2");

    println!(
        "{}",
        if value.is_none() == true {
            "None"
        } else {
            value.unwrap()
        }
    );

    println!("{}", map.get("key4").unwrap_or(&"Nothing"));
}

/* --------------------------------- Structs ----------------------------------------- */

fn struct_interfaces_objects() {
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    struct HouseLight {
        on: bool,
    }

    impl TrafficLight {
        pub fn new() -> Self {
            Self {
                color: ColorMatter::Red.to_string(),
            }
        }

        pub fn get_state(&self) -> &String {
            &self.color
        }

        pub fn turn_green(&mut self) {
            self.color = ColorMatter::Green.to_string()
        }

        pub fn turn_yellow(&mut self) {
            self.color = ColorMatter::Yellow.to_string()
        }
    }

    impl Display for TrafficLight {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
            write!(f, "TrafficLight color is {}", self.color)
        }
    }

    impl Display for HouseLight {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
            write!(
                f,
                "House light is {}",
                if self.on == true { "on" } else { "off" }
            )
        }
    }

    impl HouseLight {
        pub fn new() -> Self {
            Self { on: false }
        }

        pub fn get_state(&self) -> bool {
            self.on
        }
    }

    let mut dark = TrafficLight::new();

    let light = TrafficLight {
        color: "red".to_owned(),
    };

    enum ColorMatter {
        Red,
        Yellow,
        Green,
    }

    impl Display for ColorMatter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
            let color_string = match self {
                ColorMatter::Red => "Red",
                ColorMatter::Green => "Green",
                ColorMatter::Yellow => "Yellow",
            };

            write!(f, "{}", color_string)
        }
    }

    trait Light {
        fn get_name(&self) -> &str;
        fn get_state(&self) -> &dyn std::fmt::Debug;
    }

    impl Light for HouseLight {
        fn get_name(&self) -> &str {
            "House Light"
        }

        fn get_state(&self) -> &dyn std::fmt::Debug {
            &self.on
        }
    }

    impl Light for TrafficLight {
        fn get_name(&self) -> &str {
            "Traffic Light"
        }

        fn get_state(&self) -> &dyn std::fmt::Debug {
            &self.color
        }
    }

    fn print_state(light: &impl Light) {
        println!("{:?}", light.get_name())
    }

    println!("{}", light);
    println!("{:?}", dark);
    dark.turn_green();
    dark.turn_yellow();
    println!("{}", dark.get_state());
}

// we send the reference of self aka borrow due to giving ownership will result in
// losing the access of the object

// to make mutable version of the traffic light we need to pass &mut reference

/* --------------------------------- Enums  ----------------------------------------- */

fn colors() {
    #[derive(Debug)]
    enum ColorMatter {
        Red,
        Yellow,
        Green,
    }

    println!("{:?}", ColorMatter::Red);
}

/* --------------------------------- Modules ----------------------------------------- */

/* --------------------------------- Caller Function ----------------------------------------- */

fn caller() {
    greet("mars");

    strings();

    assignments();

    temp();

    borrowing();

    everthing_return_something();

    add_numbers(5, 6);

    hash_mapp();

    struct_interfaces_objects();

    colors();
}
