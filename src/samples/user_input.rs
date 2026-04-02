use std::io;

pub fn evaluate_user_input() {
    let mut input_str = String::new();

    while input_str.trim() != "x" {
        input_str.clear();
        io::stdin().read_line(&mut input_str).unwrap();
        println!("Your str: {}", input_str);
    }
    println!("Bye!")
}
