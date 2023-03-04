from simple_chalk import chalk

def print_h1(title: str) -> None:
  print(chalk.blue(f"\n{title}"))
  print_br()

def print_h2(title: str) -> None:
  print(chalk.magenta(f"\n{title}"))

def print_h3(title: str) -> None:
  print(chalk.yellow(f"\n\n{title}"))

def print_br() -> None:
  print("------------")
