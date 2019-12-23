const { InputFile, TestFile } = require('../util');
const input = new InputFile(__dirname + '/input.txt');
const testFile = new TestFile(__dirname + '/test.txt');
//const testFile2 = new TestFile(__dirname + '/test2.txt');

function gcd(a, b) {
    a = Math.abs(a);
    b = Math.abs(b);
    if (b > a) { const temp = a; a = b; b = temp; }
    while (true) {
        if (b == 0) return a;
        a %= b;
        if (a == 0) return b;
        b %= a;
    }
}

function countOctothorpes(mat) {
    let count = 0;
    for (const row of mat) {
        for (const point of row) {
            count += point === '#' ? 1 : 0;
        }
    }
    return count;
}

function countViewableAsteroids(map, station_row, station_col, debug) {
    map = map.map(row => row.slice(0));//take a copy so we don't change it
    if (debug) {
        console.log('\n' + map.reduce((prev, curr) => prev + '\n' + curr.join(''), ''));
    }
    //console.log(`Counting Asteroids viewable from (${station_row},${station_col})`);
    const rows = map.length, cols = map[0].length;
    for (let row = 0; row < rows; row++) {
        for (let col = 0; col < cols; col++) {
            if (row === station_row && col === station_col) {
                continue;
            }
            if (map[row][col] === '#') {//asteroid viewed at this position
                // console.log(`found asteroid at (${row},${col})`);
                let vr = row - station_row, vc = col - station_col;
                const factor = gcd(vr, vc);
                vr /= factor;
                vc /= factor;
                //console.log(`  shadowing along vector (${vr},${vc})`);
                let r = row + vr, c = col + vc;
                //shadow other asteroids
                //console.log(`  attempting to shadow (${r},${c})`)
                while (r in map && c in map[r]) {
                    //console.log(`  shadowed (${r}, ${c})`);
                    map[r][c] = '.';
                    r += vr; c += vc;
                }
            }
        }
    }
    if (debug) {
        console.log('\n' + map.reduce((prev, curr) => prev + '\n' + curr.join(''), ''));
    }
    return countOctothorpes(map) - 1;
}

function findBestAsteroid(map) {
    let max = 0, maxc = 0, maxr = 0;
    for (let r = 0; r < map.length; r++) {
        for (let c = 0; c < map[r].length; c++) {
            const count = countViewableAsteroids(map, r, c);
            if (map[r][c] === '#' && count > max) {
                maxc = c, maxr = r;
                max = count;
            }
        }
    }
    //console.log(`found at position (${maxc},${maxr})`)
    return { max, row: maxr, col: maxc };
}


testFile.tests().forEach((test, i) => {
    const map = test.input.linesAsStrings()
        .map(line => line.trim().split(''));
    testFile.check(findBestAsteroid(map).max, i, true);
});

const map = input.linesAsStrings()
    .map(line => line.trim().split(''));
const { max: result1, row: station_row, col: station_col } = findBestAsteroid(map);
console.log(`Result 1: ${result1}`);
console.log(`Station at (${station_col},${station_row})`)
let angleList = [];
map.forEach((row, rowi) =>
    row.forEach((point, coli) => {
        if (point !== '#' ||
            (rowi === station_row && coli === station_col)) { return; }
        const drow = rowi - station_row, dcol = coli - station_col;
        //r12,c11 is first for 802ex
        const angle = Math.round(
            1000 * //multiply the whole thing by 1000 before rounding
            ((Math.atan2(drow, dcol) //angle in radians
                * (180 / Math.PI) //to degrees
                * 1 + 360 + 90) % 360)//clockwise starting from 'up', no negatives
        );
        if (!(angle in angleList)) { angleList[angle] = []; }
        angleList[angle].push({
            pos: `(${coli},${rowi})`,
            angle,
            row: rowi,
            col: coli,
            distance: Math.sqrt(drow ** 2 + dcol ** 2),
        });
    })
);
angleList = angleList.filter(x => x !== undefined);
let rot = 0, asteroids = 0;
let asteroid;
while (asteroids < 200) {
    const angle = angleList[rot];
    angle.sort((a, b) => b.distance - a.distance);
    asteroid = angle.pop();
    asteroids += asteroid ? 1 : 0;
    rot++;
    rot %= angleList.length;
    //console.log(asteroid);
}

const result2 = asteroid.col * 100 + asteroid.row;
console.log(`Result 2: ${result2}`);
