// References
fn main() {
    println!("------------------------\n");

    let mut my_variable: i32 = 10;  // Variables are inmutable by default
    // We can create a reference to an existing variable (mutability must match)
    let ref_my_variable: &mut i32 = &mut my_variable;
    *ref_my_variable = 20;

    println!("my_variable: {my_variable}");
}
