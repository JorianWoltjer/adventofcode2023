use day6::big_ways_to_win;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = big_ways_to_win(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 26187338);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2() {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(big_ways_to_win(input).unwrap(), 71503)
    }
}
