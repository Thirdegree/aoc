use std::collections::HashMap;

fn lcm(first: u64, second: u64) -> u64 {
    (first * second) / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn main() {
    let mut data = include_str!("../day8/input.txt").lines();
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
    let mut step_count = 0;
    let mut found_nodepaths = vec![];
    for ins in instr.chars().cycle() {
        let mut new_cur_nodes = vec![];
        for (idx, cur_node) in cur_nodes.iter().enumerate() {
            match ins {
                'L' => new_cur_nodes.push(adj.get(cur_node).unwrap().0),
                'R' => new_cur_nodes.push(adj.get(cur_node).unwrap().1),
                _ => unreachable!(),
            }
            if new_cur_nodes.last().unwrap().ends_with('Z') {
                found_nodepaths.push((idx, step_count + 1));
            }
        }
        step_count += 1;
        if found_nodepaths.len() == cur_nodes.len() {
            break;
        }
        cur_nodes = new_cur_nodes;
    }
    println!(
        "Maybe it's this? {}",
        found_nodepaths.iter().map(|(_, s)| *s).reduce(lcm).unwrap()
    );
    println!("Day 8 result: {step_count}")
}
