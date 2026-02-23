use std::fmt::{Debug, Display};

#[derive(Debug)]
struct MyGen {
   name: String,
   value: u32,
}

fn print_object<T: Debug>(obj: T) -> T {
   println!("{:?}", obj);
   obj
}

fn print_result<T, D>(obj1: T, obj2: D, obj3: D)
where
   T: Display,
   D: Display + PartialOrd,
{
   println!("{}. Comparison result: {}", obj1, obj2 > obj3);
}

pub fn process_objects() {
   let obj: MyGen = MyGen { name: "title1".to_string(), value: 10 };
   let obj2: u32 = 20;

   print_object(obj);
   print_object(obj2);

   let str1: String = "here is the text".to_string();
   let num1: u32 = 10;
   let num2: u32 = 20;
   print_result(str1, num1, num2);
}
