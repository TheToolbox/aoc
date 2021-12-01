const { InputFile, TestFile } = require('../util');
const input = new InputFile(__dirname + '/input.txt');
const testFile = new TestFile(__dirname + '/test.txt');
const { IntCodeMachine } = require('../IntCodeMachine');

//Part 1
testFile.tests()
    .forEach((test, i) => {
        const memory = test.input.linesAsIntCSVs()[0];
        const machine = new IntCodeMachine(memory);
        const result = machine.run();
        testFile.check(machine.mem[0], i, true);
    });

const memory = input.linesAsStrings()[0]
    .split(',')
    .map(x => parseInt(x));
const machine = new IntCodeMachine(memory, false, 12, 2); //12 and 2 as specified in the problem
machine.run();
const result1 = machine.mem[0];
console.log(`Result 1: ${result1}`);

//part 2
const result2 = (() => {
    for (let noun = 0; noun < 100; noun++) {
        for (let verb = 0; verb < 100; verb++) {
            const memory = input.linesAsCSVs()[0]
                .map(x => parseInt(x));
            const machine = new IntCodeMachine(memory, false, noun, verb);
            machine.run();
            const result = machine.mem[0];
            if (result === 19690720) { return noun * 100 + verb; }
        }
    }
})();
console.log(`Result 2: ${result2}`);

module.exports = { result1, result2 };