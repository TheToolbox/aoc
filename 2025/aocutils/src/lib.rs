use std::fs;
use std::env;
use std::process;

pub mod parsing;
pub mod grid;
pub mod point;
pub mod bits;
pub mod fetch;

/// Represents which part of the Advent of Code problem to solve
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    One,
    Two,
}

impl Part {
    /// Parse the part from a string (expects "1" or "2")
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            _ => Err(format!("Invalid part: '{}'. Expected '1' or '2'.", s)),
        }
    }
}

/// Result containing the problem part and input lines
pub struct AocInput {
    pub part: Part,
    pub lines: Vec<String>,
}

/// Parse command-line arguments and read input file
///
/// Usage: program <part> <input_file>
/// - part: "1" or "2" indicating which part of the problem to solve
/// - input_file: path to the input file
///
/// # Example
/// ```no_run
/// use AOCUtils::get_input;
///
/// fn main() {
///     let input = get_input();
///
///     match input.part {
///         AOCUtils::Part::One => println!("Solving part 1"),
///         AOCUtils::Part::Two => println!("Solving part 2"),
///     }
///
///     for line in input.lines {
///         // Process each line
///     }
/// }
/// ```
pub fn get_input() -> AocInput {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <part> <input_file>", args[0]);
        eprintln!("  part: 1 or 2");
        eprintln!("  input_file: path to input file");
        process::exit(1);
    }

    let part = match Part::from_str(&args[1]) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let filename = &args[2];
    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    AocInput { part, lines }
}

/// Get input with custom arguments (useful for testing)
pub fn get_input_from_args(part_str: &str, filename: &str) -> Result<AocInput, String> {
    let part = Part::from_str(part_str)?;

    let content = fs::read_to_string(filename)
        .map_err(|e| format!("Error reading file '{}': {}", filename, e))?;

    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    Ok(AocInput { part, lines })
}
