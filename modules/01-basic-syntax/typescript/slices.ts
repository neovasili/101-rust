console.log("------------------------\n");
// Array slices
console.log('Array slices');

const my_ints_array = [10, 20, 30, 40, 50, 60];
console.log(`my_ints_array: ${my_ints_array}`);

const my_ints_slice = my_ints_array.slice(2,4);
console.log(`my_ints_slice: ${my_ints_slice}`);

console.log("------------------------\n");
// String vs &str
console.log('String vs &str');

const s1 = 'World';
console.log(`s1: ${s1}`);

let s2 = 'Hello ';
console.log(`s2: ${s2}`);
s2 += s1;
console.log(`s2: ${s2}`);

const s3 = s2.slice(6);
console.log(`s3: ${s3}`);
console.log('NOTE: Python does not have different types of string');
