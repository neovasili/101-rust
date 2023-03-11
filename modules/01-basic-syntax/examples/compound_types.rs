// Compound types
fn main() {
    println!("------------------------\n");

    // Array assignment and access
    println!("Array assignment and access\n");

    let mut my_ints_array: [i8; 10] = [42; 10];  // We can initialize arrays to a value
    my_ints_array[5] = 0;
    println!("my_ints_array: {my_ints_array:?}");  // We add `:?` because is a complex type

    println!("------------------------\n");

    // Tuple assignment and access
    println!("Tuple assignment and access\n");

    let my_mixed_types_tuple: (i8, bool) = (7, true);  // We can create tuples with different types
    println!("1st index: {}", my_mixed_types_tuple.0);
    println!("2nd index: {}", my_mixed_types_tuple.1);
}
