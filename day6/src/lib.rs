use std::error;

type Err = Box<dyn error::Error>;

struct Race {
    time: u64,
    distance: u64,
}
impl Race {
    pub fn vec_from_str(s: &str) -> Result<Vec<Race>, Err> {
        let mut lines = s.lines();
        let times: Vec<u64> = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .ok_or("prefix 'Time:' not found")?
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;

        let distances: Vec<u64> = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .ok_or("prefix 'Distance:' not found")?
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;

        Ok(times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Self { time, distance })
            .collect())
    }

    pub fn big_from_str(s: &str) -> Result<Race, Err> {
        let mut lines = s.lines();
        let time = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .ok_or("prefix 'Time:' not found")?
            .replace(' ', "")
            .parse()?;

        let distance = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .ok_or("prefix 'Distance:' not found")?
            .replace(' ', "")
            .parse()?;

        Ok(Self { time, distance })
    }

    /// Returns the number of integer ways to win
    ///
    /// ```
    /// t = 7-n  // time left to move
    /// v = n  // velocity
    /// d = v * t  // distance traveled
    ///
    /// d = n(7-n)
    /// d = 7n-n^2
    /// d = -n^2 + 7n
    ///
    /// -n^2 + 7n > 9
    /// -n^2 + 7n + 9 > 0
    /// ```
    pub fn solve(&self) -> u64 {
        let (start, end) = quadratic_formula(-1., self.time as f64, -(self.distance as f64));

        // Whole numbers don't count, and need to count only numbers inside
        let start = (start + 0.0000001).ceil() as u64;
        let end = (end - 0.0000001).floor() as u64;

        end - start + 1
    }
}

pub fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    (
        (-b + f64::sqrt(b.powi(2) - 4. * a * c)) / (2. * a),
        (-b - f64::sqrt(b.powi(2) - 4. * a * c)) / (2. * a),
    )
}

pub fn multiply_ways_to_win(s: &str) -> Result<u64, Err> {
    let races = Race::vec_from_str(s)?;

    Ok(races.iter().map(|race| race.solve()).product())
}

// PART 2

pub fn big_ways_to_win(s: &str) -> Result<u64, Err> {
    let race = Race::big_from_str(s)?;

    Ok(race.solve())
}
