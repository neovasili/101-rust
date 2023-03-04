import * as utils from '../utils/utils';

// References
export function references() {
  utils.print_h3('References');

  let x = 10;
  let ref_x = x;
  ref_x = 20;
  console.log(`x: ${x}`);
  console.log(`ref_x: ${ref_x}`);
  console.log('NOTE: references or pointers do not exists in Typescript');
}
