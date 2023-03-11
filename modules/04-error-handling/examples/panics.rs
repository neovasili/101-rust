// Panics
fn main() {
  println!("---------------------\n");

  let vector = vec![10, 20, 30];
  // This will cause a `panic` since it's a potentially unrecoverable runtime error
  println!("v[100]: {}", vector[100]);
}
