use std::fs;
use std::io::Read;
// We can add context with `anyhow` https://docs.rs/anyhow/latest/anyhow/
use anyhow::{Context, Result, bail};

fn read_username(path: &str) -> Result<String> {
  let mut username = String::with_capacity(100);

  fs::File::open(path)
    // We handle first error here adding this message to the error
    .context(format!("Failed to open {path}"))?
    .read_to_string(&mut username)
    // And now we handle the error for `read_to_string` adding new context
    .context("Failed to read username")?;

  if username.is_empty() {
    // This is equivalent to `return Err(MyError("My error message").into());`
    bail!("Found no username in {path}");
  }
  Ok(username)
}

// Errors context
fn main() {
  println!("---------------------\n");

  // This will trigger a different error, try it out!
  // fs::write("config.dat", "").unwrap();
  match read_username("config.dat") {
    Ok(username) => println!("Username: {username}"),
    Err(error)   => println!("Error: {error:?}"),
  }
}
