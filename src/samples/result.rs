use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

// map for Result (using combinators 'and_then()', 'map()')
fn multiply_with_map(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str
        .parse::<i32>()
        .and_then(|first_number| second_number_str.parse::<i32>().map(|second_number| first_number * second_number))
}

fn print(res: &Result<i32, ParseIntError>) {
    match res {
        Ok(n) => println!("number is: {}", n),
        Err(e) => println!("error: {}", e),
    }
}

// aliases for Result
type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply_with_map_aliased(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str
        .parse::<i32>()
        .and_then(|number_one| second_number_str.parse::<i32>().map(|number_two| number_one * number_two))
}

fn print_aliased(res: &AliasedResult<i32>) {
    match res {
        Ok(n) => println!("Aliased number: {}", n),
        Err(e) => println!("Aliased number err: {}", e),
    }
}

// early returns
fn multiply_early_returns(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    let second_number = match second_number_str.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    Ok(first_number * second_number)
}

// operator "?"
fn multiply_with_question_mark(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}

pub fn use_result() {
    let n = multiply("10", "3");
    let m = multiply_with_map("10", "4");
    let aliased = multiply_with_map_aliased("11", "7");
    let early_returns = multiply_early_returns("2", "2");
    let qm = multiply_with_question_mark("5", "5");

    print(&n);
    print(&m);
    print_aliased(&aliased);
    print(&early_returns);
    print(&qm);
}
