// Blocks
fn main() {
  println!("---------------------\n");

  let root_block = {  // Blocks have a type and value in Rust
    let first = 10;
    println!("first: {first}");
    let second = {
      let third = {
        3 + 4
      };
      println!("third: {third}");
      first * third
    };
    println!("second: {second}");
    second - first
  };
  println!("root_block: {root_block}");
}
