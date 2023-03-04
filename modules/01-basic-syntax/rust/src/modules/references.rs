use crate::utils;

// References
pub fn references() {
    utils::print_h3("References");

    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");
}
