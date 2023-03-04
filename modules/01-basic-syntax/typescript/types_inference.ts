function takes_u32(x: number) {
  console.log(`u32: ${x}`);
}

function takes_i8(y: number) {
  console.log(`i8: ${y}`);
}

// Types inference
console.log("------------------------\n");

let x = 10;
let y = 20;

takes_u32(x);
takes_i8(y);
takes_u32(y);
