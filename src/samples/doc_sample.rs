/// This is a struct that does nothing
struct _DoesNothing {}

/// This struct only has one method.
struct PrintThing {}

/// It just prints the same message.
impl PrintThing {
    pub fn prints_something() {
        println!("I am printing something");
    }
}

pub fn evaluate_docs() {
    PrintThing::prints_something();
}