use std::fs::File;
use std::io::Read;

// Structured errors
fn main() {
  println!("---------------------\n");

  let file = File::open("diary.txt");

  match file {  // File::open returns a `Result` that can contain an error, so we handle it with match
    Ok(mut file) => {
      let mut contents = String::new();
      let result = file.read_to_string(&mut contents);

      match result {  // Same here
        Ok(_) => println!("Dear diary: {contents}"),
        Err(error) => println!("The diary could not be opened: {error}"),
      }
    },
    Err(error) => println!("The diary could not be opened: {error}"),
  }
}
