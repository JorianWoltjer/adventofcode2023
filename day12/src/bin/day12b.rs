use day12::count_damage_arrangements_5x;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = count_damage_arrangements_5x(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 22);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_b() {
        let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(count_damage_arrangements_5x(input).unwrap(), 525152)
    }
}
