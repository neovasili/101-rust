fn first_word(text: &str) -> &str {
  match text.find(' ') {
    Some(index) => &text[..index],
    None => &text,
  }
}

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

// Unit tests
fn main() {
  first_word("Hello");
  println!("Hello world!");
}
