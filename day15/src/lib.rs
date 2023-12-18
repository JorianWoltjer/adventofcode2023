use std::{error, str::FromStr};

use linked_hash_map::LinkedHashMap;

type Err = Box<dyn error::Error>;

fn hash(s: &str) -> usize {
    s.chars().fold(0, |mut acc, c| {
        acc += c as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}

pub fn sum_hashes(s: &str) -> usize {
    s.split(',').map(hash).sum()
}

// PART 2

#[derive(Debug)]
enum Operation {
    Insert { label: String, n: u8 },
    Remove { label: String },
}
impl FromStr for Operation {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(label) = s.strip_suffix('-') {
            Ok(Self::Remove {
                label: label.to_owned(),
            })
        } else {
            let (label, n) = s.split_once('=').ok_or("Expected '=' delimiter")?;
            Ok(Self::Insert {
                label: label.to_owned(),
                n: n.parse()?,
            })
        }
    }
}

pub fn calc_hash_boxes(s: &str) -> Result<usize, Err> {
    let operations: Vec<Operation> = s
        .split(',')
        .map(|input| input.parse())
        .collect::<Result<_, _>>()?;

    let mut boxes = vec![LinkedHashMap::new(); 256];
    for op in operations {
        match op {
            Operation::Insert { label, n } => {
                *boxes[hash(&label)].entry(label).or_insert(0) = n;
            }
            Operation::Remove { label } => {
                boxes[hash(&label)].remove(&label);
            }
        }
    }

    Ok(boxes
        .iter()
        .enumerate()
        .flat_map(|(box_i, map)| {
            map.values()
                .enumerate()
                .map(move |(i, &n)| (box_i + 1) * (i + 1) * n as usize)
        })
        .sum())
}
