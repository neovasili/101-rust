import * as utils from '../utils/utils';

// Array slices
export function array_slices() {
  utils.print_h3('Array slices');

  const my_ints_array = [10, 20, 30, 40, 50, 60];
  console.log(`my_ints_array: ${my_ints_array}`);

  const my_ints_slice = my_ints_array.slice(2,4);
  console.log(`my_ints_slice: ${my_ints_slice}`);
}

// String vs &str
export function string_vs_str() {
  utils.print_h3('String vs &str');

  const s1 = 'World';
  console.log(`s1: ${s1}`);

  let s2 = 'Hello ';
  console.log(`s2: ${s2}`);
  s2 += s1;
  console.log(`s2: ${s2}`);

  const s3 = s2.slice(6);
  console.log(`s3: ${s3}`);
  console.log('NOTE: Python does not have different types of string');
}
