#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "please pass day number"
    exit 1
fi

# Check if directory already exists
if [ -d "$1" ]; then
    echo "Error: Directory '$1' already exists. Aborting to prevent overwriting."
    exit 1
fi

mkdir $1
cd $1
cargo init --name solution$1

# Add aocutils dependency
echo 'aocutils = { path = "../aocutils" }' >> Cargo.toml

# Create main.rs with template
cat > src/main.rs << 'EOF'
// Common aocutils imports:
// use aocutils::parsing::*;        // parse_csv, parse_space_separated, parse_digit_grid, etc.
// use aocutils::grid::Grid2D;      // Grid with neighbors_4/8, iter_cells, etc.
// use aocutils::point::Point;      // Point with manhattan_distance, neighbors, etc.
// use aocutils::bits::*;           // get_bit, set_bit, count_ones, etc.

use aocutils::{get_input, Part};

fn main() {
    let input = get_input();

    let result = match input.part {
        Part::One => solve_part_one(&input.lines),
        Part::Two => solve_part_two(&input.lines),
    };

    println!("{}", result);
}

fn solve_part_one(lines: &[String]) -> usize {
    println!("Solving part 1...");

    // TODO: Implement part 1
    0
}

fn solve_part_two(lines: &[String]) -> usize {
    println!("Solving part 2...");

    // TODO: Implement part 2
    0
}
EOF

# Download the input and examples (needs your session cookie; see aocutils/README.md).
# These files are git-ignored. If the fetch fails, paste the input into input.txt yourself.
cd ..
if ! cargo aoc fetch "$1"; then
    touch "$1/input.txt"
    echo "Couldn't fetch day $1. Add the input to $1/input.txt yourself."
fi
