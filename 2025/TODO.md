# AoC Utilities To-Do

Prep work for next year's Advent of Code, based on patterns repeated across the 2021 and 2025 solutions.

## 1. Fix what's broken first

These stop `cargo build --workspace` and `cargo test` from running cleanly. Do them before adding anything new.

- [ ] Rename `AOCUtils::` to `aocutils::` in the doc comments and README, so the doctests compile and run
- [ ] Make `setup_new_day.sh` add the new day to `members` in `2025/Cargo.toml`
- [ ] Fix day 8 so it compiles (the undefined `Point3d`, and `i64::pow(…, 0.5)`), or comment it out until you finish it
- [ ] Restore day 6 (lost in the commit "accidentally deleted 6") or take it out of the plan
- [ ] Move day 1 onto `get_input()` instead of the hardcoded `let part = 2;`

## 2. A better runner

This saves the most time, because every day goes through it.

- [ ] Run both parts by default when no part is given
- [ ] Print the time each part took
- [ ] Add an `--example` flag that reads `example.txt` (2021 had `testinput.txt` in 7 days; 2025 has none)
- [ ] Make the setup script create an empty `example.txt`
- [ ] Add a `#[test]` in the template that checks each part's example answer, like the 2019 JS `TestFile` helper did
- [ ] Add an `aoc fetch <day>` command that downloads the input (and puzzle text into `problem_part_*.md`) using your session cookie
- [ ] Change the template's "Solving part 1..." lines to `eprintln!` so stdout carries only the answer

## 3. Grid and Direction

Days 4, 7 and 9 each built or scanned a grid by hand. Day 9 also hand-wrote a `Direction` enum and a 16-case `Chirality` table.

**Grid2D**

- [ ] `Grid2D::from_lines(&lines, |c| T)` to build a grid from a per-character mapping (days 4 and 7)
- [ ] `find(&T)` and `positions_of(&T)` (day 7 searched for `S` and `^` by hand)
- [ ] `set(...)` and `in_bounds(i64, i64)`
- [ ] Index the grid by `Point`, so you stop converting between `(row, col)` as `usize` and `x`/`y` as `i64`
- [ ] Fix `neighbors_4` and `neighbors_8`: `height - 1` underflows on an empty grid

**Direction**

- [ ] A `Direction` enum (`Up`/`Down`/`Left`/`Right`, optionally diagonals)
- [ ] `turn_left()`, `turn_right()`, `opposite()`, `delta() -> Point`
- [ ] Parse from `^v<>`, `UDLR` and `NSEW`
- [ ] Implement `Point + Direction` and `Point * n`

## 4. Graph search

You've written this several times: day 9's `paint_from` flood fill, the 2021 day 9 basins, the 2021 day 12 paths. Have each one take a closure for neighbors, so it works on grids and on graphs.

- [ ] `bfs(start, neighbors_fn, goal_fn)` that returns the distance (and optionally the path)
- [ ] `dijkstra(start, weighted_neighbors_fn, goal_fn)`
- [ ] `flood_fill(start, passable_fn) -> HashSet<Point>`
- [ ] A `UnionFind` with `find`, `union` and component sizes (day 8's closest-points problem is a natural fit)

## 5. Intervals and parsing

Days 2 and 5 both parsed `a-b` ranges by hand, and day 5's merge loop is O(n³).

**Intervals**

- [ ] An `Interval` type that parses `a-b`
- [ ] `contains`, `len` and `intersect`
- [ ] `merge_all(Vec<Interval>)`: sort by start, then merge in one pass

**Parsing**

- [ ] `ints(line) -> Vec<i64>`: pull every signed number out of a line, whatever its format (would have covered days 2, 5, 8 and 9)
- [ ] `sections(&lines)`: split the input on blank lines (day 5 used `take_while(!is_empty)`)

## 6. Math, 3D points and cycles

You haven't needed most of these yet, but they come up nearly every year, often late in December.

- [ ] A `Point3` with `distance_squared` (what day 8 needed)
- [ ] `gcd` and `lcm`
- [ ] `divisors(n)` (day 2 wrote its own `factors`)
- [ ] `digit_count(n)`
- [ ] A Chinese Remainder Theorem solver
- [ ] `find_cycle(state, step_fn) -> (start, length)` using a `HashMap` of seen states, for "simulate a billion steps" puzzles
