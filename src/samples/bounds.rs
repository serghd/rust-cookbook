use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> i64;
}

#[derive(Debug)]
struct Rectangle {
    length: i64,
    height: i64,
}

#[allow(dead_code)]
struct Triangle {
    length: i64,
    height: i64,
}

impl HasArea for Rectangle {
    fn area(&self) -> i64 {
        self.height * self.length
    }
}

// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> i64 {
    t.area()
}

pub fn process_bounds() {
    let rectangle = Rectangle { length: 100, height: 200 };
    let _triangle = Triangle { length: 12, height: 15 };

    print_debug(&rectangle);
    println!("Area: {:?}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`.
}
