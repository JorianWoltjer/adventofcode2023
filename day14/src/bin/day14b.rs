use day14::count_roll_cycle;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = count_roll_cycle(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 105606);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_b() {
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

        assert_eq!(count_roll_cycle(input).unwrap(), 64)
    }
}
