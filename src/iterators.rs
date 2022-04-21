use std::collections::{HashMap, VecDeque};

pub fn array_iterators_conditions() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6];

    numbers.push(7);

    println!("{:?}", numbers);

    let max = 4;

    for i in 0..max {
        println!("{}", numbers[i]);
    }

    let colors = HashMap::from([("B", "Black"), ("W", "White")]);

    for prop in colors.keys() {
        println!("{}", colors.get(prop).unwrap());
    }

    for i in numbers {
        println!("{}", i);
    }

    struct Worker {
        data: Vec<&'static str>,
    }

    impl Worker {
        fn do_work(&mut self) -> Option<&'static str> {
            self.data.pop()
        }
    }

    let mut obj = Worker {
        data: vec!["a", "b", "c", "d", "e", "f", "g", "h"],
    };

    while let Some(data) = obj.do_work() {
        println!("{}", data)
    }

    let mut value = 0;

    loop {
        value += 1;
        println!("{}", value);

        if value > 2 {
            break;
        }
    }

    let ticcc = loop {
        if true {
            break "A";
        } else {
            break "B";
        }
    };

    println!("{:?}", ticcc);

    let list = vec![1, 2, 3, 4, 5];

    let doubled: Vec<_> = list.iter().map(|x| x * 2).collect();

    println!("{:?}", doubled);

    let mut strings = VecDeque::from(["garbage".to_owned(), "data".to_owned()]);

    strings.iter_mut().for_each(|x| x.clear());

    println!("{:?}", strings);

    let even_numbers: Vec<_> = list.iter().filter(|x| *x % 2 == 0).collect();

    println!("{:?}", even_numbers);

    let mut iterator = list.iter();

    let first_even = iterator.find(|x| *x % 2 == 0);

    let second_even = iterator.find(|x| *x % 2 == 0);

    println!("{:?}", first_even);
    println!("{:?}", second_even);

    strings.push_front("Hello".to_owned());
    strings.push_front("world".to_owned());

    println!("{:?}", strings);

    let names = ["beluga", "hecker", "snowie", "skittle"];

    println!("{:?}", names.join(","));
}
