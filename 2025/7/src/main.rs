// Common aocutils imports:
// use aocutils::parsing::*;        // parse_csv, parse_space_separated, parse_digit_grid, etc.
// use aocutils::grid::Grid2D;      // Grid with neighbors_4/8, iter_cells, etc.
// use aocutils::point::Point;      // Point with manhattan_distance, neighbors, etc.
// use aocutils::bits::*;           // get_bit, set_bit, count_ones, etc.

use core::panic;

use aocutils::{AocInput, Part, get_input, grid::Grid2D, point::Point};

fn main() {
    let input = get_input();

    let start = Point {
        y: 0,
        x: input.lines[0].find("S").unwrap() as i64
    };

    let splitters = input.lines.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_col,c)| *c == '^')
                .map(move |(col,_c)| Splitter::new(row as i64, col as i64) )
        })
        .collect(); //chould already be sorted by height
    

    let manifold = Manifold::new(start, splitters);

    let result = match input.part {
        Part::One => solve_part_one(manifold),
        Part::Two => solve_part_two(input),
    };

    println!("{}", result);
}

fn solve_part_one(mut manifold: Manifold) -> u128 {
    println!("Solving part 1...");
    manifold.count_splits().into()
}

fn solve_part_two(mut input: AocInput) -> u128 {
    println!("Solving part 2...");

    //start at the bottom
    input.lines.reverse();

    let mut grid: Grid2D<Element> = Grid2D::new(vec![]);

    //load the manifold into a grid (instead of the fun manifold type I made for part one thinking it would be smart...)

    input.lines.iter().for_each(|line| {
        grid.data.push(line.chars().map(|c| 
            {
                match c {
                    '.' => Element::Blank(0),
                    '^' => Element::Splitter(0),
                    'S' => Element::Start,
                    x => panic!("invalid Manifold, found: {}", x),
                }
            }
        )
        .collect())
    });

    //first row is 1s
    for col in 0..grid.data[0].len() {
        grid.data[0][col] = Element::Blank(1);
    }

    for row in 1..grid.data.len() {
        
        for col in 0..grid.data[row].len() {
            let below = grid.data[row-1][col].get_value();

            let below_left = grid.data[row-1][col.saturating_sub(1)].get_value(); 

            let below_right = grid.data[row-1][(1+col).min(grid.data[row].len() - 1)].get_value();
            
            grid.data[row][col] = match grid.data[row][col] {
                Element::Blank(_) => Element::Blank(*below),
                Element::Splitter(_) => Element::Splitter(*below_left + *below_right),
                Element::Start => {
                    return *below_left + *below_right;
                }
            };
        }

        //println!("{:?}",grid.data[row]);

    }
   
    panic!("Reached the end of input without hitting the Start");
}

#[derive(Clone,Debug)]
enum Element {
    Blank(u128),
    Splitter(u128),
    Start,
}

impl Element {
    fn get_value(&self) -> &u128 {
        match self {
                Element::Blank(x) => x,
                Element::Splitter(x) => x,
                Element::Start => &0,
        }
    }
}

struct Splitter {
    location: Point,
    used: bool,
}

impl Splitter {
    fn new(row: i64, col: i64) -> Self {
        Splitter {
            location: Point {
                x: col,
                y: row,
            },
            used: false
        }
    }
}

struct Manifold {
    start: Point,
    splitters: Vec<Splitter>,
    height: i64,
}

impl Manifold {
    fn new(start: Point, splitters: Vec<Splitter>) -> Self {
        Manifold {
            start: start,
            height: splitters.iter().map(|splitter| splitter.location.y).max().unwrap(),
            splitters: splitters,
        }
    }

    fn count_splits(&mut self) -> u64 {
        //reset splitters
        for i in 0..self.splitters.len() {
            self.splitters[i].used = false;
        }

        self.count_from(self.start)

    }

    fn count_from(&mut self, point: Point) -> u64 {

        let maybe_splitter = self.splitters.iter_mut().find(|splitter| splitter.location == point);

        if point.y > self.height { return 0; }
        //point.y += 1; //down one row
        match maybe_splitter {
            Some(splitter) => {
                if splitter.used { 
                    0
                } else {
                    splitter.used = true;
                    1 + 
                    self.count_from(Point { x: point.x - 1, y: point.y}) + 
                    self.count_from(Point { x: point.x + 1, y: point.y})
                }
            },
            None => self.count_from(Point { x: point.x, y: point.y + 1})
        }
    }
}