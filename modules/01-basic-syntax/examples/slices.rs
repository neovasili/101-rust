// Slices
fn main() {
  // Array slices
  println!("------------------------\n");

  println!("Array slices\n");

  let my_ints_array: [i32; 6] = [10, 20, 30, 40, 50, 60];
  println!("my_ints_array: {my_ints_array:?}");

  let my_ints_slice: &[i32] = &my_ints_array[2..4];  // We can create slices of arrays using references
  println!("my_ints_slice: {my_ints_slice:?}");

  // String vs &str
  println!("------------------------\n");
  println!("String vs &str");

  let first: &str = "World";
  println!("first: {first}");

  let mut second: String = String::from("Hello ");
  println!("second: {second}");
  second.push_str(first);
  println!("second: {second}");

  let third: &str = &second[6..];  // We can also create slices of strings
  println!("third: {third}");
}
