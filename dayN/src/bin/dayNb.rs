use dayN::part2_function;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = part2_function(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 22);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_b() {
        let input = "";

        assert_eq!(part2_function(input).unwrap(), 2)
    }
}
