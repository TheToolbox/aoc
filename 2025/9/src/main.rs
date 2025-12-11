// Common aocutils imports:
// use aocutils::parsing::*;        // parse_csv, parse_space_separated, parse_digit_grid, etc.
// use aocutils::grid::Grid2D;      // Grid with neighbors_4/8, iter_cells, etc.
// use aocutils::point::Point;      // Point with manhattan_distance, neighbors, etc.
// use aocutils::bits::*;           // get_bit, set_bit, count_ones, etc.

use std::{ffi::os_str::Display, fmt::Debug};

use aocutils::{Part, get_input, point::{self, Point}};

fn main() {
    let input = get_input();

    let points : Vec<Point> = input.lines.iter()
        .map(|line| {
            let mut nums = line.split(",").map(|num| num.parse().unwrap());
            Point {
                x: nums.next().unwrap(),
                y: nums.next().unwrap()
            }
        })
        .collect();

    let result = match input.part {
        Part::One => solve_part_one(points),
        Part::Two => solve_part_two(points),
    };

    println!("{}", result);
}

fn solve_part_one(points: Vec<Point>) -> i64 {
    println!("Solving part 1...");

    let mut largest = 0;
    for i in 0..points.len() {
        for j in i..points.len() {
            let area = Rect::new(points[i],points[j]).area();
            largest = i64::max(area,largest);
        }
    }
    
    largest
}

fn solve_part_two(points: Vec<Point>) -> i64 {
    println!("Solving part 2...");

    //let grid = Grid::new(&points);
    //println!("{:?}",grid);

    let mut largest = 0;
    for i in 0..points.len() {
        for j in i..points.len() {
            let rect = Rect::new(points[i],points[j]);
            let area = rect.area();
            if !rect.contains_interior_points(&points) && area > largest {
                largest = i64::max(area,largest);
                println!("Solution: {:?} | {:?}",rect.corners.0, rect.corners.1);
            }
        }
    }
    
    largest
}

struct Rect {
    corners: (Point, Point)
}

impl Rect {
    fn new(p1: Point, p2: Point) -> Self {
        Self {
            corners: (p1,p2)
        }
    }

    fn area(&self) -> i64 {
        i64::abs((1 + self.corners.0.x - self.corners.1.x) * (1 + self.corners.0.y - self.corners.1.y))
    }

    fn contains_interior_points(&self, points: &Vec<Point>) -> bool {
        points.iter().any(|point| 
            self.within_interior(point)
        )
    }

    fn within_interior(&self, point: &Point) -> bool {
        point.x < self.corners.0.x.max(self.corners.1.x) &&
        point.x > self.corners.0.x.min(self.corners.1.x) &&
        point.y < self.corners.0.y.max(self.corners.1.y) &&
        point.y > self.corners.0.y.min(self.corners.1.y)
    }
}


struct Grid {
    tiles: Vec<Vec<Color>>,//[[Color; 100000]; 100000],
    chirality: Chirality,
}

impl Grid {
    fn new(points: &Vec<Point>) -> Grid {
        let mut me = Grid {
            tiles: vec![vec![Color::White;100000];100000],//[[Color::White; 100000]; 100000],
            chirality: Chirality::Clockwise
        };

        let mut last_dir: Direction = Direction::Up;
        let mut total_turn = 0;

        for pts in points.windows(2) {
            let point1 = pts[0];
            let point2 = pts[1];
           // println!("{:?}",me);
            //println!("{:?} to {:?}", point1, point2);

            me.draw_line(point1,point2,Color::Green);

            me.tiles[point1.x as usize][point1.y as usize] = Color::Red;
            me.tiles[point2.x as usize][point2.y as usize] = Color::Red;

            let dir = Direction::from(point1,point2);
            let chir = Chirality::from(&last_dir,&dir);
            last_dir = dir;

            total_turn += match chir {
                Chirality::Clockwise => 1,
                Chirality::CounterClockwise => -1,
                Chirality::Straight => 0,
                Chirality::Flip => 0,//2?
            }
        }

        {
            let point1 = points[points.len() - 1];
            let point2 = points[0];
            me.draw_line(point1,point2,Color::Green);

            me.tiles[point1.x as usize][point1.y as usize] = Color::Red;
            me.tiles[point2.x as usize][point2.y as usize] = Color::Red;

            let dir = Direction::from(point1,point2);
            let chir = Chirality::from(&last_dir,&dir);
            last_dir = dir;

            total_turn += match chir {
                Chirality::Clockwise => 1,
                Chirality::CounterClockwise => -1,
                Chirality::Straight => 0,
                Chirality::Flip => 0,//2?
            }
        }

        

        //println!("{:?}",me);
        println!("Total turns: {}", total_turn);
        me.chirality = if total_turn > 0 { Chirality::Clockwise } else { Chirality::CounterClockwise };

        // Fill interior
        let first_dir = Direction::from(points[0],points[1]);
        let x = points[0].x;
        let y = points[0].y;
        let flip = match me.chirality {
            Chirality::Clockwise => 1,
            Chirality::CounterClockwise => -1,
            _ => panic!(),
        };
        let starting_point : Point = match first_dir {
            Direction::Up    => Point::new(x + flip, y + 1),
            Direction::Right => Point::new(x + 1, y - flip),
            Direction::Down  => Point::new(x - flip, y - 1),
            Direction::Left  => Point::new(x - 1, y + flip),
        };

        println!("about to paint");

        me.paint_from(starting_point);

        println!("Green tiles: {}",me.tiles.iter().flatten().filter(|x| matches!(x, Color::Green)).count());

        me
    }

