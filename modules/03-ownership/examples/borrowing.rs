// Borrowing
#[derive(Debug)]  // Now we don't need the Copy or Clone traits (and is more efficient)
struct Point(i32, i32);

fn add(point1: &Point, point2: &Point) -> Point {  // We can borrow values using references
  return Point(point1.0 + point2.0, point1.1 + point2.1)
}

fn main() {
  println!("---------------------\n");

  let point1 = Point(3, 4);
  let point2 = Point(10, 20);
  // The function borrow values from `point1` and `point2` and creates a new Point
  let point3 = add(&point1, &point2);
  println!("{point1:?} + {point2:?} = {point3:?}");
}
