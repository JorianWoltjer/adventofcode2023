use std::{error, str::FromStr};

type Err = Box<dyn error::Error>;

// TODO: variable = padding between damanges, recursive algorithm?

#[derive(PartialEq, Clone, Copy)]
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
    fn is_valid(&self, springs: &[Spring]) -> bool {
        let mut i = 0;
        let mut count = 0;
        for spring in springs.iter().chain([Spring::Operational].iter()) {
            match spring {
                Spring::Damanged => count += 1,
                Spring::Operational => {
                    if count > 0 {
                        if self.counts.get(i).is_some_and(|&n| n == count) {
                            i += 1;
                            count = 0;
                        } else {
                            return false;
                        }
                    }
                }
                Spring::Unknown => panic!("Unexpected '?' found is is_valid() check"),
            }
        }
        i == self.counts.len()
    }

    fn get_arrangements(&self) -> usize {
        let unknowns = self
            .springs
            .iter()
            .enumerate()
            .filter_map(|(i, &spring)| (spring == Spring::Unknown).then_some(i))
            .collect::<Vec<_>>();

        let mut total = 0;
        for i in 0..2usize.pow(unknowns.len() as u32) {
            let mut springs = self.springs.clone();
            for (j, &unknown) in unknowns.iter().enumerate() {
                springs[unknown] = if i & (1 << j) == 0 {
                    Spring::Operational
                } else {
                    Spring::Damanged
                };
            }
            if self.is_valid(&springs) {
                total += 1;
            }
        }

        total
    }
}

pub fn count_damage_arrangements(s: &str) -> Result<usize, Err> {
    let records: Vec<Record> = s
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    Ok(records.iter().map(|record| record.get_arrangements()).sum())
}
