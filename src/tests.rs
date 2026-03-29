#![allow(dead_code)]

fn return_two() -> u32 {
    2
}

fn return_six() -> u32 {
    return_two() + 4
}

#[cfg(test)]
mod tests {
    use super::*;

    fn it_returns_six() {
        assert_eq!(return_six(), 6);
    }

    fn it_returns_two() {
        assert_eq!(return_two(), 2);
    }
}

////////////////////////////////////////////

#[derive(Clone, Debug)]
struct Calculator {
    results: Vec<String>,
    current_input: String,
    total: i32,
    adds: bool,
}

impl Calculator {
    fn new() -> Self {
        Self { results: vec![], current_input: String::new(), total: 0, adds: true }
    }

    fn clear_current_input(&mut self) {
        self.current_input.clear();
    }

    fn push_char_in_current_input(&mut self, character: char) {
        self.current_input.push(character);
    }
}

const OKAY_CHARACTERS: &str = "1234567890+- ";

fn math(input: &str) -> i32 {
    if !input.chars().all(|character| OKAY_CHARACTERS.contains(character))
        || !input.chars().take(2).any(|character| character.is_numeric())
    {
        panic!("Please only input numbers, +-, or spaces");
    }

    let input = input.trim_end_matches(['+', '-', ' ']).replace(' ', "");

    let mut calculator = Calculator::new();

    for character in input.chars() {
        match character {
            '+' => {
                if !calculator.current_input.is_empty() {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.clear_current_input();
                }
            },
            '-' => {
                if calculator.current_input.contains('-') || calculator.current_input.is_empty() {
                    calculator.push_char_in_current_input(character)
                } else {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.clear_current_input();
                    calculator.push_char_in_current_input(character);
                }
            },
            number => {
                if calculator.current_input.contains('-') {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.clear_current_input();
                    calculator.push_char_in_current_input(number);
                } else {
                    calculator.push_char_in_current_input(number);
                }
            },
        }
    }
    calculator.results.push(calculator.current_input);

    for entry in calculator.results {
        if entry.contains('-') {
            if entry.chars().count() % 2 == 1 {
                calculator.adds = match calculator.adds {
                    true => false,
                    false => true,
                };
                continue;
            } else {
                continue;
            }
        }
        if calculator.adds {
            calculator.total += entry.parse::<i32>().unwrap();
        } else {
            calculator.total -= entry.parse::<i32>().unwrap();
            calculator.adds = true;
        }
    }
    calculator.total
}

#[test]
fn one_plus_one_is_two() {
    assert_eq!(math("1        +1"), 2);
}

#[test]
fn one_minus_two_is_minus_one() {
    assert_eq!(math("1 - 2"), -1);
}

#[test]
fn one_minus_minus_one_is_two() {
    assert_eq!(math("1 - -1"), 2);
}

#[test]
fn nine_plus_nine_minus_nine_minus_nine_is_zero() {
    assert_eq!(math("9+9-9-9"), 0);
}

#[test]
fn eight_minus_nine_plus_nine_is_eight_even_with_characters_on_the_end() {
    assert_eq!(math("8  - 9     +9-----+++++"), 8);
}

#[test]
#[should_panic]
fn panics_when_characters_not_right() {
    math("7 + seven");
}
