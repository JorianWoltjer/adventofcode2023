use day7::order_hands_joker;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = order_hands_joker(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 251003917);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2() {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(order_hands_joker(input).unwrap(), 5905)
    }
}
