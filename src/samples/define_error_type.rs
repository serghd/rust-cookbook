use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
struct DoubleError;

type CustomResult<T> = Result<T, DoubleError>;

impl Display for DoubleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first(v: Vec<&str>) -> CustomResult<i32> {
    v.first().ok_or(DoubleError).and_then(|first| first.parse::<i32>().map_err(|_| DoubleError).map(|s| s * 2))
}

fn print(result: CustomResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn use_custom_error() {
    let v1 = vec!["1", "2", "3"];
    let v2 = vec![];
    let v3 = vec!["a", "b", "c"];

    print(double_first(v1));
    print(double_first(v2));
    print(double_first(v3));
}
