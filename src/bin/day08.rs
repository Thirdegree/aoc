#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use std::collections::HashMap;

fn main() {
    let mut data = aoc_2023::include_data!(day8).lines();
    let instr = data.next().unwrap();
    data.next().unwrap();

    let mut adj = HashMap::new();
    let mut cur_nodes = vec![];

    for line in data {
        let (label, dests) = line.split_once(" = ").unwrap();
        let (left, right) = dests.split_once(", ").unwrap();
        adj.insert(
            label,
            (
                left.strip_prefix('(').unwrap(),
                right.strip_suffix(')').unwrap(),
            ),
        );
        if label.ends_with('A') {
            cur_nodes.push(label);
        }
    }
    let expected_nodepaths = cur_nodes.len();
    let mut found_nodepaths = vec![];
    for (step_count, ins) in instr.chars().cycle().enumerate() {
        let mut new_cur_nodes = vec![];
        for cur_node in cur_nodes {
            let new_node = match ins {
                'L' => adj.get(cur_node).unwrap().0,
                'R' => adj.get(cur_node).unwrap().1,
                _ => unreachable!(),
            };
            if new_node.ends_with('Z') {
                found_nodepaths.push(step_count + 1);
            } else {
                // no need to track once we've found it
                new_cur_nodes.push(new_node);
            }
        }
        if found_nodepaths.len() == expected_nodepaths {
            break;
        }
        cur_nodes = new_cur_nodes;
    }
    println!(
        "Day 8 result: {}",
        found_nodepaths
            .iter()
            .map(|&s| s as u64)
            // math is fun!
            // (also the problems must be constructed so this works,
            // it's not hard to construct a counter example)
            .reduce(aoc_2023::math::lcm)
            .unwrap()
    );
}
