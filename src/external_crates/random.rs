#![allow(dead_code)]

use rand::RngExt;
use std::fmt::{Display, Formatter};

struct Character {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

fn three_die_six() -> u8 {
    let mut generator = rand::rng();
    let mut result = 0;
    for _ in 0..3 {
        result += generator.random_range(0..=6);
    }
    result
}

fn four_die_six() -> u8 {
    let mut generator = rand::rng();
    let mut vec_res: Vec<u8> = vec![];

    for _ in 0..4 {
        vec_res.push(generator.random_range(0..=6));
    }
    vec_res.sort();
    vec_res.remove(0);
    vec_res.iter().sum()
}

enum Dice {
    THREE,
    FOUR,
}

impl Character {
    fn new(dice: Dice) -> Self {
        match dice {
            Dice::THREE => Character {
                strength: three_die_six(),
                dexterity: three_die_six(),
                constitution: three_die_six(),
                intelligence: three_die_six(),
                wisdom: three_die_six(),
                charisma: three_die_six(),
            },
            Dice::FOUR => Character {
                strength: four_die_six(),
                dexterity: four_die_six(),
                constitution: four_die_six(),
                intelligence: four_die_six(),
                wisdom: four_die_six(),
                charisma: four_die_six(),
            },
        }
    }

    fn display(&self) {
        println!("{}", self)
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Your character has these stats:
{}
{}
{}
{}
{}
{}",
            self.strength,
            self.dexterity,
            self.constitution,
            self.intelligence,
            self.wisdom,
            self.charisma
        )
    }
}

pub fn display_characters() {
    let character1 = Character::new(Dice::THREE);
    let character2 = Character::new(Dice::FOUR);

    character1.display();
    character2.display();
}
