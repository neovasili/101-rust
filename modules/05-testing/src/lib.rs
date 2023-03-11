/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `testing` crate:
///
/// ```
/// let result = testing::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(number_1: i32, number_2: i32) -> i32 {
  return number_1 + number_2;
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
///
/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// let result = testing::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// testing::div(10, 0);
/// ```
pub fn div(number_1: i32, number_2: i32) -> i32 {
  if number_2 == 0 {
      panic!("Divide-by-zero error");
  }

  return number_1 / number_2;
}

// Here is one public method that will be used for integration testing
pub fn my_public_method() -> bool {
  println!("Example of public function");

  return true;
}
