use aoc2025::prelude::*;

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

/// Solve part 1.
fn part_one(input: impl AsRef<str>) -> usize {
    let grid: Vec<Vec<char>> = input
        .as_ref()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let accessible = find_accessible_rolls(&grid, rows, cols);
    accessible.len()
}

/// Solve part 2.
fn part_two(input: impl AsRef<str>) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .as_ref()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let mut total_removed = 0;

    // Keep removing until no more rolls are accessible
    loop {
        let accessible = find_accessible_rolls(&grid, rows, cols);

        if accessible.is_empty() {
            break; // No more accessible rolls, we're done
        }

        // Remove all accessible rolls from this iteration
        for (r, c) in &accessible {
            grid[*r][*c] = '.';
        }

        total_removed += accessible.len();
    }

    total_removed
}

fn find_accessible_rolls(grid: &[Vec<char>], rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();

    // Check each cell in the grid
    for row in 0..rows {
        for col in 0..cols {
            // Only check paper rolls
            if grid[row][col] == '@' {
                let neighbor_count = count_adjacent_rolls(grid, row, col, rows, cols);
                if neighbor_count < 4 {
                    accessible.push((row, col));
                }
            }
        }
    }

    accessible
}

fn count_adjacent_rolls(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> usize {
    // All 8 directions: NW, N, NE, W, E, SW, S, SE
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;

    for (dr, dc) in directions {
        let row = row as i32 + dr;
        let col = col as i32 + dc;

        // Check bounds
        if row >= 0 && row < rows as i32 && col >= 0 && col < cols as i32 {
            let nr = row as usize;
            let nc = col as usize;

            if grid[nr][nc] == '@' {
                count += 1;
            }
        }
    }

    count
}
