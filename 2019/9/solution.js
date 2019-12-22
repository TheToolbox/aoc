const { InputFile, TestFile } = require('../util');
const input = new InputFile(__dirname + '/input.txt');
const testFile = new TestFile(__dirname + '/test.txt');
//const testFile2 = new TestFile(__dirname + '/test2.txt');
const { IntCodeMachine } = require('../IntCodeMachine');

testFile.tests().forEach((test, i) => {
    const program = test.input.linesAsIntCSVs()[0];
    const machine = new IntCodeMachine(program, false);
    109, 1
    204, -1
    1001, 100, 1, 100
    1008, 100, 16, 101
    1006, 101, 0
    99
    testFile.check(machine.run().join(','), i, false);
});

const program = input.linesAsIntCSVs()[0];
const machine = new IntCodeMachine(program);
machine.input(1);
const result1 = machine.run();
console.log(`Result 1: ${result1}`);

machine.reset();
machine.input(2);
const result2 = machine.run();
console.log(`Result 2: ${result2}`);
