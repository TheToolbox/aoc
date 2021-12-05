use core::panic;
use std::fmt;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }

    let data = fs::read_to_string(&args[2])
        .unwrap();
    let lines = data.lines();
    
    let mut grid = Grid::new();



    
    for line in lines {
        let line = new_line_segment(line);
        grid.plot(line);
        //println!("{}",grid)
    }
    
    println!("{}",grid.score());
}


#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(str: &str) -> Point {
        let splat = str.trim().split_once(',').unwrap();
        Point { 
            x: splat.0.parse().unwrap(), 
            y: splat.1.parse().unwrap(),
        }
    }
}

type LineSegment = (Point, Point);

fn new_line_segment(str: &str) -> LineSegment {
    let point_strings = str.split_once("->").unwrap();
    (Point::new(point_strings.0), Point::new(point_strings.1))       
}

fn horizontal(line: &LineSegment) -> bool {
    line.0.y == line.1.y
}

fn vertical(line: &LineSegment) -> bool {
    line.0.x == line.1.x
}

struct Grid {
    area: Vec<Vec<u64>>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            area: vec![vec![0; 1000]; 1000],
        }
    }

    fn plot(&mut self, line: LineSegment) {
        println!("Plotting {:?} to {:?}",line.0,line.1);

        let xstart: usize = u64::min(line.0.x, line.1.x).try_into().unwrap();
        let xend: usize = (u64::max(line.0.x, line.1.x) + 1).try_into().unwrap();
        let ystart: usize = u64::min(line.0.y, line.1.y).try_into().unwrap();
        let yend: usize = (u64::max(line.0.y, line.1.y) + 1).try_into().unwrap();

        if horizontal(&line) || vertical(&line) {
            for x in xstart..xend {
                let x: usize = x;
                for y in ystart..yend {
                    let y: usize = y;
                    self.area[x][y] += 1;
                }
            }
        } else {
            let (left,right) = if line.0.x < line.1.x {
                (line.0, line.1)
            } else {
                (line.1, line.0)
            };


            let length: usize = (1 + right.x - left.x).try_into().unwrap();

            let startx: usize = left.x.try_into().unwrap();
            let starty: usize = left.y.try_into().unwrap();

            for z in 0..length {
                if left.y < right.y {
                    self.area[startx + z][starty + z] += 1;
                } else {
                    self.area[startx + z][starty - z] += 1;
                };

                
            }
        }
    }

    fn score(&self) -> u64 {
        self.area.iter()
            .flatten()
            .map(|x| if x > &1 {1} else {0})
            .sum()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.area {
            for item in row.iter().rev() {
                if item < &1 {
                    write!(f, ".")?
                } else {
                    write!(f, "{}",item)?
                }
            }
            writeln!(f)?;
        }        
        
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f,"")
    }
}