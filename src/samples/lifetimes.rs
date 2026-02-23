use std::fmt::{Display, Formatter};

pub fn get_str() -> &'static str {
   "my string"
}

////////////////////////////////////////////

struct City<'a> {
   name: &'a str,
   date_founded: u32,
}

pub fn show_city_info() {
   let city_names = vec!["City 1", "City 2"];
   let my_city = City { name: &city_names[0], date_founded: 1000 };
   println!("My city {}, was founded at {}", my_city.name, my_city.date_founded);
}

////////////////////////////////////////////

struct Adventurer<'a> {
   name: &'a str,
   hit_points: u32,
}

impl Adventurer<'_> {
   pub fn take_damage(&mut self) {
      self.hit_points -= 100;
   }
}

impl Display for Adventurer<'_> {
   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "{} has {} hit-points", self.name, self.hit_points)
   }
}

pub fn evaluate_advanturer() {
   let mut advanturer = Adventurer { name: "Billy", hit_points: 100_00 };
   advanturer.take_damage();
   println!("{}", advanturer);
}
