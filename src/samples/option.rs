#![allow(dead_code)]

#[derive(Debug)]
struct User {
   name: String,
}

pub fn create_and_display_object() {
   let user: Option<User> = Some(User { name: String::from("vasya") });

   if let Some(u) = &user {
      println!("#1 {:?}", u);
   };

   match &user {
      Some(u) => {
         println!("#2 {:?}", u);
      },
      None => {
         println!("None");
      },
   }
}

pub fn display_weather() {
   let city_weathers =
      vec![vec!["Berlin", "cloudy", "-5", "10", "12"], vec!["Athens", "24", "40", "32"]];

   for mut weather_info in city_weathers {
      println!("City {}", weather_info[0]);

      while let Some(info_item) = weather_info.pop() {
         if let Ok(value) = info_item.parse::<i32>() {
            println!("Number: {}", value);
         }
      }
   }
}
