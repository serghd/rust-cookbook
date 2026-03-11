#![allow(dead_code)]

fn return_two() -> u32 {
   2
}

fn return_six() -> u32 {
   return_two() + 4
}

#[cfg(test)]
mod tests {
   use super::*;

   fn it_returns_six() {
      assert_eq!(return_six(), 6);
   }

   fn it_returns_two() {
      assert_eq!(return_two(), 2);
   }
}
