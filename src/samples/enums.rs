#![allow(dead_code)]

use crate::samples::enums::Foo::Quz;

enum Number {
    UINT(u32),
    INT(i32),
}

fn get_number(number: i32) -> Number {
    let result = match number.is_positive() {
        true => Number::UINT(number as u32),
        false => Number::INT(number),
    };
    result
}

pub fn process_numbers() {
    let my_vec = vec![get_number(-800), get_number(10)];
    for item in my_vec {
        match item {
            Number::UINT(number) => {
                println!("{}", number)
            },
            Number::INT(number) => {
                println!("{}", number)
            },
        }
    }
}

////////////////////////////////////////////

enum Foo {
    Bar,
    Baz,
    Quz(u32),
}

pub fn evaluate_enum() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Quz(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }
    if let Foo::Bar = b {
        println!("b is foobar")
    }
    if let Quz(value) = c {
        println!("c is {}", value);
    }
    if let Quz(value @ 1..=100) = c {
        println!("c is one hundred or less");
    }
}
