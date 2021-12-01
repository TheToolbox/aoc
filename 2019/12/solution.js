const { InputFile, TestFile } = require('../util');
const input = new InputFile(__dirname + '/input.txt');
//const testFile = new TestFile(__dirname + '/test.txt');
//const testFile2 = new TestFile(__dirname + '/test2.txt');

class Body {
    constructor(coords) {
        this.x = coords[0];
        this.y = coords[1];
        this.z = coords[2];
        this.vx = 0, this.vy = 0, this.vz = 0;
    }

    influence(other) {
        if (this.x > other.x) {
            this.vx--; other.vx++;
        } else if (this.x < other.x) {
            this.vx++; other.vx--;
        }
        if (this.y > other.y) {
            this.vy--; other.vy++;
        } else if (this.y < other.y) {
            this.vy++; other.vy--;
        }
        if (this.z > other.z) {
            this.vz--; other.vz++;
        } else if (this.z < other.z) {
            this.vz++; other.vz--;
        }
    }

    update() { this.x += this.vx, this.y += this.vy, this.z += this.vz; }

    energy() {
        return (Math.abs(this.x) + Math.abs(this.y) + Math.abs(this.z))
            * (Math.abs(this.vx) + Math.abs(this.vy) + Math.abs(this.vz));
    }

}

class System {
    constructor(coords) {
        this.startingCoords = coords;
        this.reset();
    }

    simulate(iterations) {
        for (let i = 0; i < iterations; i++) {
            for (let b1 = 0; b1 < this.bodies.length - 1; b1++) {
                //update v
                for (let b2 = b1 + 1; b2 < this.bodies.length; b2++) {
                    this.bodies[b1].influence(this.bodies[b2]);
                }
            }
            //update position
            this.bodies.forEach(body => body.update());
        }
    }

    energy() {
        return this.bodies.reduce((total, body) => total + body.energy(), 0);
    }

    periodicity() {
        this.reset();
        history = [];
        //get periodicity in x
        if (!(pos in history)) { history[pos] = []; }
        this.simulate(1);
    }

    reset() {
        this.bodies = this.startingCoords.map(coords => new Body(coords));
    }
}

const coords = input.linesAsStrings()
    .map(item => item.split(',')
        .map(coordinate =>
            parseInt(coordinate.replace(/[<=xyz ]/g, ''))
        ))
const system = new System(coords);
system.simulate(1000);
const result1 = system.energy();
console.log(`Result 1: ${result1}`);


const result2 = 0;
console.log(`Result 2: ${result2}`);
