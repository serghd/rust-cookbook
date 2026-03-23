#![allow(dead_code)]

use std::ops::{Deref, DerefMut};

struct HoldsNumber(u8);

impl HoldsNumber {
    fn print_the_number_two_times(&self) {
        println!("{}", self.0 * 2);
    }
}

impl Deref for HoldsNumber {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HoldsNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn evaluate_deref() {
    let mut item = HoldsNumber(20);
    *item = 30;
    println!("checked_sub: {:?}", item.checked_sub(31));
    item.print_the_number_two_times();
    println!("item: {}", *item);
}
