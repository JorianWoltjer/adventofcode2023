use day11::sum_expanded_lengths;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = sum_expanded_lengths(input, 1000000);

    println!("{answer}");
    assert_eq!(answer, 569052586852);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_b() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(sum_expanded_lengths(input, 10), 1030)
    }

    #[test]
    fn example_b2() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(sum_expanded_lengths(input, 100), 8410)
    }
}
