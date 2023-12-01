use day1::spelled_calibrate;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = spelled_calibrate(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 55614);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(spelled_calibrate(input), Some(281));
    }
}
