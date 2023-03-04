import chalk from 'chalk';

export function print_h1(title: string) {
  console.log(`\n${chalk.blue(title)}`);
  print_br();
}

export function print_h2(title: string) {
  console.log(`\n${chalk.magenta(title)}`);
}

export function print_h3(title: string) {
  console.log(`\n\n${chalk.yellow(title)}`);
}

export function print_br() {
  console.log('------------');
}
