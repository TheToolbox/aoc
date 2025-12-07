use core::panic;
use std::fmt;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }

    let data = fs::read_to_string(&args[1])
        .unwrap();
    let lines = data.lines();

    let mut turns = lines.map(|str| {
        let mut chars = str.chars();
        let first_char = chars.next();

        let left = first_char == Some('L');
        
        let mut val : i32 = chars.as_str().parse::<i32>().expect("Failed to parse string");
        if left {
            val = -val;
        }
        val
    });

    let part = 2;
    let result = match part {
        1 => part1(&mut turns),
        2 => part2(&mut turns),
        _ => panic!("Uh we don't have that part"),
    };
    println!("{}", result);
}

fn part1(turns : &mut dyn Iterator<Item = i32>) -> i32 {
    turns.fold((50,0), |(sum, count), x| {
        println!("{} + {} = {}",sum, x, (sum + x) % 100);
        ((sum + x) % 100, if sum == 0 { count + 1 } else { count })
    }).1
}

fn part2(turns : &mut dyn Iterator<Item = i32>) -> i32 {
    turns.fold((50,0), |(sum, count), x| {
        //println!("{} + {} = {}",sum, x, (sum + x) % 100);
        let oldsum = sum;
        let mut sum = sum + x;
        let mut extracount = 0;
        println!("==================");
        println!("Rotating {}",x);

        
        while sum > 99 {
            sum -= 100;
            extracount += 1;
        }
        if (sum < 0 && oldsum == 0) || (sum == 0 && x > 0) {
            extracount -= 1;
        }
        while sum < 0 {

            sum += 100;
            extracount += 1;
        }
        if sum == 0 { extracount += 1; }
        println!("Turns {} to {}", oldsum, sum);
        println!("Rotates past zero {} times",extracount);


        (sum, count + extracount)//if sum == 0 { count + 1 } else { count })
    }).1
}