#![allow(dead_code)]

use std::borrow::Cow;

fn modulo_3(input: u32) -> Cow<'static, str> {
   match input % 3 {
      0 => "Reminder is 0".into(),
      1 => "Reminder is 1".into(),
      input => format!("Reminder is {}", input).into(),
   }
}

pub fn evaluate_cow() {
   for number in 1..=6 {
      match modulo_3(number) {
         Cow::Borrowed(borrowed) => {
            println!("{} went in. Now Cow is borrowed with this message: {}", number, borrowed)
         },
         Cow::Owned(owned) => {
            println!("{} went in. Now Cow is owned with this message {}", number, owned)
         },
      }
   }
}

////////////////////////////////////////////

fn add_element(vec: &'_ [u32], elem: u32) -> Cow<'_, [u32]> {
   if vec.contains(&elem) {
      println!("Borrowed");
      Cow::Borrowed(vec)
   } else {
      println!("Owned");
      let mut v = vec.to_vec();
      v.push(elem);
      Cow::Owned(v)
   }
}

pub fn evaluate_cow_vector() {
   let v = vec![1, 2, 3];

   let result1 = add_element(&v, 2); // Borrowed
   let result2 = add_element(&v, 4); // Owned

   println!("{:?}", result1);
   println!("{:?}", result2);
}
