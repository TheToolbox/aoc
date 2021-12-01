use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }

    let lines = fs::read_to_string(&args[2])
        .unwrap();
    let numbers = lines.lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    
    let result = if args[1] == "part1" { count_increasing(numbers, 1) } else { count_increasing(numbers, 3) };
    
    println!("{}",result);
}

fn count_increasing(numbers:Vec<u64>, window_size: usize) -> u32 {
    let mut last = u64::MAX;
    let mut count = 0;

    for window in numbers.windows(window_size) {
        let n = window.iter().sum();
        if  n > last {
            count += 1;
        }
        last = n;
    }

    count
}