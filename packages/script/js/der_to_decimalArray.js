const fs = require('fs');
const { join } = require('path');

const INPUT_PATH = join(__dirname, '../../certs/der.txt');
const OUTPUT_PATH = join(__dirname, '../../certs/decimalArray.json');

const der = fs.readFileSync(INPUT_PATH, 'utf8');

let outputArray = [];

console.log(der);
console.log(der.length);
console.log(der[0]);
console.log(typeof(der));

let i = 0;
while(i < der.length) {
    const decimal = parseInt(der[i] + der[i+1], 16);
    outputArray.push(decimal);
    i += 2;
}

let outputLen = outputArray.length;
if (outputLen < 1600) {
    outputArray.push(256);
    outputLen = outputArray.length;
}
while (outputLen < 1600) {
    outputArray.push(0);
    outputLen = outputArray.length;
}

const data = JSON.stringify(outputArray);
fs.writeFile(OUTPUT_PATH, data, (err) => {
 if (err) throw err;
});