mod samples;

use crate::samples::chaning_methods::make_chaining;
use crate::samples::closures::{
    evaluate_closures_1, evaluate_closures_2, evaluate_closures_3, evaluate_closures_4,
    evaluate_closures_5,
};
use crate::samples::collections::{binary_heap_demo, make_survey, vec_deque_demo};
use crate::samples::destructuring::City;
use crate::samples::enums::process_numbers;
use crate::samples::generics::process_objects;
use crate::samples::inspect::evaluate_inspect;
use crate::samples::iterators::{evaluate_iterators_1, evaluate_iterators_2};
use crate::samples::option;
use crate::samples::traits::{
    evaluate_trait, evaluate_trait_bounds, evaluate_trait_from, print_objects_as_ref,
    print_string_as_bytes,
};
use crate::samples::tuples::{tuple_destructuring, tuple_print};

fn main() {
    // // #1. Option
    // option::create_and_display_object();
    // option::display_weather();

    // // #2. Tuple Operations
    // tuple_print();
    // tuple_destructuring();

    // #3. Enums
    //process_numbers();

    // // #4. Destructuring
    // let city = City::create("City New".to_string(), "City Old".to_string(), 5000, 1219);
    // city.process_city();

    // // #5. Generics
    // process_objects();

    // // #6. Collections
    //make_survey();
    //binary_heap_demo();
    //vec_deque_demo();

    // // #7. Traits
    // evaluate_trait();
    // evaluate_trait_bounds();
    // evaluate_trait_from();
    //print_string_as_bytes("abc");
    // print_objects_as_ref();

    // // #8. Chaining methods
    // make_chaining();

    // #9. Iterators
    // evaluate_iterators_1();
    // evaluate_iterators_2();

    // #10. Closures
    // evaluate_closures_1();
    // evaluate_closures_2();
    //evaluate_closures_3();
    //evaluate_closures_4();
    //evaluate_closures_5();
    evaluate_inspect();

    ////////////////////////////////////////////
}
