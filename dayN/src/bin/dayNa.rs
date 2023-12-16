use dayN::part1_function;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = part1_function(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 11);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = "";

        assert_eq!(part1_function(input).unwrap(), 1)
    }
}
