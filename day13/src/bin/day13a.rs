use day13::find_symmetries;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = find_symmetries(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 34100);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(find_symmetries(input).unwrap(), 405)
    }
}
