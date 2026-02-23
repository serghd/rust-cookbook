use std::cell::Cell;
use std::mem;
use std::sync::{Mutex, RwLock};

#[derive(Debug)]
struct Phone<'a> {
   name: &'a str,
   date_issued: u32,
   on_sale: Cell<bool>,
}

pub fn use_cell() {
   let phone = Phone { name: "Samsung", date_issued: 10500, on_sale: Cell::new(true) };
   phone.on_sale.set(false);
   println!("phone {:?}", phone);
}

pub fn use_mutex() {
   let my_mutex = Mutex::new(5);
   println!("first attempt: {:?}", my_mutex);
   let mut my_val_guard = my_mutex.lock().unwrap();
   *my_val_guard = 10;
   println!("second attempt: {:?}", my_mutex);
   mem::drop(my_val_guard);
   println!("third  attempt: {:?}", my_mutex);
}

pub fn use_rw_lock() {
   let my_val = RwLock::new(10);
   let a = my_val.read().unwrap();
   let b = my_val.read().unwrap();
   println!("a = {:?}", a);
   println!("b = {:?}", b);

   mem::drop(a);
   mem::drop(b);

   if let Ok(mut val) = my_val.try_write() {
      *val = 20;
   } else {
      println!("Couldn't get write access!")
   }
   println!("my_val = {:?}", my_val.read().unwrap());
}
