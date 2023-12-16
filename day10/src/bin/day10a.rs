use day10::furthest_in_loop;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = furthest_in_loop(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 6690);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        assert_eq!(furthest_in_loop(input).unwrap(), 4)
    }

    #[test]
    fn example_a2() {
        let input = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        assert_eq!(furthest_in_loop(input).unwrap(), 8)
    }
}
