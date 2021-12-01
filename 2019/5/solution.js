const { InputFile, TestFile } = require('../util');
const input = new InputFile(__dirname + '/input.txt');
const { IntCodeMachine } = require('../IntCodeMachine');
//const testFile = new TestFile(__dirname + '/test.txt');

//part 1
const memory = input.linesAsStrings()[0]
    .split(',')
    .map(x => parseInt(x));
const machine = new IntCodeMachine(memory);
machine.input(1); //send input 1 as specified in the problem
const output = machine.run();

const result1 = output.pop();
console.log(`Result 1: ${result1}`);

//part 2
const memory2 = input.linesAsStrings()[0]
    .split(',')
    .map(x => parseInt(x));
const machine2 = new IntCodeMachine(memory2);
machine2.input(5); //send input 1 as specified in the problem
const output2 = machine2.run();

const result2 = output2.pop();
console.log(`Result 2: ${result2}`);

module.exports = { result1, result2 };