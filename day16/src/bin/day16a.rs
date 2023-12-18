use day16::count_beam_energy;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = count_beam_energy(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 7199);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
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

        assert_eq!(count_beam_energy(input).unwrap(), 46)
    }
}

// 216 too low
