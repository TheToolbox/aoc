use core::panic;
use std::fs;
use std::env;
use std::str::Lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }

    let data = fs::read_to_string(&args[2])
        .unwrap();
    
    let (power, life_support) = Submarine::parse_diagnostics(data.lines(), 12);

    let result = if args[1] == "part1" { 
        power
    } else { 
        life_support
    };
    
    println!("{}",result);
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

    fn parse_diagnostics(lines: Lines, bit_width: usize) -> (u64,u64) {
        let lines2 = lines.clone();
        let values = lines.map(|s| 
            s.chars().map(|bit| 
                if bit == '1' {1} else {0}
            )
        );
        
        //just get the longest value's length in bits
        //let width = values.map(|value| value.count()).max().unwrap();
        let mut counts = vec![0; bit_width];
        let mut len = 0;
        for value in values.clone() {
            for (i, bit) in value.enumerate() { //MSB to LSB
                counts[i] += bit;
            }
            len += 1;
        }

        println!("{:?} out of total {:?}",counts,len);

        let gamma = counts.clone()
            .into_iter()
            .map(|count| if count > len/2 {1} else {0})
            .fold(0, |gamma, bit| 
                (gamma << 1) + bit
            );


        let mask = vec![1; bit_width].into_iter().fold(0, |gamma, bit| 
            (gamma << 1) + bit
        );
        let epsilon = !gamma & mask;
        
        println!("Epsilon:  {:#014b}",epsilon);
        println!("Gamma: {:#014b}",gamma);

        let mut values2: Vec<u64> = lines2.map(|line| 
            u64::from_str_radix(line,2).unwrap()
        ).collect();
        let mut values3 = values2.clone();

        let mut i: u32 = bit_width.try_into().unwrap();
        while values2.len() > 1 {
            i -= 1;
            let bit = most_common_bit(&values2, i, 1);
            println!("Bit {} most common: {:?}", i, bit);
            values2 = values2.into_iter()
                .filter(|x| get_bit(*x,i) == match bit {
                    Bit::Equal => 1,
                    Bit::Different(x) => x,
                })
                .collect();
            println!("After filtering:");
            println!("{:?}",values2);
            
        }
        println!("Oxygen generator rating: {:?}",values2);

        let mut i: u32 = bit_width.try_into().unwrap();
        while values3.len() > 1 {
            i -= 1;
            let bit = most_common_bit(&values3, i, 0);
            println!("Bit {} least common: {:?}", i, bit);
            values3 = values3.into_iter()
                .filter(|x| get_bit(*x,i) == match bit {
                    Bit::Equal => 0,
                    Bit::Different(x) => 1 - x,
                })
                .collect();
            println!("After filtering:");
            println!("{:?}",values3);
        }
        println!("CO2 Scrubber rating: {:?}",values3);

        (gamma * epsilon, values2[0] * values3[0])
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

fn get_bit(num: u64, position: u32) -> u64 {
    let two: u64 = 2;
    (two.pow(position) & num) >> position
}

#[derive(Debug)]
enum Bit {
    Equal,
    Different(u64),
}

fn most_common_bit(nums: &[u64], bit_position: u32, bias: u64) -> Bit {
    let two: u64 = 2;
    let len: u64 = nums.len().try_into().unwrap();
    let count: u64 = nums.iter()
        .map(|num| (two.pow(bit_position) & num) >> bit_position)
        .sum();
    if count == len/2 && len%2 == 0 {Bit::Equal} else if count > len/2 {Bit::Different(1)} else {Bit::Different(0)}
}