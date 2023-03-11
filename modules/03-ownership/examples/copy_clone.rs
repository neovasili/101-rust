// Copy & clone
#[derive(Copy, Clone, Debug)]  // This directive will create some code automatically for us
struct Point(i32, i32);

fn main() {
  println!("---------------------\n");

  let first = 42;
  let second = first;  // While move semantics are the default, certain types are copied by default
  println!("first: {first}");
  println!("second: {second}");

  let point1 = Point(3, 4);
  let point2 = point1;  // As `Point` have the Copy trait, it's value is copied
  println!("point1: {point1:?}");  // And point1 is still accessible
  println!("point2: {point2:?}");
}
