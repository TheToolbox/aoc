use core::panic;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }

    let data = fs::read_to_string(&args[2])
        .unwrap();
    let commands = data.lines().map(Command::new);
    
    let mut sub = Submarine::new();

    
    if args[1] == "part1" { 
        commands.for_each(|c| sub.naively_navigate(c));
    } else { 
        commands.for_each(|c| sub.properly_navigate(c));
    };
    
    println!("{}",sub.result());
}

enum Command {
    Up(i64),
    Down(i64),
    Forward(i64),
}

impl Command {
    fn new(string: &str) -> Command {
        let strings: Vec<&str> = string.split(' ')
            .collect();

        match strings.as_slice() {
            ["up", y] => Command::Up(y.parse().unwrap()),
            ["down", y] => Command::Down(y.parse().unwrap()),
            ["forward", x] => Command::Forward(x.parse().unwrap()),
            z => {println!("{:?}",z); panic!()} 
        }
    }
}

struct Submarine {
    depth: i64,
    distance: i64,
    aim: i64,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            distance: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn naively_navigate(&mut self, command: Command) {
        match command {
            Command::Up(y) => self.depth -= y,
            Command::Down(y) => self.depth += y,
            Command::Forward(x) => self.distance += x,
        }
    }

    fn properly_navigate(&mut self, command: Command) {
        match command {
            Command::Up(y) => self.aim -= y,
            Command::Down(y) => self.aim += y,
            Command::Forward(x) => {
                self.distance += x;
                self.depth += self.aim * x;
            },
        }
    }

    fn result(&self) -> i64 {
        self.distance * self.depth
    }
}