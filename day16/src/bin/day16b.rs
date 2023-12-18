use day16::best_beam_energy;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = best_beam_energy(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 7438);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_b() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        assert_eq!(best_beam_energy(input).unwrap(), 51)
    }
}
