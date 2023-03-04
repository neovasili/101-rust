use crate::utils;

// Array slices
pub fn array_slices() {
  utils::print_h3("Array slices");

  let my_ints_array: [i32; 6] = [10, 20, 30, 40, 50, 60];
  println!("my_ints_array: {my_ints_array:?}");

  let my_ints_slice: &[i32] = &my_ints_array[2..4];
  println!("my_ints_slice: {my_ints_slice:?}");
}

// String vs &str
pub fn string_vs_str() {
  utils::print_h3("String vs &str");

  let s1: &str = "World";
  println!("s1: {s1}");

  let mut s2: String = String::from("Hello ");
  println!("s2: {s2}");
  s2.push_str(s1);
  println!("s2: {s2}");
  
  let s3: &str = &s2[6..];
  println!("s3: {s3}");
}
