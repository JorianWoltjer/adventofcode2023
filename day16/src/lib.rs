use std::{
    collections::{HashSet, VecDeque},
    error,
    str::FromStr,
};

use grid::Grid;

type Err = Box<dyn error::Error>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn next_direction(self, tile: char) -> Vec<Direction> {
        match tile {
            // Continue as normal
            '.' => vec![self],
            // Mirror direction forward
            '/' => vec![match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
            }],
            // Mirror direction backward
            '\\' => vec![match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Down,
            }],
            '-' => match self {
                // Pass through normally
                Direction::Right | Direction::Left => vec![self],
                // Split into both sides
                Direction::Up | Direction::Down => vec![Direction::Right, Direction::Left],
            },
            '|' => match self {
                // Pass through normally
                Direction::Up | Direction::Down => vec![self],
                // Split into both sides
                Direction::Right | Direction::Left => vec![Direction::Up, Direction::Down],
            },
            _ => panic!("Unrecognized tile"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Beam {
    y: usize,
    x: usize,
    direction: Direction,
}
impl Beam {
    // Returns next position moving in direction as `Some((y, x))`, and `None` if out-of-bounds by rows and cols
    fn next_pos(&self, rows: usize, cols: usize) -> Option<(usize, usize)> {
        match self.direction {
            Direction::Up => (self.y > 0).then(|| (self.y - 1, self.x)),
            Direction::Right => (self.x + 1 < cols).then_some((self.y, self.x + 1)),
            Direction::Down => (self.y + 1 < rows).then_some((self.y + 1, self.x)),
            Direction::Left => (self.x > 0).then(|| (self.y, self.x - 1)),
        }
    }
}

struct Square {
    grid: Grid<char>,
}
impl FromStr for Square {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_vec(
            s.lines().flat_map(|line| line.chars()).collect(),
            s.lines().next().unwrap().len(),
        );

        Ok(Self { grid })
    }
}
impl Square {
    fn count_follow_beams(&self, initial_beams: Vec<Beam>) -> usize {
        let mut queue = VecDeque::from(initial_beams);
        let mut explored = HashSet::new();
        let rows = self.grid.rows();
        let cols = self.grid.cols();

        let mut energized_grid: Grid<bool> = Grid::new(self.grid.rows(), self.grid.cols());

        while let Some(beam) = queue.pop_front() {
            // If already explored, skip
            if !explored.insert(beam.clone()) {
                continue;
            }
            energized_grid[(beam.y, beam.x)] = true;

            if let Some((y, x)) = beam.next_pos(rows, cols) {
                for direction in beam.direction.next_direction(self.grid[(y, x)]) {
                    queue.push_back(Beam { y, x, direction })
                }
            }
        }

        energized_grid.iter().filter(|b| **b).count()
    }
}

pub fn count_beam_energy(s: &str) -> Result<usize, Err> {
    let square: Square = s.parse()?;

    Ok(square.count_follow_beams(
        Direction::Right
            .next_direction(square.grid[(0, 0)])
            .iter()
            .map(|&direction| Beam {
                y: 0,
                x: 0,
                direction,
            })
            .collect(),
    ))
}

// PART 2

pub fn best_beam_energy(s: &str) -> Result<usize, Err> {
    let square: Square = s.parse()?;

    let mut beams = vec![];
    beams.extend((0..square.grid.cols()).map(|i| Beam {
        y: 0,
        x: i,
        direction: Direction::Down,
    }));
    beams.extend((0..square.grid.cols()).map(|i| Beam {
        y: square.grid.rows() - 1,
        x: i,
        direction: Direction::Up,
    }));
    beams.extend((0..square.grid.rows()).map(|i| Beam {
        y: i,
        x: 0,
        direction: Direction::Right,
    }));
    beams.extend((0..square.grid.rows()).map(|i| Beam {
        y: i,
        x: square.grid.cols() - 1,
        direction: Direction::Left,
    }));

    Ok(beams
        .into_iter()
        .map(|Beam { y, x, direction }| {
            square.count_follow_beams(
                direction
                    .next_direction(square.grid[(y, x)])
                    .iter()
                    .map(|&direction| Beam { y, x, direction })
                    .collect(),
            )
        })
        .max()
        .unwrap())
}
