use aoc2025::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
}

const DIAL_SIZE: i32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rotation(Direction, i32);
impl Rotation {
    pub const fn direction(&self) -> Direction { self.0 }

    pub const fn distance(&self) -> i32 { self.1 }
}

impl core::str::FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buffer = s.trim().as_bytes();
        if buffer.is_empty() {
            return Err(ParseRotationError::Empty);
        }

        let direction = match buffer[0] {
            b'L' | b'l' => Direction::Left,
            b'R' | b'r' => Direction::Right,
            c => return Err(ParseRotationError::InvalidDirection(c)),
        };

        let distance = str::from_utf8(&buffer[1..])
            .map_err(|_| ParseRotationError::InvalidDistance)?
            .parse()
            .map_err(|_| ParseRotationError::InvalidDistance)?;

        Ok(Self(direction, distance))
    }
}

fn part1(input: impl AsRef<str>) -> i32 {
    let rotations = input
        .as_ref()
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Rotation>>();

    let (_, count) = rotations.iter().fold((50, 0), |(position, count), rot| {
        let position = match rot.direction() {
            Direction::Left => (position - rot.distance()).rem_euclid(DIAL_SIZE),
            Direction::Right => (position + rot.distance()).rem_euclid(DIAL_SIZE),
        };

        let count = if position == 0 { count + 1 } else { count };

        (position, count)
    });

    count
}

fn part2(input: impl AsRef<str>) -> i32 {
    let rotations = input
        .as_ref()
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Rotation>>();

    let (_, count) = rotations.iter().fold((50, 0), |(position, count), rot| {
        // Count how many times we cross 0 during this rotation
        let crossings = count_zero_crossings(position, rot.direction(), rot.distance());
        let position = match rot.direction() {
            Direction::Left => (position - rot.distance()).rem_euclid(DIAL_SIZE),
            Direction::Right => (position + rot.distance()).rem_euclid(DIAL_SIZE),
        };
        (position, count + crossings)
    });

    count
}

fn count_zero_crossings(start: i32, direction: Direction, steps: i32) -> i32 {
    // Steps required to reach zero from the current position.
    let steps_to_zero = match direction {
        Direction::Right => (DIAL_SIZE - start).rem_euclid(DIAL_SIZE),
        Direction::Left => start,
    };

    // Steps remaining after the first time we land on zero.
    let remaining = steps - steps_to_zero;

    match steps_to_zero {
        0 => steps / DIAL_SIZE,
        _ if remaining >= 0 => 1 + (remaining / DIAL_SIZE),
        _ => 0,
    }
}

#[derive(Debug, Clone, Copy)]
enum ParseRotationError {
    Empty,
    InvalidDirection(u8),
    InvalidDistance,
}
impl core::fmt::Display for ParseRotationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty string"),
            Self::InvalidDirection(c) => write!(f, "Invalid direction: {}", *c as char),
            Self::InvalidDistance => write!(f, "Invalid distance"),
        }
    }
}
impl core::error::Error for ParseRotationError {}

fn main() {
    use std::time::Instant;

    let input = input();
    let start = Instant::now();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    let elapsed = start.elapsed();
    println!("Elapsed time: {:.4} seconds", elapsed.as_secs_f64());
}
