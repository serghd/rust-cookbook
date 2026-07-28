use crate::samples::enums::Foo::Quz;

#[derive(Debug)]
enum NumberStorage {
    UINT(u32), // "Tuple pattern"
    INT(i32),
    Circle { radius: f32 }, // "Struct pattern"
    Unit,
}

fn get_number(number: i32) -> NumberStorage {
    let result = match number.is_positive() {
        true => NumberStorage::UINT(number as u32),
        false => NumberStorage::INT(number),
    };
    result
}

pub fn process_numbers() {
    let my_vec = vec![get_number(-800), get_number(10), NumberStorage::Circle { radius: 13.0 }, NumberStorage::Unit];
    for item in my_vec {
        match item {
            NumberStorage::UINT(number) => {
                println!("{}", number)
            },
            NumberStorage::INT(number) => {
                println!("{}", number)
            },
            NumberStorage::Circle { radius } => {
                println!("{}", radius)
            },
            NumberStorage::Unit => {
                println!("{:?}", NumberStorage::Unit)
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
    if let Quz(_value @ 1..=100) = c {
        println!("c is one hundred or less");
    }
}
