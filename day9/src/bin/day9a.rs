use day9::sum_next_predictions;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = sum_next_predictions(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 1992273652);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(sum_next_predictions(input).unwrap(), 114)
    }
}
