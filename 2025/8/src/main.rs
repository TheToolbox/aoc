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

struct PointCloud {
    points: Vec<Point3D>
}

struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn distance(&self, other: &Point3D) -> i64 {
        i64::pow(i64::pow(self.x - other.x,2) + i64::pow(self.y - other.y,2) + i64::pow(self.z - other.z,2), 0.5)
    }
}

impl PointCloud {
    fn calculate_closest(&self, p: Point3d) {
        self.points.iter().min_by_key(|p2| p.distance(p2))

    }
}