pub fn print_vars() {
    for item in std::env::vars() {
        println!("{:?}", item);
    }
}
