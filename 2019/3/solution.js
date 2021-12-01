const { InputFile, TestFile } = require('../util');
const input = new InputFile(__dirname + '/input.txt');
const testFile = new TestFile(__dirname + '/test.txt');

//Part 1
function between(x, min, max) { return ((x - min) * (x - max) <= 0); }

class Segment {
    constructor(precedingLength) {
        this.precedingLength = precedingLength;
    }
    setStart(pos) {
        this.x1 = pos.x, this.y1 = pos.y;
    }
    setEnd(pos) {
        this.x2 = pos.x; this.y2 = pos.y;
    }
    vertical() {
        return this.x1 === this.x2;
    }
    length() {
        return Math.abs(this.x1 - this.x2) + Math.abs(this.y1 - this.y2);
    }
    intersect(other) {
        if (this.vertical() === other.vertical()) { return false; }
        const vertical = this.vertical() ? this : other;
        const horizontal = this.vertical() ? other : this;
        if (between(vertical.x1, horizontal.x1, horizontal.x2)
            && between(horizontal.y1, vertical.y1, vertical.y2)) {
            return { 
                x: vertical.x1,
                y: horizontal.y1,
                rank: this.precedingLength + other.precedingLength 
                    + Math.abs(vertical.x1 - horizontal.x1)
                    + Math.abs(horizontal.y1 - vertical.y1)
            };
        }
        return false;
    }
}

function moveListToSegmentList(moveList) {
    const pos = { x: 0, y: 0 };
    const segmentList = [];
    let lengthSoFar = 0;
    for (let i = 0; i < moveList.length; i++) {
        const segment = new Segment(lengthSoFar);
        segment.setStart(pos);
        const move = moveList[i];
        const value = parseInt(move.substring(1));
        lengthSoFar += value;
        const direction = move.substring(0, 1);
        if (direction === "U") { pos.y += value; }
        if (direction === "D") { pos.y -= value; }
        if (direction === "L") { pos.x -= value; }
        if (direction === "R") { pos.x += value; }
        segment.setEnd(pos);
        segmentList.push(segment);
    }
    return segmentList;
}

function getIntersectionsFromSegmentLists(segmentLists) {
    const intersections = [];
    for (let i = 0; i < segmentLists.length - 1; i++) {
        for (let j = i + 1; j < segmentLists.length; j++) {
            const list = segmentLists[i];
            const otherList = segmentLists[j];
            list.forEach(segment => {
                otherList.forEach(otherSegment => {
                    const intersection = segment.intersect(otherSegment);
                    if (intersection) { intersections.push(intersection); }
                });
            });
        }
    }
    return intersections;
}

function shortestIntersectDistance(moveLists) {
    function manhattan(p) {
        return Math.abs(p.x) + Math.abs(p.y)
    }

    const segmentLists = moveLists.map(moveListToSegmentList);
    const intersections = getIntersectionsFromSegmentLists(segmentLists)
        .filter(point => point.x !== 0 || point.y !== 0)
        .map(p => { return { x: p.x, y: p.y, d: manhattan(p) }; })
        .sort((p1, p2) => p1.d - p2.d);
    return manhattan(intersections[0]);
}

testFile.tests().forEach((test, i) => {
    const result = shortestIntersectDistance(test.input.linesAsCSVs());
    testFile.check(result, i, true);
});

const result1 = shortestIntersectDistance(input.linesAsCSVs());
console.log(`Result 1: ${result1}`);

//Part 2
function shortestSegmentDistance(moveLists) {
    const segmentLists = moveLists.map(moveListToSegmentList);
    const intersections = getIntersectionsFromSegmentLists(segmentLists)
        .filter(point => point.x !== 0 || point.y !== 0)
        .sort((p1, p2) => p1.rank - p2.rank);
    return intersections[0].rank;
}

const result2 = shortestSegmentDistance(input.linesAsCSVs());
console.log(`Result 2: ${result2}`);

module.exports = { result1, result2 };