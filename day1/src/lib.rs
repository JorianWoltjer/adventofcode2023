use std::ops::Range;

pub fn first_last_sum(line: &str) -> Option<u32> {
    let mut total = 0;
    total += line
        .chars()
        .map(|c| c.to_digit(10))
        .find(|r| r.is_some())??
        * 10;
    total += line
        .chars()
        .rev()
        .map(|c| c.to_digit(10))
        .find(|r| r.is_some())??;
    Some(total)
}

pub fn calibrate(input: &str) -> Option<u32> {
    input.lines().map(first_last_sum).sum()
}

const SPELLED_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn reversed_range(
    range: Range<usize>,
    reverse: bool,
) -> itertools::Either<impl Iterator<Item = usize>, impl Iterator<Item = usize>> {
    if !reverse {
        itertools::Either::Left(range)
    } else {
        itertools::Either::Right(range.rev())
    }
}

pub fn spelled_digit(line: &str, reverse: bool) -> Option<u32> {
    for i in reversed_range(0..line.len(), reverse) {
        let slice = &line[i..];
        match slice.chars().next().unwrap().to_digit(10) {
            Some(digit) => return Some(digit),
            None => {
                for (n, digit) in SPELLED_DIGITS.iter().enumerate() {
                    if slice.starts_with(digit) {
                        return Some(n.try_into().unwrap());
                    }
                }
            }
        }
    }

    None
}

pub fn spelled_calibrate(input: &str) -> Option<u32> {
    input
        .lines()
        .map::<Option<u32>, _>(|line| {
            Some(spelled_digit(line, false)? * 10 + spelled_digit(line, true)?)
        })
        .sum()
}
