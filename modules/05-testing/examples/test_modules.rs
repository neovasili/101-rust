fn first_word(text: &str) -> &str {
  match text.find(' ') {
    Some(index) => &text[..index],
    None => &text,
  }
}

#[cfg(test)]
// This is essentially for tests grouping purposes
mod tests_module {
  use super::*;

  #[test]
  fn test_empty() {
    assert_eq!(first_word(""), "");
  }

  #[test]
  fn test_single_word() {
    assert_eq!(first_word("Hello"), "Hello");
  }

  #[test]
  fn test_multiple_words() {
    assert_eq!(first_word("Hello World"), "Hello");
  }
}

// Tests modules
pub fn main() {
  println!("{}", first_word("Hello World"));
}
