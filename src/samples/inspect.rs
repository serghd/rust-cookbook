pub fn evaluate_inspect() {
   let v = vec![1, 2, 3];
   let v_double = v
      .iter()
      .inspect(|item| {
         match **item % 2 {
            0 => println!("{} is even", **item),
            _ => println!("{} is odd", **item),
         }
         println!("In binary it is {:b}", item);
      })
      .map(|x| x * 2)
      .collect::<Vec<i32>>();
   dbg!(v_double);
}
