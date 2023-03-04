use crate::utils;

// Array assignment and access
pub fn array_asignment_and_access() {
    utils::print_h3("Array assignment and access");

    let mut my_ints_array: [i8; 10] = [42; 10];
    my_ints_array[5] = 0;
    println!("my_ints_array: {my_ints_array:?}");
}

// Tuple assignment and access
pub fn tuple_assignment_and_access() {
    utils::print_h3("Tuple assignment and access");

    let my_mixed_types_tuple: (i8, bool) = (7, true);
    println!("1st index: {}", my_mixed_types_tuple.0);
    println!("2nd index: {}", my_mixed_types_tuple.1);
}
