exports.IntCodeMachine = class IntCodeMachine {
    constructor(program, debug = false, noun, verb) {
        this.program = program.slice(0);
        this.pc = 0;
        this.inputs = [];
        this.outputs = [];
        this.debug = debug;
        this.running = true;
        this.waitingForIO = false;
        if (noun) { program[1] = noun; }
        if (verb) { program[2] = verb; }
        this.reset();
    }

    step() {
        const instruction = String(this.mem[this.pc])
            .padStart(5, '0')
            .split('')
            .map(char => parseInt(char));
        if (this.debug) { console.log(`PC: ${this.pc}, Instruction: ${instruction.join('')}`); }
        if (instruction.length > 5) {
            throw new Error(`Instruction (${instruction.join('')}) too long`);
        }
        //extract opcode
        const opcode = instruction[3] * 10 + instruction[4];
        //parse parameter mode
        const paramAddresses = [1, 2, 3].map(offset =>
            instruction[3 - offset]
                ? this.pc + offset
                : this.mem[this.pc + offset]
        );

        if (this.debug) { console.log(`Opcode: ${opcode}, ParameterAddresses: ${paramAddresses}`); }

        switch (opcode) {
            case 1: return this.op1(paramAddresses);
            case 2: return this.op2(paramAddresses);
            case 3: return this.op3(paramAddresses);
            case 4: return this.op4(paramAddresses);
            case 5: return this.op5(paramAddresses);
            case 6: return this.op6(paramAddresses);
            case 7: return this.op7(paramAddresses);
            case 8: return this.op8(paramAddresses);
            case 99: return this.running = false;
            default:
                throw new Error(`invalid opcode ${opcode}`);
        }
    }

    op1(addresses) {
        this.log(`mem[${addresses[2]}] = ${this.mem[addresses[0]]} + ${this.mem[addresses[1]]};`);
        this.mem[addresses[2]] = this.mem[addresses[0]] + this.mem[addresses[1]];
        this.pc += 4;
    }

    op2(addresses) {
        this.log(`mem[${addresses[2]}] = ${this.mem[addresses[0]]} * ${this.mem[addresses[1]]};`);
        this.mem[addresses[2]] = this.mem[addresses[0]] * this.mem[addresses[1]];
        this.pc += 4;
    }

    op3(addresses) {
        if (this.inputs.length > 0) {
            this.log(`Storing input value ${this.inputs[0]} to ${addresses[0]}.`);
            this.mem[addresses[0]] = this.inputs.splice(0, 1)[0];
            this.pc += 2;
        } else {
            this.log(`Halting to wait for input.`);
            //wait for input
            this.waitingForIO = true;
        }
    }

    op4(addresses) {
        this.log(`Outputting value ${this.mem[addresses[0]]} to buffer.`);
        this.outputs.push(this.mem[addresses[0]]);
        this.pc += 2;
        //if (this.outputs[this.outputs.length - 1] !== 0) { this.running = false; }
    }

    op5(addresses) {
        this.log(`If ${this.mem[addresses[0]]} is nonzero, jumping to ${this.mem[addresses[1]]}.`);
        this.pc = this.mem[addresses[0]] !== 0 ? this.mem[addresses[1]] : this.pc + 3;
    }

    op6(addresses) {
        this.log(`If ${this.mem[addresses[0]]} is zero, jumping to ${this.mem[addresses[1]]}.`);
        this.pc = this.mem[addresses[0]] === 0 ? this.mem[addresses[1]] : this.pc + 3;
    }

    op7(addresses) {
        this.log(`mem[${addresses[2]}] = ${this.mem[addresses[0]]} < ${this.mem[addresses[1]]};`);
        this.mem[addresses[2]] = this.mem[addresses[0]] < this.mem[addresses[1]] ? 1 : 0;
        this.pc += 4;
    }

    op8(addresses) {
        this.log(`mem[${addresses[2]}] = ${this.mem[addresses[0]]} === ${this.mem[addresses[1]]};`);
        this.mem[addresses[2]] = this.mem[addresses[0]] === this.mem[addresses[1]] ? 1 : 0;
        this.pc += 4;
    }

    log(string) {
        if (this.debug) {
            console.log(string);
        }
    }

    input(num) { this.inputs.push(num); }

    run() {
        this.waitingForIO = false;
        while (this.running && !this.waitingForIO) {
            this.step();
        }
        return this.outputs.splice(0);
    }

    reset() {
        this.mem = this.program.slice(0); //clone so that we can reset
        this.pc = 0;
        this.inputs = [];
        this.outputs = [];
        this.running = true;
        this.waitingForIO = false;
    }
}