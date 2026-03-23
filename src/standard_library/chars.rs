use rand::RngExt;

pub fn eval_chars() {
    let character = char::from(99);
    println!("{}", character);

    let mut random_generator = rand::rng();
    for _ in 0..40_000 {
        let bigger_character =
            char::try_from(random_generator.random_range(u32::MIN..u32::MAX)).unwrap_or('-');
        print!("{}", bigger_character);
    }
}
