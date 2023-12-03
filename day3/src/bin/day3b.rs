use day3::sum_gears;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = sum_gears(input);

    println!("{answer}");
    assert_eq!(answer, 78915902);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2() {
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

        assert_eq!(sum_gears(input), 467835);
    }
}
