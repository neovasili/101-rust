use std::{fs, io};
use std::io::Read;
// Using https://docs.rs/thiserror/ crate we can "inject" easily more error cases
use thiserror::Error;

// Here we are automatically implementing `Error` for the `ReadUsernameError` enum
#[derive(Debug, Error)]
enum ReadUsernameError {
  #[error("Could not read: {0}")]
  // And then we explicity specify different types of errors here
  IoError(#[from] io::Error),
  #[error("Found no username in {0}")]
  EmptyUsername(String),  // And here
}

// Then our new enum can be used to return multiple error types
fn read_username(path: &str) -> Result<String, ReadUsernameError> {
  let mut username = String::with_capacity(100);
  fs::File::open(path)?.read_to_string(&mut username)?;

  if username.is_empty() {
    // We can even explicity throw the specific error
    return Err(ReadUsernameError::EmptyUsername(String::from(path)));
  }
  return Ok(username);
}

// Errors deriving
fn main() {
  println!("---------------------\n");

  // This will trigger a different error, try it out!
  //fs::write("config.dat", "").unwrap();
  match read_username("config.dat") {
    Ok(username) => println!("Username: {username}"),
    Err(error)   => println!("Error: {error}"),
  }
}
