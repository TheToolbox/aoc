// Common aocutils imports:
// use aocutils::parsing::*;        // parse_csv, parse_space_separated, parse_digit_grid, etc.
// use aocutils::grid::Grid2D;      // Grid with neighbors_4/8, iter_cells, etc.
// use aocutils::point::Point;      // Point with manhattan_distance, neighbors, etc.
// use aocutils::bits::*;           // get_bit, set_bit, count_ones, etc.
use std::convert::TryFrom;
use std::convert::TryInto;
use std::error::Error;
use std::num::ParseIntError;
use aocutils::parsing::parse_csv;
use aocutils::{get_input, Part};

fn main() {
    let input = get_input();

    let data: Vec<String> = parse_csv(input.lines.iter().next().unwrap());
    let mut ranges = data.into_iter()
        .map(|val| {
            let x: Range = val.as_str().try_into().unwrap();
            x
        });

    let result = match input.part {
        Part::One => solve_part_one(&mut ranges),
        Part::Two => solve_part_two(&mut ranges),
    };

    println!("{}",result);
}

fn solve_part_one(ranges: &mut dyn Iterator<Item = Range>) -> u64 {
    println!("Solving part 1...");

    ranges.map(|range| range.get_invalids(true)).sum()
}

fn solve_part_two(ranges: &mut dyn Iterator<Item = Range>) -> u64 {
    println!("Solving part 2...");

    ranges.map(|range| range.get_invalids(false)).sum()
}

struct Range {
    start: u64,
    end: u64,
}

impl TryFrom<&str> for Range {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (s,e) = value.split_once('-').ok_or(())?;

        Ok(Range {
            start: s.parse().or(Err(()))?,
            end: e.parse().or(Err(()))?,
        })
    }
}

impl Range {
    fn get_invalids(&self, only_one_repeat: bool) -> u64 {
        let mut count = 0;

        for i in self.start..=self.end {
            let digits = i.ilog10() + 1;
            
            let invalid = factors(digits).iter()
                .filter(|x| { !only_one_repeat || **x == digits / 2 })
                .any(|pattern_length| {
                    let mut a = ProductID {x: i};

                    let mut last = a.pop(*pattern_length).unwrap_or(0);
                    while let Some(val) = a.pop(*pattern_length) {
                        if val != last { return false; } else { last = val; }
                    };
                    
                    true
                });

            
            if invalid { 
                println!("Invalid key: {}",i);
                count += i;
            }
        }

        count
    }
}

fn factors(x: u32) -> Vec<u32> {
    let mut result = vec![];
    for i in 1..(x / 2 + 1) {
        if x % i == 0 {
            result.push(i)
        }
    }
    result
}

struct ProductID { x: u64 }

impl ProductID {
    fn pop(&mut self, digits: u32) -> Option<u64> {
        if self.x == 0 { return None; }
        let divider = 10u64.pow(digits);
        let remainder = self.x % divider;
        self.x /= divider;
        Some(remainder)
    }
}