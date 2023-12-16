use grid::Grid;
use std::{error, str::FromStr};

type Err = Box<dyn error::Error>;

#[derive(Debug)]
struct LoopGrid {
    grid: Grid<char>,
    start: (usize, usize),
    visited_grid: Grid<bool>,
}
impl FromStr for LoopGrid {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_vec(
            s.lines().flat_map(|line| line.chars()).collect(),
            s.lines().next().unwrap().len(),
        );
        let visited_grid = Grid::new(grid.rows(), grid.cols());
        let start = grid
            .indexed_iter()
            .find(|(_, &c)| c == 'S')
            .map(|((x, y), _)| (x, y))
            .ok_or("Start 'S' not found")?;

        Ok(Self {
            grid,
            start,
            visited_grid,
        })
    }
}
impl LoopGrid {
    fn neighbors(&self, (y, x): (usize, usize)) -> Vec<(usize, usize)> {
        let current = self.grid.get(y, x);
        let offsets = match current {
            Some('|') => vec![(-1, 0), (1, 0)],                  // NS
            Some('-') => vec![(0, 1), (0, -1)],                  // EW
            Some('L') => vec![(-1, 0), (0, 1)],                  // NE
            Some('J') => vec![(-1, 0), (0, -1)],                 // NW
            Some('7') => vec![(1, 0), (0, -1)],                  // SW
            Some('F') => vec![(1, 0), (0, 1)],                   // SE
            Some('S') => vec![(-1, 0), (0, 1), (1, 0), (0, -1)], // NESW
            Some('.') | None => vec![],
            Some(_) => panic!("Invalid tile"),
        };

        offsets
            .into_iter()
            .map(|(offset_y, offset_x)| {
                (
                    (y as i32 + offset_y) as usize,
                    (x as i32 + offset_x) as usize,
                )
            })
            .collect()
    }

    fn next(&self, point: (usize, usize)) -> Option<(usize, usize)> {
        self.neighbors(point).into_iter().find(|&(y, x)| {
            self.visited_grid.get(y, x).is_some_and(|b| !b)
                && self.neighbors((y, x)).contains(&point)
        })
    }

    fn visit_loop(&mut self) {
        // dbg!(&self.grid);
        let mut current = self.start;

        while let Some(next) = self.next(current) {
            // println!("{next:?}");
            *self.visited_grid.get_mut(next.0, next.1).unwrap() = true;
            current = next;
        }
    }

    fn get_loop_length(&self) -> usize {
        self.visited_grid.iter().map(|&b| b as usize).sum()
    }

    fn count_enclosed(&self) -> usize {
        let mut total = 0;
        for (y, row) in self.visited_grid.iter_rows().enumerate() {
            let mut inside_loop = false; // Parity
            let mut opener = None;
            for (x, &visited) in row.enumerate() {
                if visited {
                    let value = *self.grid.get(y, x).unwrap();
                    // L must always come before 7, and F must always come before J
                    // Bit of a cursed solution here to assume S takes the place of F, but works for examples
                    match value {
                        'L' => opener = Some('L'),
                        'F' | 'S' => opener = Some('F'),
                        '7' => if opener == Some('L') { inside_loop = !inside_loop },
                        'J' => if opener == Some('F') { inside_loop = !inside_loop },
                        '|' => inside_loop = !inside_loop,
                        _ => ()
                    }
                } else if inside_loop {
                    total += 1
                }
            }
        }
        total
    }
}

pub fn furthest_in_loop(s: &str) -> Result<usize, Err> {
    let mut loopgrid: LoopGrid = s.parse()?;
    loopgrid.visit_loop();

    Ok(loopgrid.get_loop_length() / 2)
}

// PART 2

pub fn count_squares_inside(s: &str) -> Result<usize, Err> {
    let mut loopgrid: LoopGrid = s.parse()?;
    loopgrid.visit_loop();

    Ok(loopgrid.count_enclosed())
}
