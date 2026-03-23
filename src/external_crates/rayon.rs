use rayon::prelude::*;

pub fn evaluate_rayon_vec() {
    let mut my_vec: Vec<u32> = vec![0; 200_000];
    my_vec.par_iter_mut().enumerate().for_each(|(index, number)| *number = index as u32 + 1);
    println!("vector: {:?}", &my_vec[5000..5005]);
}
