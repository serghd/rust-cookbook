use std::error::Error;
use std::fmt::{Display, Formatter};
use std::mem;

#[derive(Debug)]
struct ErrorOne;

impl Error for ErrorOne {}

impl Display for ErrorOne {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "This is ErrorOne!")
    }
}

#[derive(Debug)]
struct ErrorTwo;

impl Error for ErrorTwo {}

impl Display for ErrorTwo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "This is ErrorTwo!")
    }
}

fn return_error(number: u8) -> Result<String, Box<dyn Error>> {
    match number {
        0 => Err(Box::new(ErrorOne)),
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("No errors!".to_string()),
    }
}

pub fn evaluate_errors() {
    let v = vec![0_u8, 1_u8, 12];

    for number in v {
        match return_error(number) {
            Ok(input) => println!("{}", input),
            Err(input) => println!("Error: {}", input),
        }
    }
}

/////////////////////////////////////////

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

pub fn evaluate_boxes() {
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle { top_left: origin(), bottom_right: Point { x: 3.0, y: -4.0 } };
    let boxed_point: Box<Point> = Box::new(origin());
    let boxed_rectangle: Box<Rectangle> =
        Box::new(Rectangle { top_left: origin(), bottom_right: Point { x: 3.0, y: -4.0 } });
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytes on the stack", mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack", mem::size_of_val(&rectangle));

    // box size == pointer size
    println!("Boxed point occupies {} bytes on the stack", mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on the stack", mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on the stack", mem::size_of_val(&box_in_a_box));

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack", mem::size_of_val(&unboxed_point));
}
