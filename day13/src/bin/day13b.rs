use day13::find_imperfect_symmetries;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = find_imperfect_symmetries(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 33106);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_b() {
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

        assert_eq!(find_imperfect_symmetries(input).unwrap(), 400)
    }
}
