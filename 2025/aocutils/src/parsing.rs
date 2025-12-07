use std::str::FromStr;

/// Parse a comma-separated line into a vector of values
///
/// # Example
/// ```
/// use AOCUtils::parsing::parse_csv;
/// let numbers: Vec<i64> = parse_csv("1,2,3,4,5");
/// assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
/// ```
pub fn parse_csv<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    s.split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

/// Parse a space-separated line into a vector of values
///
/// # Example
/// ```
/// use AOCUtils::parsing::parse_space_separated;
/// let numbers: Vec<i64> = parse_space_separated("1 2 3 4 5");
/// assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
/// ```
pub fn parse_space_separated<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    s.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

/// Parse lines into a vector of values
///
/// # Example
/// ```
/// use AOCUtils::parsing::parse_lines;
/// let lines = vec!["123", "456", "789"];
/// let numbers: Vec<i64> = parse_lines(&lines);
/// assert_eq!(numbers, vec![123, 456, 789]);
/// ```
pub fn parse_lines<T>(lines: &[String]) -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    lines.iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

/// Parse a grid of single-digit numbers into a 2D vector
///
/// # Example
/// ```
/// use AOCUtils::parsing::parse_digit_grid;
/// let input = "123\n456\n789";
/// let grid = parse_digit_grid(input);
/// assert_eq!(grid[0], vec![1, 2, 3]);
/// assert_eq!(grid[1], vec![4, 5, 6]);
/// ```
pub fn parse_digit_grid(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

/// Parse a binary string into a u64
///
/// # Example
/// ```
/// use AOCUtils::parsing::parse_binary;
/// let num = parse_binary("1010");
/// assert_eq!(num, 10);
/// ```
pub fn parse_binary(s: &str) -> u64 {
    u64::from_str_radix(s, 2).unwrap()
}

/// Parse a string with a custom delimiter
///
/// # Example
/// ```
/// use AOCUtils::parsing::parse_delimited;
/// let numbers: Vec<i64> = parse_delimited("1|2|3|4", "|");
/// assert_eq!(numbers, vec![1, 2, 3, 4]);
/// ```
pub fn parse_delimited<T>(s: &str, delimiter: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    s.split(delimiter)
        .map(|x| x.trim().parse().unwrap())
        .collect()
}
