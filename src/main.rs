#![allow(unused_imports)]

mod crates_and_modules;
mod external_crates;
mod samples;
mod standard_library;
mod tests;

use crate::samples::boxes::evaluate_errors;
use crate::samples::builder::use_builder;
use crate::samples::chaining_methods::make_chaining;
use crate::samples::channels::use_channels;
use crate::samples::closures::{
    evaluate_closures_1, evaluate_closures_2, evaluate_closures_3, evaluate_closures_4,
    evaluate_closures_5, evaluate_closures_6,
};
use crate::samples::collections::{binary_heap_demo, make_survey, vec_deque_demo};
use crate::samples::cow::{evaluate_cow, evaluate_cow_vector};
use crate::samples::deref::evaluate_deref;
use crate::samples::destructuring::City;
use crate::samples::enums::process_numbers;
use crate::samples::generics::process_objects;
use crate::samples::inspect::evaluate_inspect;
use crate::samples::interior_mutability::{use_cell, use_mutex, use_rw_lock};
use crate::samples::iterators::{evaluate_iterators_1, evaluate_iterators_2};
use crate::samples::lifetimes::{evaluate_advanturer, get_str, show_city_info};
use crate::samples::option;
use crate::samples::rc::{evaluate_rc, evaluate_rc2};
use crate::samples::threads::{
    call_concurrent_threads, call_thread, using_arc_variant1, using_arc_variant2,
};
use crate::samples::traits::{
    call_returned_closures, evaluate_trait, evaluate_trait_bounds, evaluate_trait_from,
    gives_higher_i32, print_objects_as_ref, print_string_as_bytes, print_with_impl_trait,
};
use crate::samples::tuples::{tuple_destructuring, tuple_print};
use crate::standard_library::vectors::evaluate_vector;

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
    // make_survey();
    // binary_heap_demo();
    // vec_deque_demo();

    // // #7. Traits
    // evaluate_trait();
    // evaluate_trait_bounds();
    // evaluate_trait_from();
    // print_string_as_bytes("abc");
    // print_objects_as_ref();
    // gives_higher_i32(8, 10);
    // print_with_impl_trait();
    // call_returned_closures();

    // // #8. Chaining methods
    // make_chaining();

    // #9. Iterators
    // evaluate_iterators_1();
    // evaluate_iterators_2();

    // #10. Closures
    // evaluate_closures_1();
    // evaluate_closures_2();
    // evaluate_closures_3();
    // evaluate_closures_4();
    // evaluate_closures_5();
    // evaluate_closures_6();

    // #11. Inspect
    //evaluate_inspect();

    // #12. Lifetimes
    // dbg!(get_str());
    // show_city_info();
    // evaluate_advanturer();

    // #13. Interior mutability
    // use_cell();
    // use_mutex();
    // use_rw_lock();

    // #14. Cow
    // evaluate_cow();
    // evaluate_cow_vector();

    // #15. Reference Counter
    // evaluate_rc();
    // evaluate_rc2();

    // #16. Threads
    // call_thread();
    // call_concurrent_threads();
    // using_arc_variant1();
    // using_arc_variant2();

    // #17. Channels
    // use_channels();

    // #18. Box
    // evaluate_errors();

    // #19. Builder
    // use_builder();

    // #20. Deref
    // evaluate_deref();

    // // #21. Crates and modules
    // use crates_and_modules::print_things;
    // print_things::prints_one_thing("One thing...");
    // use crates_and_modules::country;
    // country::province::city::print_city("Canada", "New Brunswick", "Moncton");
    // // or just:
    // crates_and_modules::country::province::city::print_city("Canada", "New Brunswick", "Moncton");

    // #22. External crates
    // rand
    // external_crates::random::display_characters();
    // rayon
    // external_crates::rayon::evaluate_rayon_vec();

    // #23. Standard library
    // standard_library:: arrays::print_cities();
    // standard_library::chars::eval_chars();
    // standard_library::integers::print_countries_info();
    // standard_library::floats::evaluate_floats();
    // standard_library::booleans::evaluate_booleans();
    // evaluate_vector();
    // standard_library::strings::string_capacity();
    // standard_library::strings::string_popping();
    standard_library::strings::string_retain();


    ////////////////////////////////////////////
}
