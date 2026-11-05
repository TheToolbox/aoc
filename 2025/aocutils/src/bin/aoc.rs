//! `aoc` command-line tool. Run it from the year's folder (e.g. `2025/`):
//!
//!     cargo aoc fetch 10            # input.txt and example_1.txt (and example_2.txt once part 1 is solved)
//!     cargo aoc fetch 10 --force    # download input.txt again even if it exists
//!     cargo aoc fetch 10 --year 2024
//!
//! It needs your adventofcode.com `session` cookie in `AOC_SESSION`,
//! `~/.config/aoc/session`, or `.aoc-session` (all git-ignored or outside the repo).
//! Downloads are written into `<day>/` and are git-ignored: the AoC creator
//! asks people not to publish inputs or puzzle text.

use std::env;
use std::fs;
use std::path::Path;
use std::process;

use aocutils::fetch::{extract_example, find_session, split_parts, year_from_dir_name};

const USER_AGENT: &str = "github.com/TheToolbox/aoc aoc-fetch (aocutils)";

fn usage() -> ! {
    eprintln!("Usage: cargo aoc fetch <day> [--year YYYY] [--force]");
    process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.first().map(String::as_str) != Some("fetch") {
        usage();
    }

    let mut day: Option<u32> = None;
    let mut year: Option<u32> = None;
    let mut force = false;
    let mut it = args.iter().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--force" => force = true,
            "--year" => year = it.next().and_then(|y| y.parse().ok()).or_else(|| usage()),
            _ => day = Some(a.parse().unwrap_or_else(|_| usage())),
        }
    }
    let Some(day) = day.filter(|d| (1..=25).contains(d)) else { usage() };

    let year = year.or_else(|| {
        env::current_dir()
            .ok()
            .and_then(|d| d.file_name().map(|n| n.to_string_lossy().into_owned()))
            .and_then(|n| year_from_dir_name(&n))
    });
    let Some(year) = year else {
        eprintln!("Can't tell the year from this folder's name. Pass --year YYYY.");
        process::exit(2);
    };

    let Some(session) = find_session() else {
        eprintln!(
            "No session cookie found. Copy the `session` cookie from adventofcode.com \
             into AOC_SESSION, ~/.config/aoc/session or .aoc-session."
        );
        process::exit(1);
    };

    let dir = Path::new(&day.to_string()).to_path_buf();
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Can't create {}: {e}", dir.display());
        process::exit(1);
    }

    let base = format!("https://adventofcode.com/{year}/day/{day}");
    let mut ok = true;

    // Puzzle input. Only downloaded once unless --force, to go easy on the server.
    let input_path = dir.join("input.txt");
    let have_input = fs::metadata(&input_path).map(|m| m.len() > 0).unwrap_or(false);
    if have_input && !force {
        println!("{} already exists (use --force to download again)", input_path.display());
    } else {
        match get(&format!("{base}/input"), &session) {
            Ok(body) => {
                write(&input_path, &body);
            }
            Err(e) => {
                eprintln!("Couldn't download the input: {e}");
                ok = false;
            }
        }
    }

    // Examples from the puzzle page. Part 2 only shows once part 1 is solved,
    // so running this again later picks it up.
    match get(&base, &session) {
        Ok(html) => {
            let parts = split_parts(&html);
            let examples: Vec<Option<String>> = parts.iter().map(|p| extract_example(p)).collect();
            match examples.first() {
                Some(Some(ex)) => write(&dir.join("example_1.txt"), ex),
                _ => eprintln!("No example found in part 1's text."),
            }
            if let Some(Some(ex2)) = examples.get(1) {
                if examples.first().and_then(|e| e.as_ref()) != Some(ex2) {
                    write(&dir.join("example_2.txt"), ex2);
                } else {
                    println!("Part 2 reuses part 1's example.");
                }
            } else if parts.len() < 2 {
                println!("Part 2 isn't unlocked yet; run this again after solving part 1.");
            }
        }
        Err(e) => {
            eprintln!("Couldn't download the puzzle page: {e}");
            ok = false;
        }
    }

    if !ok {
        process::exit(1);
    }
}

fn get(url: &str, session: &str) -> Result<String, String> {
    let resp = ureq::get(url)
        .set("User-Agent", USER_AGENT)
        .set("Cookie", &format!("session={session}"))
        .call();
    match resp {
        Ok(r) => r.into_string().map_err(|e| e.to_string()),
        Err(ureq::Error::Status(code, r)) => {
            let body = r.into_string().unwrap_or_default();
            let hint = match code {
                400 | 401 => " (is the session cookie current?)",
                404 => " (is that day unlocked yet?)",
                _ => "",
            };
            Err(format!("HTTP {code}{hint}: {}", body.trim()))
        }
        Err(e) => Err(e.to_string()),
    }
}

fn write(path: &Path, contents: &str) {
    match fs::write(path, contents) {
        Ok(()) => println!("Wrote {}", path.display()),
        Err(e) => {
            eprintln!("Couldn't write {}: {e}", path.display());
            process::exit(1);
        }
    }
}
