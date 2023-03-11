// Lifetime
#[derive(Debug)]
struct Point(i32, i32);

// Returning a borrowed value requires specifing a lifetime
fn left_most<'lifetime>(point1: &'lifetime Point, point2: &'lifetime Point) -> &'lifetime Point {
  return if point1.0 < point2.0 { point1 } else { point2 }
}

// If data structure contains borrowed value, we need to specify a lifetime
#[derive(Debug)]
struct Highlight<'other_lifetime>(&'other_lifetime str);

fn erase(text: String) {
    println!("Bye {text}!");
}

fn main() {
  println!("---------------------\n");

  let point1: Point = Point(10, 10);
  let point2: Point = Point(20, 20);
  let point3: &Point = left_most(&point1, &point2);
  println!("left-most point: {:?}", point3);

  println!("---------------------\n");
  let text = String::from("The quick brown fox jumps over the lazy dog.");
  let fox = Highlight(&text[4..19]);
  let dog = Highlight(&text[35..43]);
  // erase(text);  // If we transfer ownership of `text` then borrowed values are wiped out
  println!("{fox:?}");
  println!("{dog:?}");
}
