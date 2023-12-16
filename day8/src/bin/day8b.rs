use day8::count_steps_all_paths;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = count_steps_all_paths(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 16563603485021);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2() {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(count_steps_all_paths(input).unwrap(), 6)
    }
}
