import * as utils from '../utils/utils';
import process from 'process';

// FizzBuzz functions
function is_divisible_by(number: number, divisible_by: number): boolean {
  if (divisible_by === 0) {
    return false;
  }

  return number % divisible_by === 0;
}

function fizzbuzz(number: number): void {
  const divisible_by_3 = is_divisible_by(number, 3);
  const divisible_by_5 = is_divisible_by(number, 5);

  if (divisible_by_3 && divisible_by_5) console.log('fizzbuzz');
  if (divisible_by_3 && !divisible_by_5) console.log('fizz');
  if (!divisible_by_3 && divisible_by_5) console.log('buzz');
  if (!divisible_by_3 && !divisible_by_5) console.log(number.toString());
}

export function fizzbuzz_to(number: number): void {
  utils.print_h3('FizzBuzz');

  for (let index=1; index <=number; index++) {
    fizzbuzz(index);
  }
}

// Methods
class Rectangle {
  private width: number;
  private readonly height: number;

  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
  }

  public area(): number {
    return this.width * this.height;
  }

  public inc_width(delta: number): void {
    this.width += delta;
  }
}

export function test_methods_from_rectangle() {
  utils.print_h3('Methods');

  const rectangle = new Rectangle(10, 5);
  console.log(`old area: ${rectangle.area()}`);
  rectangle.inc_width(5);
  console.log(`new area: ${rectangle.area()}`);
}

function pick_one(fisrt_element: any, second_element: any) {
  if (process.pid % 2 === 0) return fisrt_element; else return second_element;
}

export function test_overloading() {
  utils.print_h3('Overloading');

  console.log(`coin toss: ${pick_one('heads', 'tails')}`);
  console.log(`cash price: ${pick_one(500, 1000)}`);
}
