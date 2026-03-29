pub fn make_chaining() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let v_new = v.into_iter().skip(2).take(9).collect::<Vec<i32>>();
    println!("{:?}", v_new);
}
