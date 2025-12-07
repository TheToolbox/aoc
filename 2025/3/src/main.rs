// Common aocutils imports:
// use aocutils::parsing::*;        // parse_csv, parse_space_separated, parse_digit_grid, etc.
// use aocutils::grid::Grid2D;      // Grid with neighbors_4/8, iter_cells, etc.
// use aocutils::point::Point;      // Point with manhattan_distance, neighbors, etc.
// use aocutils::bits::*;           // get_bit, set_bit, count_ones, etc.

use aocutils::{get_input, Part};

fn main() {
    let input = get_input();

    let result: u64 = match input.part {
        Part::One => solve_part_one(input.lines),
        Part::Two => solve_part_two(input.lines),
    };

    println!("{}", result);

}

fn solve_part_one(lines: Vec<String>) -> u64 {
    println!("Solving part 1...");

    lines.iter()
        .map(|line| { 
            let bank: BatteryBank = line.as_str().into();
            bank.max_joltage_for_n_batteries(2)
        })
        .sum()
}

fn solve_part_two(lines: Vec<String>) -> u64 {
    println!("Solving part 2...");

    lines.iter()
        .map(|line| { 
            let bank: BatteryBank = line.as_str().into();
            bank.max_joltage_for_n_batteries(12)
        })
        .sum()
    // Your solution here
}

struct BatteryBank {
    joltages: Vec<u64>
}

impl From<&str> for BatteryBank {

    fn from(value: &str) -> Self {
        let strings = value.chars();


        let joltages = strings.into_iter()
            .map(|x| {
                x.to_digit(10).unwrap() as u64
            })
            .collect();

        BatteryBank {
            joltages
        }
    }
}

impl BatteryBank {
    fn max_joltage_for_n_batteries(self, n: usize) -> u64 {
        Self::max_for_n(self.joltages, n)
    }

    fn max_for_n(joltages: Vec<u64>, n: usize) -> u64 {
        if n == 0 { return 0; }
        let (index, &value) = joltages.iter()
            .enumerate()
            .take(joltages.len() - (n - 1))
            .rev()
            .max_by_key(|(_,val)| **val)
            .unwrap();
        return value * 10u64.pow(n as u32 - 1) + Self::max_for_n(joltages.into_iter().skip(index + 1).collect(), n - 1);
    }
}