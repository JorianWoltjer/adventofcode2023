use day7::order_hands;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = order_hands(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 251029473);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(order_hands(input).unwrap(), 6440)
    }

    #[test]
    fn extra_example() {
        let input = "\
82222 1
28828 2
AKAKA 3
";
        assert_eq!(order_hands(input).unwrap(), 11)
    }
}
