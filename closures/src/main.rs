fn main() {
    println!("Hello, world!");

    closures_in_rust()
}

// <------------------------------- Closures ----------------------------------->

fn closures_in_rust() {
    let my_closure = || {
        println!("Hello, world!");
    };

    let multiple_of_two = |num: i64| {
        println!("{}", num * 2);
    };

    let addition = |num1: i64, num2: i64| num1 + num2;

    my_closure();

    multiple_of_two(5);

    println!("{}", addition(2, 2));

    let name = "beluga";

    let print_name = || {
        println!("{}", name);
    };

    print_name();

    let mut counter = 0;

    let mut counter_clouser = || {
        counter += 1;
        println!("counter closure call number of times: {}", counter);
    };

    counter_clouser();

    counter_clouser();

    let multiple = move |num1: i64, num2: i64| num1 * num2;

    let result = multiple(8, 712);

    println!("result: {}", result);

    let plus_two = make_add(4);

    println!("{}", plus_two(10));

    println!("{}", plus_two(19));
    println!("{}", plus_two(18));
    println!("{}", plus_two(17));
    println!("{}", plus_two(112));
    println!("{}", plus_two(190003));

    let name = "hecker".to_owned();

    let consuming_closure = move || name.into_bytes();

    println!("{:?}", consuming_closure());

    double_plus_two();
    calling_dynamic_closure()
}

fn make_add(right: i64) -> impl Fn(i64) -> i64 {
    move |left: i64| {
        println!("{} + {} = {}", left, right, left + right);
        left + right
    }
}

fn compose<T>(f: impl Fn(T) -> T, g: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |i: T| f(g(i))
}

fn double_plus_two() {
    let plus_two = make_add(2);
    let double = |num1: i64| num1 * 2;

    let double_plus_two_closure = compose(plus_two, double);

    println!("{} * 2 + 2 {}", 10, double_plus_two_closure(10));
}

struct DynamicBehaviour<T> {
    closure: Box<dyn Fn(T) -> T>,
}

impl<T> DynamicBehaviour<T> {
    fn new(closure: Box<dyn Fn(T) -> T>) -> Self {
        Self { closure }
    }

    fn run(&self, arg: T) -> T {
        (self.closure)(arg)
    }
}

fn calling_dynamic_closure() {
    let square = DynamicBehaviour::new(Box::new(|num: i64| num * num));

    println!("{} squared with {} is  {}", 5, 5, square.run(5))
}

// Fn borrows the value from its closes over
// FnMut => mutably borrows the variable

// FnOnce loses ownership once called
