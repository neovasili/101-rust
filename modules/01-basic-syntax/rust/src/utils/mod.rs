use colored::Colorize;

pub fn print_h1(title: &str) -> () {
  println!("\n{}", title.bright_blue());
  print_br();
}

pub fn print_h2(title: &str) -> () {
  println!("{}\n", title.bright_purple());
}

pub fn print_h3(title: &str) -> () {
  println!("\n{}", title.bright_yellow());
}

pub fn print_br() -> () {
  println!("------------\n");
}
