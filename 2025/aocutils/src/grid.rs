use std::fmt;

/// A 2D grid structure with convenience methods for common operations
#[derive(Clone, Debug)]
pub struct Grid2D<T> {
    pub data: Vec<Vec<T>>,
}

impl<T: Clone> Grid2D<T> {
    /// Create a new grid from a 2D vector
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Grid2D { data }
    }

    /// Get the width (number of columns) of the grid
    pub fn width(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data[0].len()
        }
    }

    /// Get the height (number of rows) of the grid
    pub fn height(&self) -> usize {
        self.data.len()
    }

    /// Get dimensions as (height, width)
    pub fn dimensions(&self) -> (usize, usize) {
        (self.height(), self.width())
    }

    /// Safely get a value at the given coordinates
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row).and_then(|r| r.get(col))
    }

    /// Safely get a mutable reference to a value at the given coordinates
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut(row).and_then(|r| r.get_mut(col))
    }

    /// Get all valid 4-directional neighbors (up, down, left, right)
    pub fn neighbors_4(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let height = self.height();
        let width = self.width();

        // Up
        if row > 0 {
            neighbors.push((row - 1, col));
        }
        // Down
        if row < height - 1 {
            neighbors.push((row + 1, col));
        }
        // Left
        if col > 0 {
            neighbors.push((row, col - 1));
        }
        // Right
        if col < width - 1 {
            neighbors.push((row, col + 1));
        }

        neighbors
    }

    /// Get all valid 8-directional neighbors (including diagonals)
    pub fn neighbors_8(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let height = self.height();
        let width = self.width();

        for dr in -1i32..=1 {
            for dc in -1i32..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }

                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                if new_row >= 0 && new_row < height as i32
                    && new_col >= 0 && new_col < width as i32 {
                    neighbors.push((new_row as usize, new_col as usize));
                }
            }
        }

        neighbors
    }

    /// Get neighbor values (4-directional)
    pub fn neighbor_values_4(&self, row: usize, col: usize) -> Vec<&T> {
        self.neighbors_4(row, col)
            .iter()
            .filter_map(|(r, c)| self.get(*r, *c))
            .collect()
    }

    /// Get neighbor values (8-directional)
    pub fn neighbor_values_8(&self, row: usize, col: usize) -> Vec<&T> {
        self.neighbors_8(row, col)
            .iter()
            .filter_map(|(r, c)| self.get(*r, *c))
            .collect()
    }

    /// Iterate over all cells with their coordinates
    pub fn iter_cells(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data.iter().enumerate().flat_map(|(row, row_data)| {
            row_data.iter().enumerate().map(move |(col, value)| (row, col, value))
        })
    }
}

impl<T: fmt::Display> fmt::Display for Grid2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Create a Grid2D from a string of single-digit numbers
///
/// # Example
/// ```
/// use AOCUtils::grid::Grid2D;
/// let input = "123\n456\n789";
/// let grid = Grid2D::from_digit_string(input);
/// assert_eq!(grid.get(0, 0), Some(&1));
/// assert_eq!(grid.get(1, 1), Some(&5));
/// ```
impl Grid2D<u32> {
    pub fn from_digit_string(input: &str) -> Self {
        let data = input.lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();
        Grid2D::new(data)
    }
}

/// Create a Grid2D from a string of characters
impl Grid2D<char> {
    pub fn from_char_string(input: &str) -> Self {
        let data = input.lines()
            .map(|line| line.chars().collect())
            .collect();
        Grid2D::new(data)
    }
}
