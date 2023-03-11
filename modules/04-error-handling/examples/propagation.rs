use std::fs;
use std::io::{self, Read};

/* The try-operator ? is used to return errors to the caller.
It lets you turn the common:

  match some_expression {
    Ok(value) => value,
    Err(error) => return Err(error),
  }

into the much simpler

  some_expression? */
fn read_username(path: &str) -> Result<String, io::Error> {
  let username_file_result = fs::File::open(path);

  // We can use the `?` to propagate the Result-Error pattern
  let mut username_file = username_file_result?;

  let mut username = String::new();

  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(error) => Err(error),
  }
}

// Errors propagation
fn main() {
  println!("---------------------\n");

  // This will trigger a different error, try it out!
  //fs::write("config.dat", "").unwrap();
  let username = read_username("config.dat");
  println!("username or error: {username:?}");
}
