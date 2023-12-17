#![feature(iter_intersperse)]
use std::{collections::VecDeque, error, str::FromStr};

use rayon::prelude::*;

type Err = Box<dyn error::Error>;

// TODO: variable = padding between damanges, recursive algorithm?

#[derive(Debug, PartialEq, Clone, Copy)]
enum Spring {
    Operational,
    Damanged,
    Unknown,
}
impl TryFrom<char> for Spring {
    type Error = Err;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damanged),
            '?' => Ok(Self::Unknown),
            _ => Err(format!("Unrecognized value {value:?}").into()),
        }
    }
}

#[derive(Debug)]
struct Record {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}
impl FromStr for Record {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, counts) = s.split_once(' ').ok_or("Expected ' ' delimiter")?;
        let springs = springs
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<_, _>>()?;
        let counts = counts
            .split(',')
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;
        Ok(Self { springs, counts })
    }
}
impl Record {
    fn unfold(&mut self, n: usize) {
        self.springs = (0..n)
            .map(|_| self.springs.clone())
            .intersperse(vec![Spring::Unknown])
            .flatten()
            .collect();
        self.counts = self.counts.repeat(5);
    }

    fn maybe_valid(&self, springs: &[Spring]) -> bool {
        let mut counts = 0;
        let mut length = 0;
        for spring in springs.iter().chain([Spring::Operational].iter()) {
            match spring {
                Spring::Damanged => length += 1,
                Spring::Operational => {
                    if length > 0 {
                        if self.counts.get(counts).is_some_and(|&n| n == length) {
                            counts += 1;
                            length = 0;
                        } else {
                            return false;
                        }
                    }
                }
                Spring::Unknown => return true,
            }
        }
        counts == self.counts.len()
    }

    fn recursive_get_arrangements(
        &self,
        mut springs: Vec<Spring>,
        mut unknowns: VecDeque<usize>,
    ) -> usize {
        if !self.maybe_valid(&springs) {
            return 0; // If not maybe valid, never will be
        } else if unknowns.is_empty() {
            return 1; // If valid and no unknowns, counts
        }

        let mut total = 0;
        let index = unknowns.pop_front().unwrap();
        let mut springs_clone = springs.clone();
        springs[index] = Spring::Operational;
        total += self.recursive_get_arrangements(springs, unknowns.clone());
        springs_clone[index] = Spring::Damanged;
        total += self.recursive_get_arrangements(springs_clone, unknowns);

        total
    }

    fn get_arrangements(&self) -> usize {
        let unknowns = self
            .springs
            .iter()
            .enumerate()
            .filter_map(|(i, &spring)| (spring == Spring::Unknown).then_some(i))
            .collect::<VecDeque<_>>();

        self.recursive_get_arrangements(self.springs.clone(), unknowns)
    }
}

pub fn count_damage_arrangements(s: &str) -> Result<usize, Err> {
    let records: Vec<Record> = s
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    Ok(records.iter().map(|record| record.get_arrangements()).sum())
}

pub fn count_damage_arrangements_5x(s: &str) -> Result<usize, Err> {
    let mut records: Vec<Record> = s
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;
    // let mut records: Vec<Record> = vec!["???.### 1,1,3".parse()?];

    records.iter_mut().for_each(|record| record.unfold(5));
    // dbg!(&records);

    Ok(records
        .par_iter()
        .map(|record| record.get_arrangements())
        .sum())
}
