#![allow(dead_code)]

pub mod print_things {
    use std::fmt::Display;
    pub fn prints_one_thing<T: Display>(input: T) {
        println!("{}", input);
    }
}

pub mod country {
    fn print_country(name: &str) {
        println!("Country: {}", name);
    }

    pub mod province {
        fn print_province(name: &str) {
            println!("Province: {}", name);
        }

        pub mod city {
            use super::*;
            use super::super::*;

             pub fn print_city(country: &str, province: &str, city: &str) {
                print_country(country);
                print_province(province);
                println!("City: {}", city);
            }
        }
    }
}