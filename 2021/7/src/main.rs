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
    


    let crabs: Vec<Crab> = line.split(',')
        .map(|x| 
            Crab::new(x.parse::<u64>().unwrap())
        )
        .collect();
   

    let using_improved_cost_estimate = args[1] != "part1";
    let mut min = u64::MAX;
    for i in 0..2000 {
        let cost = crabs.iter()
            .map(|crab| crab.cost_to_move_to(i, using_improved_cost_estimate))
            .sum();

        if cost < min {
            min = cost;
        }
    }
    
    
    println!("{}",min);
}
struct Crab(u64);

impl Crab {
    fn new(num: u64) -> Crab {
        Crab(num)
    }



    fn cost_to_move_to(&self, pos: u64, using_improved_cost_estimate: bool) -> u64 {
        let distance = if self.0 > pos {
            self.0 - pos
        } else {
            pos - self.0
        };

        if using_improved_cost_estimate {
            distance * (distance + 1) / 2
        } else {
            distance
        }       

    }
}