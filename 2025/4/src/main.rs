// Common aocutils imports:
// use aocutils::parsing::*;        // parse_csv, parse_space_separated, parse_digit_grid, etc.
// use aocutils::grid::Grid2D;      // Grid with neighbors_4/8, iter_cells, etc.
// use aocutils::point::Point;      // Point with manhattan_distance, neighbors, etc.
// use aocutils::bits::*;           // get_bit, set_bit, count_ones, etc.

use core::panic;

use aocutils::{Part, get_input, grid::Grid2D};

fn main() {
    let input = get_input();

    let grid: Grid2D<Space> = Grid2D::new(
            input.lines.into_iter()
                    .map(|s| s.chars().map(|c| c.into()).collect::<Vec<Space>>())
                    .collect()
    );

    let result: usize = match input.part {
        Part::One => solve_part_one(grid),
        Part::Two => solve_part_two(grid),
    };

    println!("{}", result);
}

fn solve_part_one(grid: Grid2D<Space>) -> usize {
    println!("Solving part 1...");

    grid.iter_cells()
        .filter(|(row,col,val)| {
            **val == Space::Full && grid.neighbor_values_8(*row, *col).into_iter().map(|space| *space as usize).sum::<usize>() < 4
        })
        .count()
}

fn solve_part_two(grid: Grid2D<Space>) -> usize {
    println!("Solving part 2...");

    remove_rolls_recursive(grid)
}

#[derive(Clone, PartialEq, Copy)]
enum Space {
    Full = 1,
    Empty = 0,
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '@' => Self::Full,
            '.' => Self::Empty,
            _ => panic!("uh oh parsing issue"),
        }
    }
}

fn remove_rolls_recursive(grid: Grid2D<Space>) -> usize {
    let mut count = 0;
    let mut new_grid = grid.clone();

    for row in 0..grid.data.len() {
        for col in 0..grid.data[row].len() {
            if grid.data[row][col] == Space::Full && grid.neighbor_values_8(row, col).into_iter().map(|s| *s as usize).sum::<usize>() < 4 {
                new_grid.data[row][col] = Space::Empty;
                count += 1;
                println!("({},{})",row,col);
            }
        }
    }
    if count == 0 { 
        0
    } else {
        count + remove_rolls_recursive(new_grid)
    }
}

