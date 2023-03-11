// Loops
fn main() {
  println!("---------------------\n");
  let vector = vec![10, 20, 30];
  let mut iter = vector.into_iter();

  // We can manually define lifetimes to break loops at diferent levels
  'outer: while let Some(value) = iter.next() {
    println!("value: {value}");
    let mut index = 0;
    while index < value {
      println!("value: {value}, index: {index}");
      index += 1;
      if index == 3 {
        break 'outer;
      }
    }
  }
}
