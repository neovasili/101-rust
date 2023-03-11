// Move semantics
fn main() {
  println!("---------------------\n");

  let first: String = String::from("Hello!");
  // An assignment will transfer ownership between variables
  let second: String = first;
  println!("second: {second}");
  // Now `first` value is not accessible
  // println!("first: {first}");
}
