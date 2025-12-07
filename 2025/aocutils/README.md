# AOCUtils

A comprehensive utility library for Advent of Code solutions in Rust, designed to eliminate boilerplate and common patterns.

## Features

- **Input Handling**: Parse command-line arguments and read input files
- **Parsing Utilities**: CSV, space-separated, digit grids, binary strings, and more
- **Grid Operations**: 2D grid structure with neighbor finding and iteration
- **Point/Coordinate Math**: Manhattan distance, vector operations, neighbor finding
- **Bit Manipulation**: Common bit operations (get, set, clear, toggle, count)

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
AOCUtils = { path = "../AOCUtils" }
```

Basic usage:

```rust
use AOCUtils::{get_input, Part};

fn main() {
    let input = get_input();

    match input.part {
        Part::One => solve_part_one(&input.lines),
        Part::Two => solve_part_two(&input.lines),
    }
}

fn solve_part_one(lines: &[String]) {
    // Your solution for part 1
}

fn solve_part_two(lines: &[String]) {
    // Your solution for part 2
}
```

Run your solution:

```bash
cargo run 1 input.txt  # Solve part 1
cargo run 2 input.txt  # Solve part 2
```

## Modules

### Input Handling

```rust
use AOCUtils::{get_input, Part};

let input = get_input();
// input.part: Part::One or Part::Two
// input.lines: Vec<String>
```

### Parsing Module

```rust
use AOCUtils::parsing::*;

// Parse comma-separated values
let numbers: Vec<i64> = parse_csv("1,2,3,4,5");

// Parse space-separated values
let values: Vec<i64> = parse_space_separated("10 20 30");

// Parse lines into numbers
let nums: Vec<i64> = parse_lines(&lines);

// Parse a grid of single digits
let grid: Vec<Vec<u32>> = parse_digit_grid("123\n456\n789");

// Parse binary strings
let num = parse_binary("1010"); // Returns 10

// Parse with custom delimiter
let nums: Vec<i64> = parse_delimited("1|2|3", "|");
```

### Grid Module

```rust
use AOCUtils::grid::Grid2D;

// Create from digit string
let grid = Grid2D::from_digit_string("123\n456\n789");

// Create from character string
let grid = Grid2D::from_char_string("abc\ndef\nghi");

// Access values
if let Some(value) = grid.get(0, 0) {
    println!("Top-left: {}", value);
}

// Get dimensions
let (height, width) = grid.dimensions();

// Get neighbors (4-directional)
let neighbors = grid.neighbors_4(1, 1);
for (row, col) in neighbors {
    println!("Neighbor at ({}, {})", row, col);
}

// Get neighbors (8-directional, including diagonals)
let neighbors = grid.neighbors_8(1, 1);

// Get neighbor values
let values = grid.neighbor_values_4(1, 1);

// Iterate over all cells
for (row, col, value) in grid.iter_cells() {
    println!("({}, {}) = {}", row, col, value);
}

// Print grid
println!("{}", grid);
```

### Point Module

```rust
use AOCUtils::point::Point;

// Create points
let p1 = Point::new(3, 5);
let p2 = Point::from_str_with_delimiter("10,20", ",");

// Get neighbors
let neighbors_4 = p1.neighbors_4(); // Up, down, left, right
let neighbors_8 = p1.neighbors_8(); // Including diagonals

// Calculate distances
let manhattan = p1.manhattan_distance(&p2);
let dist_sq = p1.distance_squared(&p2);

// Vector operations
let sum = p1.add(&p2);
let diff = p1.sub(&p2);
let scaled = p1.scale(3);

// Standalone function
let dist = AOCUtils::point::manhattan_distance(0, 0, 3, 4); // Returns 7
```

### Bits Module

```rust
use AOCUtils::bits::*;

let num = 0b1010;

// Get specific bit
let bit = get_bit(num, 1); // Returns 1

// Set bit
let new_num = set_bit(num, 2); // 0b1110

// Clear bit
let new_num = clear_bit(num, 1); // 0b1000

// Toggle bit
let new_num = toggle_bit(num, 2); // 0b1110

// Count bits
let ones = count_ones(num); // Returns 2
let zeros = count_zeros(num, 4); // Returns 2

// Check if bit is set
if is_bit_set(num, 1) {
    println!("Bit 1 is set");
}

// Extract bit range
let bits = extract_bits(0b11010, 1, 3); // Returns 0b101

// Reverse bits
let reversed = reverse_bits(0b1010, 4); // Returns 0b0101
```

## Common Patterns

### Parse input as numbers

```rust
use AOCUtils::parsing::parse_lines;

let input = get_input();
let numbers: Vec<i64> = parse_lines(&input.lines);
```

### Parse CSV line

```rust
use AOCUtils::parsing::parse_csv;

let input = get_input();
let first_line = &input.lines[0];
let numbers: Vec<i64> = parse_csv(first_line);
```

### Work with digit grid

```rust
use AOCUtils::grid::Grid2D;

let input = get_input();
let grid_str = input.lines.join("\n");
let grid = Grid2D::from_digit_string(&grid_str);

for (row, col, value) in grid.iter_cells() {
    // Process each cell
    let neighbors = grid.neighbors_4(row, col);
    // ...
}
```

### Parse coordinate pairs

```rust
use AOCUtils::point::Point;

let input = get_input();
let points: Vec<Point> = input.lines
    .iter()
    .map(|line| Point::from_str_with_delimiter(line, ","))
    .collect();
```

## API Reference

### Core Types

**`Part` enum**
```rust
pub enum Part {
    One,
    Two,
}
```

**`AocInput` struct**
```rust
pub struct AocInput {
    pub part: Part,
    pub lines: Vec<String>,
}
```

**`Grid2D<T>` struct**
```rust
pub struct Grid2D<T> {
    pub data: Vec<Vec<T>>,
}
```

**`Point` struct**
```rust
pub struct Point {
    pub x: i64,
    pub y: i64,
}
```

### Functions

See inline documentation and examples above for detailed function usage.

## Design Philosophy

This library is designed based on patterns from real AoC solutions to:
- Eliminate repetitive boilerplate
- Provide safe, ergonomic APIs
- Focus on the most common use cases
- Keep solutions focused on problem-solving logic

Enjoy your Advent of Code!
