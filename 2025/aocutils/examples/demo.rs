use AOCUtils::{get_input, Part};
use AOCUtils::parsing::*;
use AOCUtils::grid::Grid2D;
use AOCUtils::point::Point;
use AOCUtils::bits::*;

fn main() {
    println!("=== AOCUtils Demo ===\n");

    // Example 1: Parsing utilities
    println!("1. Parsing utilities:");
    let csv_data = "1,2,3,4,5";
    let numbers: Vec<i64> = parse_csv(csv_data);
    println!("   CSV parse: {} -> {:?}", csv_data, numbers);

    let space_data = "10 20 30 40";
    let nums: Vec<i64> = parse_space_separated(space_data);
    println!("   Space-separated: {} -> {:?}", space_data, nums);

    let binary = "1010";
    let num = parse_binary(binary);
    println!("   Binary parse: {} -> {}\n", binary, num);

    // Example 2: Grid operations
    println!("2. Grid operations:");
    let grid_str = "123\n456\n789";
    let grid = Grid2D::from_digit_string(grid_str);
    println!("   Grid:\n{}", grid);
    println!("   Dimensions: {:?}", grid.dimensions());
    println!("   Center value: {:?}", grid.get(1, 1));
    let neighbors = grid.neighbors_4(1, 1);
    println!("   4-neighbors of center: {:?}\n", neighbors);

    // Example 3: Point operations
    println!("3. Point operations:");
    let p1 = Point::new(0, 0);
    let p2 = Point::new(3, 4);
    println!("   Point 1: {:?}", p1);
    println!("   Point 2: {:?}", p2);
    println!("   Manhattan distance: {}", p1.manhattan_distance(&p2));
    println!("   4-neighbors of p1: {:?}\n", p1.neighbors_4());

    // Example 4: Bit operations
    println!("4. Bit operations:");
    let value = 0b1010;
    println!("   Value: {:b}", value);
    println!("   Bit 1: {}", get_bit(value, 1));
    println!("   Bit count: {}", count_ones(value));
    println!("   Set bit 2: {:b}", set_bit(value, 2));
    println!("   Toggle bit 0: {:b}\n", toggle_bit(value, 0));

    println!("=== Ready for Advent of Code! ===");
}
