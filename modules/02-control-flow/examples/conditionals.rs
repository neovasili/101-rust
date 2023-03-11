// Conditionals
fn main() {
  println!("---------------------\n");

  let mut my_variable = 10;
  if my_variable % 2 == 0 {  // If statements work like in most languages
    my_variable = my_variable / 2;
  } else {
    my_variable = 3 * my_variable + 1;
  }
  println!("my_variable: {my_variable}");

  println!("---------------------\n");
  let mut my_variable_2 = 10;
  my_variable_2 = if my_variable_2 % 2 == 0 {  // We can use also if statements as expressions
    my_variable_2 / 2
  } else {
    3 * my_variable_2 + 1
  };
  println!("my_variable_2: {my_variable_2}");

  println!("---------------------\n");
  let argument = std::env::args().next();
  if let Some(value) = argument {  // We can use `if let` for single case pattern matching
    println!("Program name: {value}");
  } else {
    println!("Missing name?");
  }
}
