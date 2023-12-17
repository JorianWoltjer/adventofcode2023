use day12::count_damage_arrangements;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = count_damage_arrangements(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 7506);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(count_damage_arrangements(input).unwrap(), 21)
    }
}
