#![allow(dead_code)]

use std::error::Error;
use std::fmt::{Display, Formatter};

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
