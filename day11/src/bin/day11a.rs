use day11::sum_expanded_lengths;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = sum_expanded_lengths(input, 2);

    println!("{answer}");
    assert_eq!(answer, 9724940);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
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

        assert_eq!(sum_expanded_lengths(input, 2), 374)
    }
}
