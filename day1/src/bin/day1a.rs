use day1::calibrate;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = calibrate(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 55488);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(calibrate(input), Some(142));
    }
}
