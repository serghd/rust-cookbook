#![allow(dead_code)]

pub fn evaluate_vector() {
    let mut v = vec!["sun", "sun", "moon", "moon", "sun", "moon", "moon"];
    v.sort();
    v.dedup();
    println!("v: {:?}", v);
}