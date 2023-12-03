use day3::sum_engine_parts;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = sum_engine_parts(input);

    println!("{answer}");
    assert_eq!(answer, 514969);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(sum_engine_parts(input), 4361);
    }
}
