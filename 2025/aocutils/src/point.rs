use std::str::FromStr;

/// A 2D point with integer coordinates
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    /// Create a new point
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    /// Parse a point from a string with a custom delimiter
    ///
    /// # Example
    /// ```
    /// use AOCUtils::point::Point;
    /// let p = Point::from_str_with_delimiter("3,5", ",");
    /// assert_eq!(p, Point::new(3, 5));
    /// ```
    pub fn from_str_with_delimiter(s: &str, delimiter: &str) -> Self {
        let parts: Vec<&str> = s.split(delimiter).collect();
        Point {
            x: parts[0].trim().parse().unwrap(),
            y: parts[1].trim().parse().unwrap(),
        }
    }

    /// Get all 4-directional neighbors (up, down, left, right)
    pub fn neighbors_4(&self) -> Vec<Point> {
        vec![
            Point::new(self.x, self.y - 1), // Up
            Point::new(self.x, self.y + 1), // Down
            Point::new(self.x - 1, self.y), // Left
            Point::new(self.x + 1, self.y), // Right
        ]
    }

    /// Get all 8-directional neighbors (including diagonals)
    pub fn neighbors_8(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y - 1), // Upper-left
            Point::new(self.x, self.y - 1),     // Up
            Point::new(self.x + 1, self.y - 1), // Upper-right
            Point::new(self.x - 1, self.y),     // Left
            Point::new(self.x + 1, self.y),     // Right
            Point::new(self.x - 1, self.y + 1), // Lower-left
            Point::new(self.x, self.y + 1),     // Down
            Point::new(self.x + 1, self.y + 1), // Lower-right
        ]
    }

    /// Calculate Manhattan distance to another point
    pub fn manhattan_distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Calculate Euclidean distance squared (to avoid floating point)
    pub fn distance_squared(&self, other: &Point) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Add two points (vector addition)
    pub fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    /// Subtract two points (vector subtraction)
    pub fn sub(&self, other: &Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }

    /// Scale a point by a scalar
    pub fn scale(&self, scalar: i64) -> Point {
        Point::new(self.x * scalar, self.y * scalar)
    }
}

impl FromStr for Point {
    type Err = String;

    /// Parse a point from "x,y" format by default
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Point::from_str_with_delimiter(s, ","))
    }
}

/// Standalone Manhattan distance function
pub fn manhattan_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}
