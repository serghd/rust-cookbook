#![allow(dead_code)]

pub fn tuple_print() {
   let random_tuple = ("Here is a string", 8, vec!['a'], 'a', [8, 9, 10], 7.7);

   println!(
      "Inside the tuple is:
First item: {:?}
Second item: {:?}
Third item: {:?}
Fourth item: {:?}
Fifth item: {:?}
Sixth item: {:?}",
      random_tuple.0,
      random_tuple.1,
      random_tuple.2,
      random_tuple.3,
      random_tuple.4,
      random_tuple.5,
   )
}

pub fn tuple_destructuring() {
   let arr = ["one", "two", "three"];
   let (_, _, variable) = (arr[0], arr[1], arr[2]);
   println!("Destructured variable: {}", variable);
}
