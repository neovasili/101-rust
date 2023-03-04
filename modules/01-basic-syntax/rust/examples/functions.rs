// FizzBuzz functions
fn is_divisible_by(number: u32, divisible_by: u32) -> bool {
  if divisible_by == 0 {
      return false;
  }
  number % divisible_by == 0     // The last expression in a block is the return value
}

fn fizzbuzz(number: u32) -> () {
  match (is_divisible_by(number, 3), is_divisible_by(number, 5)) {
      (true,  true)  => println!("fizzbuzz"),
      (true,  false) => println!("fizz"),
      (false, true)  => println!("buzz"),
      (false, false) => println!("{number}"),
  }
}

fn fizzbuzz_to(number: u32) {
  for i in 1..=number {
      fizzbuzz(i);
  }
}

// Methods
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
      self.width * self.height
  }

  fn inc_width(&mut self, delta: u32) {
      self.width += delta;
  }
}

// Overloading
fn pick_one<T>(first_element: T, second_element: T) -> T {
  if std::process::id() % 2 == 0 { first_element } else { second_element }
}

fn main() {
  // Fizzbuzz
  println!("------------------------\n");
  
  println!("Fizzbuzz\n");
  fizzbuzz_to(10);

  // Methods
  println!("------------------------\n");
  println!("Methods");
  
  let mut rectangle = Rectangle { width: 10, height: 5 };
  println!("old area: {}", rectangle.area());
  rectangle.inc_width(5);
  println!("new area: {}", rectangle.area());
  
  // Overloading
  println!("------------------------\n");
  println!("Overloading");

  println!("coin toss: {}", pick_one("heads", "tails"));
  println!("cash prize: {}", pick_one(500, 1000));
}
