// Move semantics
fn main() {
  println!("---------------------\n");

  let s1: String = String::from("Hello!");
  let s2: String = s1;
  println!("s2: {s2}");
  // println!("s1: {s1}");
}