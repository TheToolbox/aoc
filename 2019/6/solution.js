const { InputFile, TestFile } = require('../util');
const input = new InputFile(__dirname + '/input.txt');
const testFile = new TestFile(__dirname + '/test.txt');

function parseOrbits(lines) {
    return lines.reduce((orbits, orbit) => {
        const [orbited, orbiting] = orbit.split(')');
        orbits[orbiting] = orbited;
        return orbits;
    }, {});
}

function countMyOrbits(orbits, orbiter) {
    return orbits[orbiter] ? 1 + countMyOrbits(orbits, orbits[orbiter]) : 0;
}

function countOrbits(orbits) {
    return Object.keys(orbits).reduce((total, orbiter) => total + countMyOrbits(orbits, orbiter), 0);
}

testFile.tests().forEach((test, i) => {
    const orbits = parseOrbits(test.input.linesAsStrings());
    testFile.check(countOrbits(orbits), i, true);
});

const orbits = parseOrbits(input.linesAsStrings());
const result1 = countOrbits(orbits);

console.log(`Result 1: ${result1}`);

//part 2

function doesOrbit(orbits, orbiter, target) {
    return orbits[orbiter] === target ? true :
        orbits[orbiter] === undefined ? false :
            doesOrbit(orbits, orbits[orbiter], target);
}

function findCommonBody(orbits, first, second) {
    return doesOrbit(orbits, second, orbits[first]) ? orbits[first] : findCommonBody(orbits, orbits[first], second);
}

function transfersRequired(orbits, start, end) {
    const common = countMyOrbits(orbits, findCommonBody(orbits, start, end));
    return countMyOrbits(orbits, start) - common + countMyOrbits(orbits, end) - common;
}

const testFile2 = new TestFile(__dirname + '/test2.txt');
testFile2.tests().forEach((test, i) => {
    const orbits = parseOrbits(test.input.linesAsStrings());
    testFile2.check(transfersRequired(orbits, orbits['YOU'], orbits['SAN']), i, true);
});

const result2 = transfersRequired(orbits, orbits['YOU'], orbits['SAN']);
console.log(`Result 2: ${result2}`);

module.exports = { result1, result2 };



