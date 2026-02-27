use std::sync::{Arc, Mutex};

pub fn call_thread() {
   let str = "My string".to_string();
   let handle = std::thread::spawn(move || {
      println!("{}", str);
   });
   handle.join().unwrap();
}

////////////////////////////////////////////

pub fn call_concurrent_threads() {
   let handle1 = std::thread::spawn(|| {
      for _ in 0..5 {
         println!("Thread 1 is working");
      }
   });

   let handle2 = std::thread::spawn(|| {
      for _ in 0..5 {
         println!("Thread 2 is working");
      }
   });

   handle1.join().unwrap();
   handle2.join().unwrap();
}

////////////////////////////////////////////

pub fn using_arc_variant1() {
   let number = Arc::new(Mutex::new(0));

   let number1 = Arc::clone(&number);
   let number2 = Arc::clone(&number);

   let handle1 = std::thread::spawn(move || {
      for _ in 0..10 {
         *number1.lock().unwrap() += 1;
      }
   });

   let handle2 = std::thread::spawn(move || {
      for _ in 0..10 {
         *number2.lock().unwrap() += 1;
      }
   });

   handle1.join().unwrap();
   handle2.join().unwrap();

   println!("number: {:?}", number);
}

////////////////////////////////////////////

pub fn using_arc_variant2() {
   let number = Arc::new(Mutex::new(0));
   let mut handle_vec = vec![];

   for _ in 0..2 {
      let number1 = Arc::clone(&number);
      let handle = std::thread::spawn(move || {
         for _ in 0..10 {
            *number1.lock().unwrap() += 1;
         }
      });
      handle_vec.push(handle);
   }

   handle_vec.into_iter().for_each(|item| item.join().unwrap());
   println!("number: {:?}", number);
}
