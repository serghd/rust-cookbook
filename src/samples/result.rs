use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => {
            match second_number_str.parse::<i32>() {
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e)
            }
        },
        Err(e) => Err(e)
    }
}

fn multiply_with_map(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError>{
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print(res: &Result<i32, ParseIntError>) {
    match res {
        Ok(n) => {
            println!("number is: {}", n)
        },
        Err(e) => {
            println!("error: {}", e)
        }
    }
}

pub fn use_result() {
    let n = multiply("10", "3");
    let m = multiply_with_map("10", "3");
    print(&n);
    print(&m);

    // // error:
    // let n = multiply("10", "a");
    // let m = multiply_with_map("10", "a");
    // print(&n);
    // print(&m);
}