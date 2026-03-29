macro_rules! six_or_print {
    (6) => {
        6
    };
    () => {
        println!("You didn't give me 6.");
    };
}

pub fn macro_six_or_print() {
    let var = six_or_print!(6);
    println!("var: {}", var);
    six_or_print!();
}

/////////////////////////////////////////

macro_rules! might_print {
    ($input: expr) => {
        println!("You game me: {:?}", $input)
    };
}

pub fn macro_might_print() {
    might_print!(10);
    might_print!("abc");
    might_print!(vec![1, 2, 3]);
}

/////////////////////////////////////////

macro_rules! check_if_equal {
    ($input1: ident, $input2: expr) => {
        println!("Is {:?} is equal to {:?}? {:?}", $input1, $input2, $input1 == $input2);
    };
}

pub fn macro_check_if_equal() {
    let a = 10;
    let my_vec = vec![1, 2, 3];

    check_if_equal!(a, 10);
    check_if_equal!(my_vec, vec![4, 1, 2]);
}

/////////////////////////////////////////

macro_rules! print_anything {
    ($input: tt) => {
        let k = stringify!($input);
        println!("{}", k);
    };
}

pub fn macro_print_anything() {
    print_anything!(srwersdfsfwer);
    print_anything!(324234abc);
}

/////////////////////////////////////////

macro_rules! print_anything2 {
    ($($input: tt),*) => {
        let k = stringify!($($input),*);
        println!("{}", k);
    };
}

pub fn macro_print_anything2() {
    print_anything2!();
    print_anything2!(sdfsaa, 009sdfdsf);
}

/////////////////////////////////////////

macro_rules! make_a_function {
    ($name: ident, $($input: tt),*) => {
        fn $name() {
            let output = stringify!($($input),*);
            println!("{}", output);
        }
    };
}

pub fn macro_make_a_function() {
    make_a_function!(my_func, abcd, 0007);
    my_func();
}
