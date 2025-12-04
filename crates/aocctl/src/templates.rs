pub const TEMPLATE_FAST: &str = r#"
use aoc{YEAR}::prelude::*;

/// Fast parser using &str slices, no extra heap allocations.
fn parse(input: &str) -> Vec<&str> {
    input.trim_end().split('\n').collect()
}

/// Part 1 working directly on &str slices.
fn part_one(data: &[&str]) -> i64 {
    let _ = data;
    0
}

/// Part 2.
fn part_two(data: &[&str]) -> i64 {
    let _ = data;
    0
}

fn main() {
    use std::time::Instant;

    let data = read_input();

    let start = Instant::now();
    println!("Part 1: {}", part_one(&data));
    println!("Elapsed time: {:.4} seconds", start.elapsed().as_secs_f64());

    let start = Instant::now();
    println!("Part 2: {}", part_two(&data));
    println!("Elapsed time: {:.4} seconds", start.elapsed().as_secs_f64());
}
"#;

pub const TEMPLATE_STREAMING: &str = r#"
use aoc{YEAR}::prelude::*;

/// Read input line-by-line in a streaming fashion.
///
/// The returned iterator owns all relevant data.
fn input_lines() -> Box<dyn Iterator<Item = String>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path).expect("Cannot open file");
        let reader = BufReader::new(file);
        return Box::new(reader.lines().filter_map(Result::ok));
    }

    let text = read_input();
    Box::new(text.lines().map(|s| s.to_string()))
}

/// Solve part 1 using a streaming iterator.
fn part_one<I: Iterator<Item = String>>(lines: I) -> i64 {
    let _ = lines;
    0
}

/// Solve part 2 using a streaming iterator.
fn part_two<I: Iterator<Item = String>>(lines: I) -> i64 {
    let _ = lines;
    0
}

fn main() {
    use std::time::Instant;

    let data = read_input();

    let start = Instant::now();
    println!("Part 1: {}", part_one(&data));
    println!("Elapsed time: {:.4} seconds", start.elapsed().as_secs_f64());

    let start = Instant::now();
    println!("Part 2: {}", part_two(&data));
    println!("Elapsed time: {:.4} seconds", start.elapsed().as_secs_f64());
}
"#;
pub const TEMPLATE_BUFFERED: &str = r#"
use aoc{YEAR}::prelude::*;

/// Load input from file (if provided) or from stdin.
///
/// - `cargo run --bin d01 input.txt` → file mode
/// - `aocctl run` → stdin mode
fn input() -> String {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path).expect("Failed to open input file");
        let reader = BufReader::new(file);
        return reader
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
            .join("\n");
    }

    read_input()
}

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

fn part_one(data: &[String]) -> i64 {
    let _ = data;
    0
}

fn part_two(data: &[String]) -> i64 {
    let _ = data;
    0
}

fn main() {
    use std::time::Instant;

    let data = read_input();

    let start = Instant::now();
    println!("Part 1: {}", part_one(&data));
    println!("Elapsed time: {:.4} seconds", start.elapsed().as_secs_f64());

    let start = Instant::now();
    println!("Part 2: {}", part_two(&data));
    println!("Elapsed time: {:.4} seconds", start.elapsed().as_secs_f64());
}
"#;
pub const TEMPLATE_MINIMAL: &str = r#"
use aoc{YEAR}::prelude::*;

/// Domain-specific parser.
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

/// Solve part 1.
fn part_one(data: &[String]) -> i64 {
    let _ = data;
    0
}

/// Solve part 2.
fn part_two(data: &[String]) -> i64 {
    let _ = data;
    0
}

fn main() {
    use std::time::Instant;

    let input = read_input();
    let data = parse(&input);

    let start = Instant::now();

    println!("Part 1: {}", part_one(&data));
    println!("Part 2: {}", part_two(&data));

    println!("Elapsed time: {:.4} s", start.elapsed().as_secs_f64());
}
"#;
