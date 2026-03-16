#![allow(dead_code)]

enum Number {
   UINT(u32),
   INT(i32),
}

fn get_number(number: i32) -> Number {
   let result = match number.is_positive() {
      true => Number::UINT(number as u32),
      false => Number::INT(number),
   };
   result
}

pub fn process_numbers() {
   let my_vec = vec![get_number(-800), get_number(10)];
   for item in my_vec {
      match item {
         Number::UINT(number) => {
            println!("{}", number)
         },
         Number::INT(number) => {
            println!("{}", number)
         },
      }
   }
}
