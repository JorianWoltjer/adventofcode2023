use std::{collections::HashMap, error};

type Err = Box<dyn error::Error>;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn parse_nodes(s: &str) -> Result<HashMap<String, Node>, Err> {
    s.lines()
        .map(|line| -> Result<_, _> {
            let (name, children) = line.split_once(" = ").ok_or("Expected ' = ' delimiter")?;
            let (left, right) = children
                .strip_prefix('(')
                .ok_or("Expected '(' prefix")?
                .strip_suffix(')')
                .ok_or("Expected ')' suffix")?
                .split_once(", ")
                .ok_or("Expected ', ' delimiter")?;

            Ok((
                name.to_owned(),
                Node {
                    left: left.to_owned(),
                    right: right.to_owned(),
                },
            ))
        })
        .collect()
}

fn follow_path(
    nodes: &HashMap<String, Node>,
    steps: &str,
    start: &str,
    done: fn(&str) -> bool,
) -> Result<usize, Err> {
    let mut current_name = start;
    let mut total_steps = 0;
    'outer: loop {
        for step in steps.chars() {
            total_steps += 1;
            let current_node = nodes
                .get(current_name)
                .ok_or(format!("Node {current_name:?} not found"))?;
            match step {
                'L' => current_name = &current_node.left,
                'R' => current_name = &current_node.right,
                _ => Err(format!("Direction {step:?} not recognized"))?,
            }
            if done(current_name) {
                break 'outer;
            }
        }
    }
    Ok(total_steps)
}

pub fn count_steps(s: &str) -> Result<usize, Err> {
    let (steps, nodes) = s.split_once("\n\n").ok_or("Expected '\\n\\n' delimiter")?;
    let nodes = parse_nodes(nodes)?;

    follow_path(&nodes, steps, "AAA", |name| name == "ZZZ")
}

// Source: https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

// PART 2

pub fn count_steps_all_paths(s: &str) -> Result<usize, Err> {
    let (steps, nodes) = s.split_once("\n\n").ok_or("Expected '\\n\\n' delimiter")?;
    let nodes = parse_nodes(nodes)?;

    let start_names = nodes
        .keys()
        .filter(|name| name.ends_with('A'))
        .collect::<Vec<_>>();

    let all_totals = start_names
        .into_iter()
        .map(|start| follow_path(&nodes, steps, start, |name| name.ends_with('Z')))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(lcm(&all_totals))
}
