// pulling Results out of Options

use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| n * 2));
    opt.transpose()
}

pub fn result_and_option() {
    let numbers = vec!["12", "2", "30"];
    let empty = vec![];
    let strings = vec!["a", "b", "c"];

    println!("The first doubled is: {:?}", double_first(numbers));
    println!("The first doubled is: {:?}", double_first(empty));
    println!("The first doubled is: {:?}", double_first(strings));
}
