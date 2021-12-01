const InputFile = require('../util').InputFile;
const input = new InputFile(__dirname + '/input.txt');

//Part 1
const result1 = input.linesAsInts()
    .map(mass => Math.floor(mass/3) - 2)
    .reduce((a, b) => a + b, 0); //sum it all

console.log(`Result1: ${result1}`);

//Part 2
const result2 = input.linesAsInts()
    .map(mass => {
        let total = 0;
        let additionalFuel = Math.floor(mass/3) - 2;
        while (additionalFuel > 0) {
            total += additionalFuel;
            additionalFuel = Math.floor(additionalFuel/3) - 2;
        }
        return total;
    })
    .reduce((a,b) => a + b, 0);
    
console.log(`Result2: ${result2}`);

exports = { result1, result2 };