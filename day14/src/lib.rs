use std::{
    collections::{hash_map::Entry, HashMap},
    error,
};

use grid::Grid;

type Err = Box<dyn error::Error>;

fn roll_north(grid: &mut Grid<char>) {
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            if grid[(y, x)] == 'O' {
                let target = (0..y)
                    .rev()
                    .find(|&y| grid[(y, x)] != '.')
                    .map(|y| y + 1)
                    .unwrap_or(0);
                grid[(y, x)] = '.';
                grid[(target, x)] = 'O';
            }
        }
    }
}

fn roll_west(grid: &mut Grid<char>) {
    for x in 0..grid.cols() {
        for y in 0..grid.rows() {
            if grid[(y, x)] == 'O' {
                let target = (0..x)
                    .rev()
                    .find(|&x| grid[(y, x)] != '.')
                    .map(|x| x + 1)
                    .unwrap_or(0);
                grid[(y, x)] = '.';
                grid[(y, target)] = 'O';
            }
        }
    }
}

fn roll_south(grid: &mut Grid<char>) {
    grid.flip_rows();
    roll_north(grid);
    grid.flip_rows();
}

fn roll_east(grid: &mut Grid<char>) {
    grid.flip_cols();
    roll_west(grid);
    grid.flip_cols();
}

fn roll_cycle(grid: &mut Grid<char>) {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid);
}

fn calculate_load(grid: &Grid<char>) -> usize {
    grid.iter_rows()
        .enumerate()
        .map(|(i, row)| row.filter(|&&c| c == 'O').count() * (grid.rows() - i))
        .sum()
}

pub fn count_roll_north(s: &str) -> Result<usize, Err> {
    let mut grid = Grid::from_vec(
        s.lines().flat_map(|line| line.chars()).collect(),
        s.lines().next().unwrap().len(),
    );

    roll_north(&mut grid);

    Ok(calculate_load(&grid))
}

// PART 2

pub fn count_roll_cycle(s: &str) -> Result<usize, Err> {
    let mut grid = Grid::from_vec(
        s.lines().flat_map(|line| line.chars()).collect(),
        s.lines().next().unwrap().len(),
    );
    let mut grid_clone = grid.clone();

    let mut states = HashMap::new();
    let mut i = 0;
    let (cycle_start, cycle_length) = loop {
        roll_cycle(&mut grid);
        let hash: String = grid.iter().collect();
        match states.entry(hash) {
            Entry::Occupied(o) => break (*o.get(), i - *o.get()),
            Entry::Vacant(v) => v.insert(i),
        };
        i += 1;
    };

    let required = cycle_start + ((1000000000 - cycle_start) % cycle_length);
    for _ in 0..required {
        roll_cycle(&mut grid_clone);
    }

    Ok(calculate_load(&grid_clone))
}
