fn four_operations(num: f64) {
    println!(
        "Four operations:
floor: {},
ceiling: {},
rounded: {},
truncated: {}\n",
        num.floor(),
        num.ceil(),
        num.round(),
        num.trunc()
    )
}

fn folded_operations() {
    let my_vec = vec![8.0_f64, 7.6, 9.4, 10.0, 22.0, 77.345, 10.22, 3.2, -7.77, -10.0];
    let minimum = my_vec.iter().fold(f64::MAX, |current_number, next_number| current_number.min(*next_number));
    let maximum = my_vec.iter().fold(f64::MIN, |current_number, next_number| current_number.max(*next_number));
    println!("minimum: {}, maximum: {}", minimum, maximum);
}

pub fn evaluate_floats() {
    four_operations(9.1);
    four_operations(100.7);
    four_operations(-1.1);
    four_operations(-19.9);

    folded_operations();
}
