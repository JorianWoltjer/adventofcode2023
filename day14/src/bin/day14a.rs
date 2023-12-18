use day14::count_roll_north;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = count_roll_north(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 112048);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(count_roll_north(input).unwrap(), 136)
    }
}
