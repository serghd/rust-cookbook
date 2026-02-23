use std::rc::Rc;

struct City {
   name: String,
   population: u32,
   history: Rc<String>,
}

struct Cities {
   names: Vec<String>,
   histories: Vec<Rc<String>>,
}

pub fn evaluate_rc() {
   let city = City {
      name: "Toronto".to_string(),
      population: 100500,
      history: Rc::new("A few more words about the history of Toronto".to_string()),
   };

   let _cities = Cities { names: vec![city.name], histories: vec![city.history.clone()] };

   println!("History: {}", city.history);
   println!("Strong count: {}", Rc::strong_count(&city.history));
   let _next_owner = city.history.clone();
   println!("Strong count: {}", Rc::strong_count(&city.history));
}

////////////////////////////////////////////

pub fn evaluate_rc2() {
   let number_strong = Rc::new(42);
   let _number_weak = Rc::downgrade(&number_strong);

   println!("Strong {}, weak {}", Rc::strong_count(&number_strong), Rc::weak_count(&number_strong));

   let number_weak2 = Rc::downgrade(&number_strong);

   println!("Strong {}, weak {}", Rc::strong_count(&number_strong), Rc::weak_count(&number_strong));

   if let Some(number_strong2) = number_weak2.upgrade() {
      println!("Strong Ptr created successfully from number_weak2: {}", number_strong2);
   }

   println!("Strong {}, weak {}", Rc::strong_count(&number_strong), Rc::weak_count(&number_strong));

   drop(number_strong);

   if let Some(number_weak3) = number_weak2.upgrade() {
      println!("Strong Ptr created successfully from number_weak2: {}", number_weak3);
   } else {
      println!("Can't create a Strong Ptr from number_weak2, `number_strong` is dead");
   }
}
