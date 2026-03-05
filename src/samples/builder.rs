#[derive(Debug)]
enum Lifestate {
   Alive,
   Dead,
   NeverAlive,
   Uncertain,
}

#[derive(Debug)]
struct Character {
   name: String,
   age: u32,
   height: u32,
   weight: u32,
   lifestate: Lifestate,
   can_use: bool,
}

impl Character {
   fn new(name: &str, age: u32, height: u32, weight: u32, alive: bool) -> Self {
      Self {
         name: name.to_string(),
         age,
         height,
         weight,
         lifestate: if alive { Lifestate::Alive } else { Lifestate::Dead },
         can_use: false,
      }
   }

   fn height(mut self, height: u32) -> Self {
      self.height = height;
      self
   }

   fn weight(mut self, weight: u32) -> Self {
      self.weight = weight;
      self
   }

   fn name(mut self, name: &str) -> Self {
      self.name = name.to_string();
      self
   }

   fn build(mut self) -> Result<Character, String> {
      if self.height > 120 && self.weight > 80 && !self.name.to_lowercase().contains("smurf") {
         self.can_use = true;
         Ok(self)
      } else {
         Err("Wrong fields!".to_string())
      }
   }
}

impl Default for Character {
   fn default() -> Self {
      Self::new("Billy", 10, 170, 60, true)
   }
}

pub fn use_builder() {
   let character_1 = Character::default().name("I am Smurf!!!").weight(100).height(160).build();
   let character_2 = Character::default().name("Dickon").weight(60).height(156).build();
   let character_3 = Character::default().name("Tommy").weight(90).height(140).build();

   let v = vec![character_1, character_2, character_3];
   for item in v {
      match item {
         Ok(character) => {
            if character.can_use {
               println!("Valid character: {:?}", character);
            }
         }
         Err(error_info) => println!("Error info: {:?}", error_info),
      }
   }
}
