use std::env::args;

enum Letters {
    Nothing,
    Capitalize,
    Lowercase,
}

/**
* cmd command should look like this:
* cargo run --package rust-cookbook --bin rust-cookbook -- capital I think I understand now
*/
pub fn evaluate_args() {
    let mut mode = Letters::Nothing;
    let input = args().collect::<Vec<_>>();

    if input.len() > 2 {
        match input[1].as_str() {
            "capital" => mode = Letters::Capitalize,
            "lowercase" => mode = Letters::Lowercase,
            _ => {},
        }
    }

    for word in input.iter().skip(2) {
        match mode {
            Letters::Capitalize => println!("{}", word.to_uppercase()),
            Letters::Lowercase => println!("{}", word.to_lowercase()),
            _ => println!("{}", word),
        }
    }
}
