use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_DIGITS: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug)]
struct Number {
    row: usize,
    col_start: usize,
    col_end: usize,
    n: u32,
}
impl Number {
    pub fn from_str(s: &str) -> Vec<Number> {
        let mut numbers = vec![];

        for (i, line) in s.lines().enumerate() {
            numbers.extend(RE_DIGITS.find_iter(line).map(|m| Number {
                row: i,
                col_start: m.start(),
                col_end: m.end(),
                n: m.as_str().parse().unwrap(),
            }));
        }

        numbers
    }
}

/// Find position of symbol around a Number in the grid based on f()
fn find_around(n: &Number, grid: &Vec<Vec<char>>, f: fn(char) -> bool) -> Option<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();

    if n.col_start > 0 && f(grid[n.row][n.col_start - 1]) {
        return Some((n.row, n.col_start - 1)); // Left
    }
    if n.col_end < grid[n.row].len() && f(grid[n.row][n.col_end]) {
        return Some((n.row, n.col_end)); // Right
    }
    for x in n.col_start as isize - 1..=n.col_end as isize {
        if x < 0 || x >= width as isize {
            continue;
        }
        let x = x as usize;

        if n.row > 0 && f(grid[n.row - 1][x]) {
            return Some((n.row - 1, x)); // Top
        }
        if n.row + 1 < height && f(grid[n.row + 1][x]) {
            return Some((n.row + 1, x)); // Bottom
        }
    }

    None
}

/// c is not a digit or '.'
fn is_symbol(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

pub fn sum_engine_parts(s: &str) -> u32 {
    let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

    Number::from_str(s)
        .iter()
        .filter_map(|n| find_around(n, &grid, is_symbol).is_some().then_some(n.n))
        .sum()
}

// PART 2

pub fn sum_gears(s: &str) -> u32 {
    let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let mut gears = HashMap::new();

    for n in Number::from_str(s) {
        if let Some(gear) = find_around(&n, &grid, |c| c == '*') {
            gears.entry(gear).or_insert(vec![]).push(n.n);
        }
    }

    gears
        .values()
        .filter_map(|numbers| (numbers.len() == 2).then(|| numbers[0] * numbers[1]))
        .sum()
}
