use std::{error, str::FromStr};

use grid::Grid;
use itertools::Itertools;

type Err = Box<dyn error::Error>;

struct Pattern {
    grid: Grid<bool>,
}
impl FromStr for Pattern {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_vec(
            s.lines()
                .flat_map(|line| line.chars().map(|c| c == '#'))
                .collect(),
            s.lines().next().unwrap().len(),
        );
        Ok(Self { grid })
    }
}
impl Pattern {
    fn is_perfect_horizontal_symmetry(&self, center: usize) -> bool {
        let bound = usize::min(center + 1, self.grid.rows() - center - 1);
        (0..bound).all(|i| {
            let up = center - i;
            let down = center + i + 1;
            self.grid.iter_row(up).collect_vec() == self.grid.iter_row(down).collect_vec()
        })
    }
    fn is_perfect_vertical_symmetry(&self, center: usize) -> bool {
        let bound = usize::min(center + 1, self.grid.cols() - center - 1);
        (0..bound).all(|i| {
            let left = center - i;
            let right = center + i + 1;
            self.grid.iter_col(left).collect_vec() == self.grid.iter_col(right).collect_vec()
        })
    }

    fn horizontal_symmetry(&self) -> Option<usize> {
        self.grid
            .iter_rows()
            .tuple_windows()
            .positions(|(row1, row2)| row1.collect_vec() == row2.collect_vec())
            .find(|&i| self.is_perfect_horizontal_symmetry(i))
    }
    fn vertical_symmetry(&self) -> Option<usize> {
        self.grid
            .iter_cols()
            .tuple_windows()
            .positions(|(col1, col2)| col1.collect_vec() == col2.collect_vec())
            .find(|&i| self.is_perfect_vertical_symmetry(i))
    }

    fn is_imperfect_horizontal_symmetry(&self, center: usize) -> bool {
        let bound = usize::min(center + 1, self.grid.rows() - center - 1);
        let mut mistakes = 0;
        for i in 0..bound {
            let up = center - i;
            let down = center + i + 1;
            mistakes += self
                .grid
                .iter_row(up)
                .zip(self.grid.iter_row(down))
                .filter(|(a, b)| a != b)
                .count();

            if mistakes > 1 {
                return false;
            }
        }
        mistakes == 1
    }

    fn is_imperfect_vertical_symmetry(&self, center: usize) -> bool {
        let bound = usize::min(center + 1, self.grid.cols() - center - 1);
        let mut mistakes = 0;
        for i in 0..bound {
            let left = center - i;
            let right = center + i + 1;
            mistakes += self
                .grid
                .iter_col(left)
                .zip(self.grid.iter_col(right))
                .filter(|(a, b)| a != b)
                .count();

            if mistakes > 1 {
                return false;
            }
        }
        mistakes == 1
    }

    fn imperfect_horizontal_symmetry(&self) -> Option<usize> {
        (0..self.grid.rows()).find(|&i| self.is_imperfect_horizontal_symmetry(i))
    }
    fn imperfect_vertical_symmetry(&self) -> Option<usize> {
        (0..self.grid.cols()).find(|&i| self.is_imperfect_vertical_symmetry(i))
    }
}

pub fn find_symmetries(s: &str) -> Result<usize, Err> {
    let patterns: Vec<Pattern> = s
        .split("\n\n")
        .map(|pattern| pattern.parse())
        .collect::<Result<_, _>>()?;

    let row_sum = patterns
        .iter()
        .filter_map(|pattern| pattern.horizontal_symmetry().map(|i| i + 1))
        .sum::<usize>();
    let col_sum = patterns
        .iter()
        .filter_map(|pattern| pattern.vertical_symmetry().map(|i| i + 1))
        .sum::<usize>();

    Ok(col_sum + row_sum * 100)
}

pub fn find_imperfect_symmetries(s: &str) -> Result<usize, Err> {
    let patterns: Vec<Pattern> = s
        .split("\n\n")
        .map(|pattern| pattern.parse())
        .collect::<Result<_, _>>()?;

    let row_sum = patterns
        .iter()
        .filter_map(|pattern| pattern.imperfect_horizontal_symmetry().map(|i| i + 1))
        .sum::<usize>();
    let col_sum = patterns
        .iter()
        .filter_map(|pattern| pattern.imperfect_vertical_symmetry().map(|i| i + 1))
        .sum::<usize>();

    Ok(col_sum + row_sum * 100)
}
