#[derive(PartialOrd, PartialEq)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let x = self.0;
        Centimeters(x as f64 * 2.54_f64)
    }
}

struct Seconds(i32);

pub fn evaluate_derive_attributes() {
    let _one_second = Seconds(1).0;
    // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
    // println!("One second looks like: {:?}", _one_second);
    // TODO ^ Try uncommenting this line

    // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
    // let _this_is_true = (_one_second ==_one_second);
    // TODO ^ Try uncommenting this line

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter { "smaller" } else { "bigger" };
    println!("One foot is {:?} than one meter", cmp);
}
