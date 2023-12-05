#![feature(iter_advance_by)]

use std::{collections::VecDeque, error, ops::Range, str::FromStr};

use itertools::Itertools;

type Err = Box<dyn error::Error>;

#[derive(Debug)]
struct Offsetter {
    src: Range<u64>,
    offset: i64,
}
impl Offsetter {
    pub fn convert(&self, n: u64) -> u64 {
        (self.offset + n as i64) as u64
    }

    pub fn convert_range(&self, range: Range<u64>) -> Range<u64> {
        self.convert(range.start)..self.convert(range.end)
    }
}

#[derive(Debug)]
struct Map {
    offsetters: Vec<Offsetter>,
}
impl FromStr for Map {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut offsetters = s.lines();
        offsetters.next(); // Skip '... map:' line
        let ranges = offsetters
            .map(|line| {
                let mut split = line.split_whitespace();
                let dst_start: u64 = split.next().ok_or("dst_start not found")?.parse()?;
                let src_start: u64 = split.next().ok_or("src_start not found")?.parse()?;
                let length: u64 = split.next().ok_or("length not found")?.parse()?;
                Ok::<_, Err>(Offsetter {
                    src: src_start..src_start + length,
                    offset: dst_start as i64 - src_start as i64,
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { offsetters: ranges })
    }
}
impl Map {
    pub fn convert(&self, n: u64) -> u64 {
        for offset in &self.offsetters {
            if offset.src.contains(&n) {
                return offset.convert(n);
            }
        }

        n
    }

    // PART 2

    pub fn convert_ranges(&self, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut result = vec![];

        // Queue is needed because one range can have multiple offsetters inside
        // The idea is to split a range when an offsetter overlaps
        let mut queue = VecDeque::from(ranges);
        while let Some(range) = queue.pop_front() {
            let mut done = false;

            for offset in &self.offsetters {
                if offset.src.start <= range.start {
                    if offset.src.end <= range.start {
                        continue;
                    } else if offset.src.end < range.end {
                        // End is inside range, start is converted
                        result.push(offset.convert_range(range.start..offset.src.end));
                        queue.push_back(offset.src.end..range.end);
                        done = true;
                        break;
                    } else {
                        // End is after range, everything is converted
                        result.push(offset.convert_range(range.start..range.end));
                        done = true;
                        break;
                    }
                } else if offset.src.start < range.end {
                    if offset.src.end < range.end {
                        // Start and end are inside, so split twice
                        queue.push_back(range.start..offset.src.start);
                        result.push(offset.convert_range(offset.src.start..offset.src.end));
                        queue.push_back(offset.src.end..range.end);
                        done = true;
                        break;
                    } else {
                        // Start is inside, and end is outside, so end is converted
                        queue.push_back(range.start..offset.src.start);
                        result.push(offset.convert_range(offset.src.start..range.end));
                        done = true;
                        break;
                    }
                } else {
                    continue;
                }
            }
            if !done {
                result.push(range);
            }
        }

        result
    }
}

#[derive(Debug)]
struct Converter {
    maps: Vec<Map>,
}
impl FromStr for Converter {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maps = s.split("\n\n");
        maps.next(); // Skip 'seeds: ' line
        let maps = maps.map(|map| map.parse()).collect::<Result<_, _>>()?;

        Ok(Self { maps })
    }
}
impl Converter {
    pub fn convert(&self, mut n: u64) -> u64 {
        for map in &self.maps {
            n = map.convert(n);
        }
        n
    }

    // PART 2

    pub fn convert_ranges(&self, mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        for map in &self.maps {
            ranges = map.convert_ranges(ranges);
        }

        ranges
    }
}

pub fn minimum_converted_location(s: &str) -> Result<u64, Err> {
    let seeds: Vec<u64> = s
        .lines()
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .ok_or("First line does not start with 'seeds: '")?
        .split_whitespace()
        .map(|n| n.parse())
        .collect::<Result<_, _>>()?;

    let converter: Converter = s.parse()?;

    Ok(seeds
        .into_iter()
        .map(|n| converter.convert(n))
        .min()
        .unwrap())
}

// PART 2

pub fn minimum_location_range(s: &str) -> Result<u64, Err> {
    let chunks = s
        .lines()
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .ok_or("First line does not start with 'seeds: '")?
        .split_whitespace()
        .map(|n| n.parse::<u64>())
        .chunks(2);

    let mut ranges = vec![];

    for mut chunk in &chunks {
        let start = chunk.next().unwrap()?;
        let length = chunk.next().unwrap()?;
        ranges.push(start..start + length);
    }

    let converter: Converter = s.parse()?;

    Ok(converter
        .convert_ranges(ranges)
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap())
}
