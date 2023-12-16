use day8::count_steps;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = count_steps(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 16897);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(count_steps(input).unwrap(), 2)
    }

    #[test]
    fn example2() {
        let input = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(count_steps(input).unwrap(), 6)
    }
}
