const { InputFile, TestFile } = require('../util');
const input = new InputFile(__dirname + '/input.txt');
const testFile = new TestFile(__dirname + '/test.txt');
const testFile2 = new TestFile(__dirname + '/test2.txt');
const { IntCodeMachine } = require('../IntCodeMachine');

class AmplifierArray {
    constructor(program) {
        this.amps = [1, 2, 3, 4, 5].map(x => new IntCodeMachine(program));
    }

    reset() { this.amps.forEach(amp => amp.reset()); }

    setPhaseSettings(phaseSettings, shouldReset = true) {
        if (shouldReset) { this.reset(); }
        this.amps.forEach((amp, i) => amp.input(phaseSettings[i]));
    }

    activate(feedback = false, io = 0, loopLevel = 0) {
        for (let i = 0; i < this.amps.length; i++) {
            this.amps[i].input(io);
            io = this.amps[i].run()[0];
        }
        const stillRunning = this.amps.reduce((otherRunning, amp) => otherRunning && amp.running, true);
        return feedback && stillRunning ? this.activate(true, io, ++loopLevel) : io;
    }
}

// part 1
function permutations(arr) { //stolen from the internet
    let ret = [];

    for (let i = 0; i < arr.length; i = i + 1) {
        let rest = permutations(arr.slice(0, i).concat(arr.slice(i + 1)));

        if (!rest.length) {
            ret.push([arr[i]])
        } else {
            for (let j = 0; j < rest.length; j = j + 1) {
                ret.push([arr[i]].concat(rest[j]))
            }
        }
    }
    return ret;
}

function findBestOutput(program, possibleSettings, feedback) {
    const amps = new AmplifierArray(program);
    const result = Math.max.apply(null,
        permutations(possibleSettings)
            .map(settings => {
                amps.setPhaseSettings(settings);
                return amps.activate(feedback);
            })
    );
    return result
}

testFile.tests().forEach((test, i) => {
    const program = test.input.linesAsIntCSVs()[0];
    testFile.check(findBestOutput(program, [0, 1, 2, 3, 4]), i, true);
});

const program = input.linesAsIntCSVs()[0];
const result1 = findBestOutput(program, [0, 1, 2, 3, 4]);
console.log(`Result 1: ${result1}`);

// part 2

testFile2.tests().forEach((test, i) => {
    const program = test.input.linesAsIntCSVs()[0];
    testFile2.check(findBestOutput(program, [5, 6, 7, 8, 9], true), i, true);
});

const result2 = findBestOutput(program, [5, 6, 7, 8, 9], true);
console.log(`Result 2: ${result2}`);

module.exports = { result1, result2 };