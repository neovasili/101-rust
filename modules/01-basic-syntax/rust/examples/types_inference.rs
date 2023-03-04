fn takes_u32(x: u32) {
  println!("u32: {x}");
}

fn takes_i8(y: i8) {
  println!("i8: {y}");
}

// Types inference
fn main() {
  println!("------------------------\n");

  let x = 10;
  let y = 20;

  takes_u32(x);
  takes_i8(y);
  // takes_u32(y);
}
