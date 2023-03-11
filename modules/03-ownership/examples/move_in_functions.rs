// Move in functions
fn say_hello(name: String) {
  println!("Hello {name}")
}

fn main() {
  println!("---------------------\n");

  let name = String::from("Alice");
  // This transfer ownership of `name` to the function parameter
  say_hello(name);
  // Now `name` cannot be accessed, since it's freed when function ends
  // say_hello(name);
}
