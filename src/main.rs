mod crates_and_modules;
mod external_crates;
mod samples;
mod standard_library;
mod tests;

use crate::crates_and_modules::evaluate_crates_and_modules;
use crate::samples::args::evaluate_args;
use crate::samples::boxes::evaluate_errors;
use crate::samples::builder::use_builder;
use crate::samples::chaining_methods::make_chaining;
use crate::samples::channels::use_channels;
use crate::samples::closures::{
    evaluate_closures_1, evaluate_closures_2, evaluate_closures_3, evaluate_closures_4, evaluate_closures_5,
    evaluate_closures_6,
};
use crate::samples::collections::{binary_heap_demo, make_survey, vec_deque_demo};
use crate::samples::cow::{evaluate_cow, evaluate_cow_vector};
use crate::samples::deref::evaluate_deref;
use crate::samples::destructuring::destruct_city;
use crate::samples::enums::process_numbers;
use crate::samples::generics::process_objects;
use crate::samples::inspect::evaluate_inspect;
use crate::samples::interior_mutability::{use_cell, use_mutex, use_rw_lock};
use crate::samples::iterators::{evaluate_iterators_1, evaluate_iterators_2};
use crate::samples::lifetimes::{evaluate_advanturer, lt_get_str, show_city_info};
use crate::samples::macros::{
    macro_check_if_equal, macro_make_a_function, macro_might_print, macro_print_anything, macro_print_anything2,
    macro_six_or_print,
};
use crate::samples::option;
use crate::samples::rc::{evaluate_rc, evaluate_rc2};
use crate::samples::threads::{call_concurrent_threads, call_thread, using_arc_variant1, using_arc_variant2};
use crate::samples::traits::{
    call_returned_closures, evaluate_trait, evaluate_trait_bounds, evaluate_trait_from, print_objects_as_ref,
    print_with_impl_trait, traits_gives_higher_i32, traits_print_string_as_bytes,
};
use crate::samples::tuples::{tuple_destructuring, tuple_print};
use crate::samples::user_input::evaluate_user_input;
use crate::samples::vars::print_vars;
use crate::standard_library::vectors::evaluate_vector;
use std::collections::HashMap;

fn main() {
    let mut examples: HashMap<&str, fn()> = HashMap::new();

    // #1. Option
    examples.insert("option::create_and_display_object", option::create_and_display_object);
    examples.insert("option::display_weather", option::display_weather);

    // #2. Tuple Operations
    examples.insert("tuple_print", tuple_print);
    examples.insert("tuple_destructuring", tuple_destructuring);

    // #3. Enums
    examples.insert("process_numbers", process_numbers);

    // #4. Destructuring
    examples.insert("destruct_city", destruct_city);

    // #5. Generics
    examples.insert("process_objects", process_objects);

    // #6. Collections
    examples.insert("make_survey", make_survey);
    examples.insert("binary_heap_demo", binary_heap_demo);
    examples.insert("vec_deque_demo", vec_deque_demo);

    // #7. Traits
    examples.insert("evaluate_trait", evaluate_trait);
    examples.insert("evaluate_trait_bounds", evaluate_trait_bounds);
    examples.insert("evaluate_trait_from", evaluate_trait_from);
    examples.insert("traits_print_string_as_bytes", traits_print_string_as_bytes);
    examples.insert("print_objects_as_ref", print_objects_as_ref);
    examples.insert("traits_gives_higher_i32", traits_gives_higher_i32);
    examples.insert("print_with_impl_trait", print_with_impl_trait);
    examples.insert("call_returned_closures", call_returned_closures);

    // #8. Chaining methods
    examples.insert("make_chaining", make_chaining);

    // #9. Iterators
    examples.insert("evaluate_iterators_1", evaluate_iterators_1);
    examples.insert("evaluate_iterators_2", evaluate_iterators_2);

    // #10. Closures
    examples.insert("evaluate_closures_1", evaluate_closures_1);
    examples.insert("evaluate_closures_2", evaluate_closures_2);
    examples.insert("evaluate_closures_3", evaluate_closures_3);
    examples.insert("evaluate_closures_4", evaluate_closures_4);
    examples.insert("evaluate_closures_5", evaluate_closures_5);
    examples.insert("evaluate_closures_6", evaluate_closures_6);

    // #11. Inspect
    examples.insert("evaluate_inspect", evaluate_inspect);

    // #12. Lifetimes
    examples.insert("lt_get_str", lt_get_str);
    examples.insert("show_city_info", show_city_info);
    examples.insert("evaluate_advanturer", evaluate_advanturer);

    // #13. Interior mutability
    examples.insert("use_cell", use_cell);
    examples.insert("use_mutex", use_mutex);
    examples.insert("use_rw_lock", use_rw_lock);

    // #14. Cow
    examples.insert("evaluate_cow", evaluate_cow);
    examples.insert("evaluate_cow_vector", evaluate_cow_vector);

    // #15. Reference Counter
    examples.insert("evaluate_rc", evaluate_rc);
    examples.insert("evaluate_rc2", evaluate_rc2);

    // #16. Threads
    examples.insert("call_thread", call_thread);
    examples.insert("call_concurrent_threads", call_concurrent_threads);
    examples.insert("using_arc_variant1", using_arc_variant1);
    examples.insert("using_arc_variant2", using_arc_variant2);

    // #17. Channels
    examples.insert("use_channels", use_channels);

    // #18. Box
    examples.insert("evaluate_errors", evaluate_errors);

    // #19. Builder
    examples.insert("use_builder", use_builder);

    // #20. Deref
    examples.insert("evaluate_deref", evaluate_deref);

    // #21. Crates and modules
    examples.insert("evaluate_crates_and_modules", evaluate_crates_and_modules);

    // #22. External crates
    examples.insert("external_crates::random::display_characters", external_crates::random::display_characters);
    examples.insert("external_crates::rayon::evaluate_rayon_vec", external_crates::rayon::evaluate_rayon_vec);

    // #23. Standard library
    examples.insert("standard_library::arrays::print_cities", standard_library::arrays::print_cities);
    examples.insert("standard_library::chars::eval_chars", standard_library::chars::eval_chars);
    examples
        .insert("standard_library::integers::print_countries_info", standard_library::integers::print_countries_info);
    examples.insert("standard_library::floats::evaluate_floats", standard_library::floats::evaluate_floats);
    examples.insert("standard_library::booleans::evaluate_booleans", standard_library::booleans::evaluate_booleans);
    examples.insert("evaluate_vector", evaluate_vector);
    examples.insert("standard_library::strings::string_capacity", standard_library::strings::string_capacity);
    examples.insert("standard_library::strings::string_popping", standard_library::strings::string_popping);
    examples.insert("standard_library::strings::string_retain", standard_library::strings::string_retain);

    // #24. Macros
    examples.insert("macro_six_or_print", macro_six_or_print);
    examples.insert("macro_might_print", macro_might_print);
    examples.insert("macro_check_if_equal", macro_check_if_equal);
    examples.insert("macro_print_anything", macro_print_anything);
    examples.insert("macro_print_anything2", macro_print_anything2);
    examples.insert("macro_make_a_function", macro_make_a_function);

    // #25. User input & args & vars
    examples.insert("evaluate_user_input", evaluate_user_input);
    examples.insert("evaluate_args", evaluate_args);
    examples.insert("print_vars", print_vars);

    let sample_name = "print_vars";
    if let Some(sample) = examples.get(sample_name) {
        sample();
    } else {
        println!("No example found with name '{}'", sample_name);
    }
}
