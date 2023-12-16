use day10::count_squares_inside;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = count_squares_inside(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 525);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_b() {
        let input = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(count_squares_inside(input).unwrap(), 4)
    }

    #[test]
    fn example_b2() {
        let input = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        assert_eq!(count_squares_inside(input).unwrap(), 4)
    }

    #[test]
    fn example_b_larger() {
        let input = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(count_squares_inside(input).unwrap(), 8)
    }

    #[test]
    fn example_b_junk() {
        let input = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(count_squares_inside(input).unwrap(), 10)
    }
}
