fn takes_u32(value: u32) {
  println!("u32: {value}");
}

fn takes_i8(value: i8) {
  println!("i8: {value}");
}

// Types inference
fn main() {
  println!("------------------------\n");

  let first = 10;
  let second = 20;

  // Compiler will determine that `first` is an `u32` because we are using it (first usage) as such
  takes_u32(first);
  takes_i8(second);
  // Since compiler already said `second` is a `i8`, we cannot use it as a `u32`
  // takes_u32(second);
}
