const fs = require('fs');

class Input {
    constructor(rawData) {
        this.data = rawData;
    }

    linesAsStrings() {
        return this._linesAsStrings ?
            this._linesAsStrings :
            this._linesAsStrings = this.data.split('\n');
    }

    linesAsInts() {
        return this._linesAsInts ?
            this._linesAsInts :
            this._linesAsInts =
            this.linesAsStrings()
                .map(line => parseInt(line, 10))
                .filter(x => !isNaN(x));
    }

    linesAsCSVs() {
        return this._linesAsCSVs ?
            this._linesAsCSVs :
            this._linesAsCSVs =
            this.linesAsStrings()
                .map(line => line.split(','));
    }

    linesAsIntCSVs() {
        return this._linesAsIntCSVs ?
            this._linesAsIntCSVs :
            this._linesAsIntCSVs =
            this.linesAsStrings()
                .map(line =>
                    line.split(',')
                        .map(element => parseInt(element))
                );
    }
}

exports.InputFile = class InputFile extends Input {
    constructor(filename) {
        super(fs.readFileSync(filename, 'utf8').trim());
    }
}

exports.TestFile = class TestFile extends exports.InputFile {
    constructor(filename) {
        super(filename);
        this.testData = this.data
            .split('<=')
            .filter(testString => testString !== '')
            .map(testString => {
                const [data, result] = testString.split('=>');
                const input = new Input(data.trim());
                return { input, result };
            });
    }

    tests() {
        return this.testData;
    }

    check(result, testNumber, integerResults = false) {
        const expected = integerResults ?
            parseInt(this.testData[testNumber].result) :
            this.testData[testNumber].result;
        if (result === expected) {
            console.log(`Test ${testNumber} passed!`);
        } else {
            console.log(`Test ${testNumber} failed. Got ${typeof result} ${result}, expected ${typeof expected} ${expected}.`);
        }
    }

    answers() { return this._answers; }
}