    fn draw_line(&mut self, point1: Point, point2: Point, color: Color) {
        if point1.x == point2.x {
            //draw along y
            let x = point1.x;
            let range = if point1.y > point2.y {
                point2.y..=point1.y
            } else {
                point1.y..=point2.y
            };
            for y in range {
                self.tiles[x as usize][y as usize] = color;
            }
        } else {
            //draw along x
            let y = point1.y;
            let range = if point1.x > point2.x {
                point2.x..=point1.x
            } else {
                point1.x..=point2.x
            };
            for x in range {
                self.tiles[x as usize][y as usize] = color;
            }
        }
    }

    fn paint_from(&mut self, point: Point) {

        let mut to_paint : Vec<Point> = vec![];
        to_paint.push(point);

        while let Some(paint_me) = to_paint.pop() {
            let x = paint_me.x as usize;
            let y = paint_me.y as usize;
            //println!("paint me: {:?} ({:?})",paint_me, self.tiles[x][y]);
            if matches!(self.tiles[x][y],Color::White) {
                //println!("Matched white and adding");
                self.tiles[x][y] = Color::Green;
                to_paint.push(Point{
                    x: (x + 1) as i64,
                    y: y as i64,
                });
                to_paint.push(Point{
                    x: (x - 1) as i64,
                    y: y as i64,
                });
                to_paint.push(Point{
                    x: x as i64,
                    y: (y + 1) as i64,
                });
                to_paint.push(Point{
                    x: x as i64,
                    y: (y - 1) as i64,
                });
                //println!("Now to_paint is: {:?}",to_paint);
            }
        }
    }

    fn _paint(&mut self) { //issues with horizontal borders (would have similar issues with vertical ones)
        for x in 0..self.tiles.len() {
            let mut painting = false;
            let mut flipped_last = false;

            for y in 0..self.tiles[x].len() {
                match self.tiles[x][y] {
                    Color::Green | Color::Red => if !flipped_last {
                        flipped_last = true;
                        painting = !painting
                    },
                    Color::White => if painting {
                        flipped_last = false;
                        self.tiles[x][y] = Color::Green;
                    },
                }
            }
        }
    }
    
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tiles.iter().for_each(|x| {
            x.iter().for_each(|y| {
                write!(f, "{:?}",y).unwrap();
            });
            write!(f,"\n").unwrap();
        });
        Ok(())
    }
}

#[derive(Copy,Clone)]
enum Color {
    Red,
    Green,
    White,
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Color::Red => "#",
            Color::Green => "X",
            Color::White => ".",
        })
    }
}

enum Chirality {
    Clockwise,
    CounterClockwise,
    Straight,
    Flip,
}

impl Chirality {
    fn from(dir1: &Direction, dir2: &Direction) -> Chirality {
        match (dir1,dir2) {
            (Direction::Left, Direction::Up) => Chirality::Clockwise,
            (Direction::Left, Direction::Down) => Chirality::CounterClockwise,
            (Direction::Left, Direction::Left) => Chirality::Straight,
            (Direction::Left, Direction::Right) => Chirality::Flip,

            (Direction::Right, Direction::Up) => Chirality::CounterClockwise,
            (Direction::Right, Direction::Down) => Chirality::Clockwise,
            (Direction::Right, Direction::Left) => Chirality::Flip,
            (Direction::Right, Direction::Right) => Chirality::Straight,

            (Direction::Up, Direction::Up) => Chirality::Straight,
            (Direction::Up, Direction::Down) => Chirality::Flip,
            (Direction::Up, Direction::Left) => Chirality::CounterClockwise,
            (Direction::Up, Direction::Right) => Chirality::Clockwise,

            (Direction::Down, Direction::Up) => Chirality::Flip,
            (Direction::Down, Direction::Down) => Chirality::Straight,
            (Direction::Down, Direction::Left) => Chirality::Clockwise,
            (Direction::Down, Direction::Right) => Chirality::CounterClockwise,
        }
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    fn from(point1: Point, point2: Point) -> Direction {
        if point1.x > point2.x {
            return Direction::Left;
        }
        if point1.x < point2.x {
            return Direction::Right;
        }
        if point1.y > point2.y {
            return Direction::Down;
        }
        
        Direction::Up
    }

}