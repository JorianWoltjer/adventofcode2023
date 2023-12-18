use day15::calc_hash_boxes;

fn main() {
    let input = include_str!("../../input.txt");

    let answer = calc_hash_boxes(input).unwrap();

    println!("{answer}");
    assert_eq!(answer, 284674);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_b() {
        let input = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(calc_hash_boxes(input).unwrap(), 145)
    }
}
