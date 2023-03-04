// Scopes and shadowing
console.log('------------------------\n');

let a = 10;
console.log(`before: ${a}`);

{
  let a = `hello`;
  console.log(`inner scope: ${a}`);

  let a = true;
  console.log(`shadowed in inner scope: ${a}`);
}

console.log(`after: ${a}`);
