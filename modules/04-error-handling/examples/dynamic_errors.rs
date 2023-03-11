// Using self here is like `use std::fs;`
use std::fs::{self, File};
use std::io::Read;
use thiserror::Error;
use std::error::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Found no username in {0}")]
struct EmptyUsernameError(String);

/* We can use `Box<dyn Error>` to return a "generic error";
with that we are saying to the compiler:

"Here is a pointer to a dynamically sized Error" */
fn read_username(path: &str) -> Result<String, Box<dyn Error>> {
  let mut username = String::with_capacity(100);
  File::open(path)?.read_to_string(&mut username)?;

  if username.is_empty() {
    return Err(EmptyUsernameError(String::from(path)).into());
  }
  return Ok(username);
}

// Dynamic errors
fn main() {
  println!("---------------------\n");

  // This will trigger a different error, try it out!
  // fs::write("config.dat", "").unwrap();
  match read_username("config.dat") {
    Ok(username) => println!("Username: {username}"),
    Err(error)   => println!("Error: {error}"),
  }
}
