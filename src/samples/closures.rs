#![allow(dead_code)]

pub fn evaluate_closures_1() {
   let v = vec![1, 2, 3];
   v.iter().enumerate().for_each(|(index, value)| println!("index {}, value {}", index, value));
}

pub fn evaluate_closures_2() {
   let numbers_together = "140399923481800622623218009598281";
   for (index, symbol) in numbers_together.char_indices() {
      match index % 3 {
         (0..=1) => print!("{}", symbol),
         _ => print!("{}\t", symbol),
      }
   }
}

////////////////////////////////////////////

struct Company {
   name: String,
   ceo: Option<String>,
}

impl Company {
   pub fn new(company_name: &str, ceo: &str) -> Self {
      let ceo = match ceo {
         "" => None,
         _ => Some(ceo.to_string()),
      };
      Self { name: company_name.to_string(), ceo }
   }

   pub fn get_ceo(&self) -> Option<String> {
      self.ceo.clone()
   }
}

pub fn evaluate_closures_3() {
   let v = vec![
      Company::new("company 1", "CEO 1"),
      Company::new("company 2", ""),
      Company::new("company 3", "CEO 3"),
   ];

   let v_filtered = v.into_iter().filter_map(|company| company.get_ceo()).collect::<Vec<String>>();
   println!("{:?}", v_filtered);
}

pub fn evaluate_closures_4() {
   let v = vec![
      Company::new("company 1", "CEO 1"),
      Company::new("company 2", ""),
      Company::new("company 3", "CEO 3"),
   ];
   let mut res_v = vec![];
   v.iter().for_each(|company| {
      res_v.push(company.get_ceo().ok_or_else(|| {
         let str = format!("No CEO found for {}", company.name);
         str
      }))
   });
   println!("{:?}", res_v);
}

pub fn evaluate_closures_5() {
   let locations = vec![("Nevis", 25), ("Taber", 8428), ("Markerville", 45), ("Cardston", 3585)];

   let mut locations_peakable = locations.iter().peekable();
   while locations_peakable.peek().is_some() {
      match locations_peakable.peek() {
         Some((name, count)) if *count < 100 => {
            println!("{} is a village with population {}", name, count)
         }
         Some((name, count)) => println!("{} is a town with population {}", name, count),
         None => break,
      }
      locations_peakable.next();
   }
}

////////////////////////////////////////////

struct City {
   name: String,
   years: Vec<u32>,
   population: Vec<u32>,
}

impl City {
   fn new(name: &str, years: Vec<u32>, population: Vec<u32>) -> Self {
      Self { name: name.to_string(), years, population }
   }

   fn city_data<F>(&mut self, mut f: F)
   where
      F: FnMut(&mut Vec<u32>, &mut Vec<u32>),
   {
      f(&mut self.years, &mut self.population);
   }
}

pub fn evaluate_closures_6() {
   let years = vec![1372, 1834, 1851, 1881, 1897, 1925, 1959, 1989, 2000, 2005, 2010, 2020];
   let populations = vec![
      3_250, 15_300, 24_000, 45_900, 58_800, 119_800, 283_071, 478_974, 400_378, 401_694, 406_703,
      437_619,
   ];

   let mut tallinn = City::new("Tallinn", years, populations);

   // collect data
   tallinn.city_data(|city_years, city_populations| {
      let new_city_data =
         city_years.into_iter().zip(city_populations.into_iter()).take(5).collect::<Vec<(_, _)>>();
      println!("New city data: {:?}", new_city_data);
   });

   // add data
   tallinn.city_data(|x, y| {
      x.push(2030);
      y.push(500_000);
   });

   // remove data
   tallinn.city_data(|city_years, city_populations| {
      let position_option = city_years.iter().position(|x| *x == 1834);
      if let Some(position) = position_option {
         println!("Now we're removing {} at position {}", city_years[position], position);
         city_years.remove(position);
         city_populations.remove(position);
         println!("Modified city years {:?} and population: {:?}", city_years, city_populations);
      }
   });
}
