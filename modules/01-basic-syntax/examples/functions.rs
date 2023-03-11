// FizzBuzz functions
fn is_divisible_by(number: u32, divisible_by: u32) -> bool {
  if divisible_by == 0 {
    return false;
  }
  number % divisible_by == 0  // The last expression in a block is the return value
}

fn fizzbuzz(number: u32) -> () {
  match (is_divisible_by(number, 3), is_divisible_by(number, 5)) {  // This is "similar" to switch statement
    (true,  true)  => println!("fizzbuzz"),
    (true,  false) => println!("fizz"),
    (false, true)  => println!("buzz"),
    (_, _) => println!("{number}"),  // Default case
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

  // With `&mut self` we are saying that we need "write access" to the object
  fn inc_width(&mut self, delta: u32) {
      self.width += delta;
  }
}

// Overloading
fn generate_random_number() -> i32 {
  return 4;  // Chosen by fair dice roll. Guaranteed to be random.
}

// Using generics
fn pick_one<T>(first_element: T, second_element: T) -> T {
  if generate_random_number() % 2 == 0 {
    return first_element;
  } else {
    return second_element;
  }
}

#[derive(Debug)]
enum CoinFlip {  // We can define enum values
  Heads,
  Tails,
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

  println!("coin toss: {:?}", pick_one(CoinFlip::Heads, CoinFlip::Tails));
  println!("cash prize: {}", pick_one(500, 1000));
}
