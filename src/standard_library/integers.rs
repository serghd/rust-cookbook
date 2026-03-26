#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display, Formatter, write};
use std::ops::Add;

#[derive(Clone)]
struct Country {
    name: String,
    population: u32,
    gdp: u32,
}

impl Country {
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self { name: name.to_string(), population, gdp }
    }
}

impl Add for Country {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            name: format!("{} and {}", self.name, rhs.name),
            population: self.population + rhs.population,
            gdp: self.gdp + rhs.gdp,
        }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "In {} are {} people and a GDP of ${}", self.name, self.population, self.gdp)
    }
}

pub fn print_countries_info() {
    let nauru = Country::new("Nauru", 10_670, 160_000_000);
    let vanuatu = Country::new("Vanuatu", 307_815, 820_000_000);
    let micronesia = Country::new("Micronesia", 104_468, 367_000_000);

    println!("{}", nauru.clone());
    println!("{}", vanuatu.clone() + micronesia);
}
