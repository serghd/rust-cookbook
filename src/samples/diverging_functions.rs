fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
        let addition = match i % 2 == 1 {
            true => i,
            false => continue,
        };
        acc += addition;
    }
    acc
}

pub fn process_diverging_functions() {
    let sum  = sum_odd_numbers(155);
    println!("sum: {}", sum);
}