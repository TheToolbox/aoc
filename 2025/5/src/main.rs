// Common aocutils imports:
// use aocutils::parsing::*;        // parse_csv, parse_space_separated, parse_digit_grid, etc.
// use aocutils::grid::Grid2D;      // Grid with neighbors_4/8, iter_cells, etc.
// use aocutils::point::Point;      // Point with manhattan_distance, neighbors, etc.
// use aocutils::bits::*;           // get_bit, set_bit, count_ones, etc.

use std::ops::{RangeInclusive};

use aocutils::{get_input, Part};
 
fn main() {
    let input = get_input();
    let mut lines = input.lines.iter();
    let mut fresh : Vec<IngredientRange> = lines.by_ref().take_while(|val| !val.is_empty())
        .map(|s| {
            let mut pieces = s.split('-').map(|num| num.parse::<usize>().unwrap());
            let start = pieces.next().unwrap();
            let end = pieces.next().unwrap();
            start..=end
        })
        .collect();

    let ingredients : Vec<usize> = lines.map(|line| line.parse().unwrap()).collect();

    let result = match input.part {
        Part::One => solve_part_one(fresh,ingredients),
        Part::Two => solve_part_two(&mut fresh),
    };

    println!("{}", result);
}

fn solve_part_one(fresh: Vec<IngredientRange>, ingredients: Vec<usize>) -> usize {
    println!("Solving part 1...");

    ingredients.into_iter()
        .filter(|ingredient| 
            fresh.iter().any(|fresh| fresh.contains(ingredient))
        )
        .count()
}

fn solve_part_two(fresh: &mut Vec<IngredientRange>) -> usize {
    println!("Solving part 2...");
    fresh.sort_by_key(|x| *x.start());

    while merge_once(fresh) {
        
    }

    fresh.sort_by_key(|x| *x.start());

    //println!("{:?}",fresh);

    fresh.iter()
        .map(|range| 1 + range.end() - range.start())
        .sum()
}


type IngredientRange = RangeInclusive<usize>;

fn merge_once(fresh: &mut Vec<IngredientRange>) -> bool {
    for i in 0..fresh.len() {
        for j in (i+1)..fresh.len() {
            if let Some(combined) = intersection(&fresh[i], &fresh[j]) {
                //println!("Intersection {:?} and {:?} => {:?}, removing.",fresh[i],fresh[j],combined);
                fresh.remove(j);
                fresh.remove(i);
                fresh.push(combined);
                return true;
            }
        }
    }
    false
}

fn intersection(x: &IngredientRange, y: &IngredientRange) -> Option<IngredientRange> {
    let (x1,x2) = (x.start(), x.end());
    let (y1,y2) = (y.start(), y.end());
    if (x1 <= y1 && x2 >= y1) || (y1 <= x1 && y2 >= x1) {
        Some(
            *x1.min(y1) ..= *x2.max(y2)
        )
    } else {
        None
    }
}

