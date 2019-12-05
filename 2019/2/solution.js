const { InputFile, TestFile } = require('../util');
const input = new InputFile(__dirname + '/input.txt');
const testFile = new TestFile(__dirname + '/test.txt');

//Part 1
class IntCodeMachine {
    constructor(memory, debug = false, noun, verb) {
        this.mem = memory;
        this.pc = 0;
        this.running = true;
        this.debug = debug;
        if (noun) { memory[1] = noun; }
        if (verb) { memory[2] = verb; }
    }

    step() {
        const opcode = this.mem[this.pc];
        switch (opcode) {
            case 1:
                return this.op1();
            case 2:
                return this.op2();
            case 99:
                return this.op99();
            default:
                if (this.debug) { console.log(`invalid opcode: ${opcode}.`); }
                throw new Error('invalid opcode');
        }
    }

    op1() {
        if (this.debug) { console.log(`mem[${this.mem[this.pc + 3]}] = ${this.mem[this.mem[this.pc + 1]]} + ${this.mem[this.mem[this.pc + 2]]};`); }
        this.mem[this.mem[this.pc + 3]] = this.mem[this.mem[this.pc + 1]] + this.mem[this.mem[this.pc + 2]];
        this.pc += 4;
    }

    op2() {
        if (this.debug) { console.log(`mem[${this.mem[this.pc + 3]}] = ${this.mem[this.mem[this.pc + 1]]} * ${this.mem[this.mem[this.pc + 2]]};`) }
        this.mem[this.mem[this.pc + 3]] = this.mem[this.mem[this.pc + 1]] * this.mem[this.mem[this.pc + 2]];
        this.pc += 4;
    }

    op99() {
        return this.running = false;
    }

    run() {
        while (this.running) {
            try {
                this.step();
            } catch (err) {
                return -Infinity;
            }
        }
        return this.mem[0];
    }
}

testFile.tests()
    .forEach((test, i) => {
        const memory = test.input.linesAsIntCSVs()[0];
        const machine = new IntCodeMachine(memory);
        const result = machine.run();
        testFile.check(result, i, true);
    });

const memory = input.linesAsStrings()[0]
    .split(',')
    .map(x => parseInt(x));
const machine = new IntCodeMachine(memory, false, 12, 2); //12 and 2 as specified in the problem
const result1 = machine.run();
console.log(`Result 1: ${result1}`);

//part 2

const result2 = (() => {
    for (let noun = 0; noun < 100; noun++) {
        for (let verb = 0; verb < 100; verb++) {
            const memory = input.linesAsCSVs()[0]
                .map(x => parseInt(x));
            const machine = new IntCodeMachine(memory, false, noun, verb);
            const result = machine.run();
            if (result === 19690720) { return noun * 100 + verb; }
        }
    }
})();
console.log(`Result 2: ${result2}`);

module.exports = { result1, result2 };