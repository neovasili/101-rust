console.log("------------------------\n");
// Array assignment and access
console.log('Array assignment and access');

let my_ints_array: Array<number> = [];
for (let i=0; i<10; i++) {
  my_ints_array.push(42);
}
my_ints_array[5] = 0;
console.log(`my_ints_array: ${my_ints_array}`);

console.log("------------------------\n");
// Tuple assignment and access
console.log('Tuple assignment and access');

const my_mixed_types_tuple = [7, true];
console.log(`1st index: ${my_mixed_types_tuple[0]}`);
console.log(`2nd index: ${my_mixed_types_tuple[1]}`);
