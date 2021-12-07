use core::panic;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Missing argument");
    }

    let data = fs::read_to_string(&args[2])
        .unwrap();
    let line = data.lines().next().unwrap();

    println!("{:?}",line);
    
    let fishes: Vec<LanternFish> = line.split(',')
        .map(|x| 
            LanternFish::new(x.parse::<u64>().unwrap())
        )
        .collect();
   
    let mut school = School::new(fishes);
    let iterations = args[3].parse::<u64>().unwrap();

    for i in 0..iterations {
        println!("Iteration {}. Fish Count: {}", i, school.total());
        school.step();
    }
    
    println!("{}",school.total());
}

struct LanternFish(u64);

impl LanternFish {
    fn new(age: u64) -> LanternFish{
        LanternFish(age)
    }

    //naive first attempt. works for part 1, but is way too slow for part 2 (hence the 'School' approach)
    fn _step(&mut self) -> Option<LanternFish> {
        if self.0 < 1 {
            self.0 = 6;
            Some(LanternFish::new(8))
        } else {
            self.0 -= 1;
            None
        }
    }
}

struct School([u64; 9]);

impl School {
    fn new(fishes: Vec<LanternFish>) -> School {
        let mut counts = [0u64; 9];
        
        for fish in fishes {
            let index: usize = fish.0.try_into().unwrap();
            counts[index] += 1;
        }

        School(counts)
    }

    fn step(&mut self) {

        //save and reset the 0 day fish
        let zeroes = self.0[0];
        self.0[0] = 0;

        for count in 1..9 {
            self.0[count - 1] = self.0[count];
        }

        self.0[8] = zeroes;
        self.0[6] += zeroes;
    }

    fn total(&self) -> u64 {
        self.0.iter().sum()
    }
}