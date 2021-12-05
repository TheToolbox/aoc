use core::panic;
use std::fmt;
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
    let lines = data.lines();
    
    println!("hi");
    let (callouts, mut boards) = Submarine::parse_input(lines);
    
    if args[1] == "part1" { 
        for callout in callouts {
            println!("Drew a {}!", callout);
            for board in &mut boards {
                board.mark(callout);
                if board.is_winner() {
                    println!("Winner!");
                    println!("{}", board.score() * callout);
                    board.print();
                    return;
                }
            }
        }
    } else { 
        for callout in callouts {
            println!("Drew a {}!", callout);
            for board in &mut boards {
                board.mark(callout);
                if board.is_winner() {
                    println!("Winner!");
                    println!("{}", board.score() * callout);
                    board.print();
                }
            }
        }
    };
    
    //println!("{}",sub.result());
}

struct Submarine {}


#[derive(Clone,Copy)]
enum BingoNumber {
    Unmarked(u64),
    Marked(u64),
}

impl fmt::Display for BingoNumber {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            Self::Unmarked(x) => write!(f, " {:3}", x),
            Self::Marked(x) => write!(f, ">{:3}", x)
        }
        
    }
}

struct BingoBoard {
    numbers: Vec<BingoNumber>,
    haswon: bool,
}

impl BingoBoard {
    fn new(input: Vec<&str>) -> BingoBoard {
        let mut numbers: Vec<BingoNumber> = vec![];
        for line in input {
            for x in line.split_ascii_whitespace() {
                numbers.push(BingoNumber::Unmarked(
                    x.parse().unwrap()
                ));
            }
        }

        BingoBoard {
            numbers,
            haswon: false
        }
    }

    fn mark(&mut self, num: u64) {
        let newnumbers: Vec<BingoNumber> = self.numbers.clone();
        self.numbers = newnumbers.into_iter().map(|x| match x {
                BingoNumber::Unmarked(n) if n == num => BingoNumber::Marked(num),
                z => z
            })
            .collect();
    }

    fn is_winner(&mut self) -> bool {
        if self.haswon { return false; }
        //check rows
        for i in 0..5 {
            let mut winner = true;
            for j in 0..5 {
                if let BingoNumber::Unmarked(_) = self.numbers[i * 5 + j] {
                    winner = false;
                }
            }
            if winner { self.haswon = true; return true }
        }
        
        //check columns
        for i in 0..5 {
            let mut winner = true;
            for j in 0..5 {
                if let BingoNumber::Unmarked(_) = self.numbers[i + j * 5] {
                    winner = false;
                }
            }
            if winner { self.haswon = true; return true }
        }

        false
    }

    fn score(&self) -> u64 {
        let numbers = &self.numbers;
        
        numbers.iter()
            .filter_map(|num| 
                match num {
                    BingoNumber::Unmarked(x) => Some(x),
                    BingoNumber::Marked(_) => None,
                })
            .sum()
    }

    fn print(&self) {
        println!();
        for i in 0..5 {
            for j in 0..5 {
                print!("{}",self.numbers[i * 5 + j]);
            }
            println!();
        }
    }
}

impl Submarine {
    fn parse_input(mut lines: Lines) -> (Vec<u64>, Vec<BingoBoard>) {
        let callouts: Vec<u64> = lines.next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        let mut boards : Vec<BingoBoard> = vec![];
        while lines.next() != None {
            let input = lines.by_ref().take(5).collect();
            boards.push(BingoBoard::new(input));
        }

        (callouts,boards)
    }
}