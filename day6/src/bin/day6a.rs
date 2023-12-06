use day6::multiply_ways_to_win;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = multiply_ways_to_win(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 440000);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(multiply_ways_to_win(input).unwrap(), 288)
    }
}
