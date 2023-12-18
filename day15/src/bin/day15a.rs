use day15::sum_hashes;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = sum_hashes(input);

    println!("{answer}");
    assert_eq!(answer, 511498);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(sum_hashes(input), 1320)
    }
}
