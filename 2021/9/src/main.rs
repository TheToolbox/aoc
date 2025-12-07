use core::panic;
use std::fs;
use std::env;
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }

    let data = fs::read_to_string(&args[2])
        .unwrap();
    
    let map = HeightMap::new(data);
    let result = if args[1] == "part1" {
        map.risk_level()
    } else {
        let mut basins = map.basin_sizes();
        basins.sort_unstable();
        basins.reverse();
        basins[0..3].iter().product()
    };
    
    println!("{:?}",result);
}


#[derive(Clone)]
struct HeightMap(Vec<Vec<u64>>);

impl HeightMap {
    fn new(input: String) -> HeightMap {
        HeightMap(
            input.lines()
                .map(|line| 
                    line.chars()
                        .map(|c| u64::from(c.to_digit(10).unwrap()))
                        .collect()
                    )
                .collect()
        )    
    }

    fn risk_level(&self) -> u64 {
        self.lowest_points().iter()
            .map(|(x,y)| self.0[*x][*y] + 1)
            .sum()
    }

    fn lowest_points(&self) -> Vec<(usize, usize)> {
        let mut lowest_points = vec![];
        for (x, row) in self.0.iter().enumerate() {
            for (y, _value) in row.iter().enumerate() {
                if self.lower_than_neighbors(x, y) {
                    lowest_points.push((x,y));
                }
            }
        }
        lowest_points
    }

    fn lower_than_neighbors(&self, x: usize, y: usize) -> bool {
        self.0[x][y] < self.lowest_neighbor(x, y)
    }

    fn lowest_neighbor(&self, x: usize, y: usize) -> u64 {

        let left = if x > 0 {self.0[x - 1][y]} else { 9 };
        let right = if x < self.xmax() - 1 {self.0[x + 1][y]} else { 9 };
        let up = if y > 0 {self.0[x][y - 1]} else { 9 };
        let down = if y < self.ymax() - 1 {self.0[x][y + 1]} else { 9 };
        
        left.min(right.min(up.min(down)))
    }

    fn xmax(&self) -> usize { self.0.len() }
    fn ymax(&self) -> usize { self.0[0].len() }

    fn basin_sizes(&self) -> Vec<u64> {
        let mut scrap_map: HeightMap = self.clone();
        self.lowest_points().iter()
            .map(|(x,y)| scrap_map.color(*x, *y))
            .collect()
    }

    fn color(&mut self, x: usize, y: usize) -> u64 {
        if self.0[x][y] == 9 { return 0; }
        self.0[x][y] = 9;

        let mut sum = 1;
        if x > 0 {
            sum += self.color(x - 1, y);
        }
        if x < self.xmax() - 1 {
            sum += self.color(x + 1, y);
        }
        if y > 0 {
            sum += self.color(x, y - 1);
        }
        if y < self.ymax() - 1 {
            sum += self.color(x, y + 1);
        }

        sum
    }


}

impl fmt::Debug for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        self.0.iter().for_each(|row| {

                writeln!(f).unwrap();
                row.iter().for_each(|val|
                    write!(f, "{}", val).unwrap()
                )
            });
        
        write!(f, "")
    }
}