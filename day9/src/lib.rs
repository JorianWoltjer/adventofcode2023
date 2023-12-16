use std::{error, str::FromStr};

type Err = Box<dyn error::Error>;

fn differences(v: &[i32]) -> Vec<i32> {
    v.windows(2)
        .map(|ab| {
            let [a, b] = ab else { unreachable!() };
            b - a
        })
        .collect()
}

struct History {
    measurements: Vec<i32>,
}
impl History {
    fn get_differences(&self) -> Vec<Vec<i32>> {
        let mut all_differences = vec![self.measurements.clone()];
        while !all_differences.last().unwrap().iter().all(|&n| n == 0) {
            let diff = differences(all_differences.last().unwrap());
            all_differences.push(diff);
        }
        all_differences
    }

    pub fn predict_next(&self) -> i32 {
        let mut all_differences = self.get_differences();

        // Extrapolate values (last)
        for i in (1..all_differences.len()).rev() {
            let lower = *all_differences[i].last().unwrap();
            let upper = all_differences.get_mut(i - 1).unwrap();
            *upper.last_mut().unwrap() += lower;
        }

        *all_differences[0].last().unwrap()
    }

    pub fn predict_prev(&self) -> i32 {
        let mut all_differences = self.get_differences();

        // Extrapolate values (first)
        for i in (1..all_differences.len()).rev() {
            let lower = *all_differences[i].first().unwrap();
            let upper = all_differences.get_mut(i - 1).unwrap();
            *upper.first_mut().unwrap() -= lower;
        }

        *all_differences[0].first().unwrap()
    }
}
impl FromStr for History {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let measurements = s
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;
        Ok(Self { measurements })
    }
}

pub fn sum_next_predictions(s: &str) -> Result<i32, Err> {
    let histories: Vec<History> = s
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    Ok(histories
        .into_iter()
        .map(|history| history.predict_next())
        .sum())
}

pub fn sum_prev_predictions(s: &str) -> Result<i32, Err> {
    let histories: Vec<History> = s
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    Ok(histories
        .into_iter()
        .map(|history| history.predict_prev())
        .sum())
}
