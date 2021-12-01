const { InputFile, TestFile } = require('../util');
const input = new InputFile(__dirname + '/input.txt');
const testFile = new TestFile(__dirname + '/test.txt');
//const testFile2 = new TestFile(__dirname + '/test2.txt');
const { IntCodeMachine } = require('../IntCodeMachine');

class Hull {
    constructor() {
        this.data = [];
        this.painted = new Set([]);
        this.minx = 0, this.maxx = 0, this.miny = 0, this.maxy = 0;
    }

    set(x, y, value) {
        this.painted.add(`${x},${y}`);
        if (!(x in this.data)) {
            this.data[x] = [];
        }
        this.data[x][y] = value;
        if (y > this.maxy) { this.maxy = y; }
        if (x > this.maxx) { this.maxx = x; }
        if (y < this.miny) { this.miny = y; }
        if (x < this.minx) { this.minx = x; }
    }

    get(x, y) {
        if (!(x in this.data) || !(y in this.data[x])) {
            return 0;
        }
        return this.data[x][y];
    }

    getNumberPainted() { return this.painted.size; }

    print() {
        const result = [];
        for (let y = this.miny; y < this.maxy + 1; y++) {
            const row = [];
            for (let x = this.minx; x < this.maxx + 1; x++) {
                row.push(this.get(x, y) ? '█' : ' ');
            }
            result.push(row.join(''));
        }
        console.log(result.reverse().join('\n'));
    }
}

class PainterBot {
    constructor(program, hull) {
        this.ai = new IntCodeMachine(program);
        this.hull = hull;
        this.reset();
    }

    move() {
        switch (this.direction) {
            case 0: return this.y++;
            case 1: return this.x++;
            case 2: return this.y--;
            case 3: return this.x--;
            default: throw new Error('impossible direction');
        }
    }

    step() {
        const color = this.hull.get(this.x, this.y);
        this.ai.input(color);
        const [newColor, turnRight] = this.ai.run();
        this.hull.set(this.x, this.y, newColor);
        this.direction = (this.direction + 4 + (turnRight ? 1 : -1)) % 4;
        this.move();
    }

    run() {
        while (this.ai.running) {

            this.step();
        }
    }

    countPaintedTiles() { return this.hull.getNumberPainted(); }

    reset() {
        this.ai.reset();
        this.x = 0; this.y = 0; this.direction = 0;
    }
}

const program = input.linesAsIntCSVs()[0];
const bot = new PainterBot(program, new Hull());
bot.run();
const result1 = bot.countPaintedTiles();
console.log(`Result 1: ${result1}`);

const hull = new Hull();
hull.set(0, 0, 1);
const bot2 = new PainterBot(program, hull)
bot2.run();
hull.print();
//console.log(hull.data);
const result2 = 0;
console.log(`Result 2: ${result2}`);
