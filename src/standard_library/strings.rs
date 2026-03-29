#![allow(dead_code)]

pub fn string_capacity() {
    let mut str = String::new();
    let mut capacity_counter = 0;

    for _ in 0..100_000 {
        if str.capacity() != capacity_counter {
            println!("{:?}", str.capacity());
            capacity_counter = str.capacity();
        }
        str.push_str("I'm getting pushed into the string!")
    }
}

pub fn string_popping() {
    let mut str = String::from(".daer ot drah tib elttil a si gnirts sihT");
    loop {
        let pop_result = str.pop();
        match pop_result {
            Some(character) => print!("{}", character),
            None => break,
        }
    }
}

pub fn string_retain() {
    let mut str = "Age: 20 Height: 194 Weight: 80".to_string();
    str.retain(|character| character.is_alphabetic() || character == ' ');
    dbg!(str);
}
