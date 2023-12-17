#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::{HashMap, HashSet};

use priority_queue::PriorityQueue;

#[derive(PartialEq, Hash, Eq, Debug, Copy, Clone)]
struct Node {
    cost: u32,
    pos: (usize, usize),
}
fn dijkstra<'a, T, G, W2>(
    adj_list: &'a HashMap<T, Vec<T>>,
    start: &'a T,
    is_goal: G,
    weight: W2,
) -> Vec<&'a T>
where
    T: Eq + std::hash::Hash + Clone + std::fmt::Debug,
    W2: Fn(&T) -> u32,
    G: Fn(&T) -> bool,
{
    let mut to_visit = priority_queue::PriorityQueue::new();
    to_visit.push(start, 0);

    let mut distances = HashMap::new();
    let mut prev: HashMap<&T, Option<&T>> = HashMap::new();
    for k in adj_list.keys() {
        distances.insert(k, u32::MAX);
    }
    for k in adj_list.keys() {
        prev.insert(k, None);
    }
    distances.insert(start, 0);
    while !to_visit.is_empty() {
        let (mut current, _) = to_visit.pop().unwrap();
        if is_goal(current) {
            let mut path = vec![current];
            while let Some(Some(next)) = prev.get(current) {
                path.push(next);
                current = next;
            }
            return path;
        }
        for neighbor in &adj_list[current] {
            let tentative_dist = distances[current] + weight(neighbor);
            if tentative_dist < distances[neighbor] {
                to_visit.push(neighbor, -(tentative_dist as i32));
                distances.insert(neighbor, tentative_dist);
                prev.insert(neighbor, Some(current));
            }
        }
    }
    unreachable!()
}

fn astar<'a, T, G, W, W2>(
    adj_list: &'a HashMap<T, Vec<T>>,
    start: &'a T,
    is_goal: G,
    heuristic: W,
    weight: W2,
) -> Vec<&'a T>
where
    T: Eq + std::hash::Hash + Clone + std::fmt::Debug,
    W: Fn(&T) -> u32,
    W2: Fn(&T) -> u32,
    G: Fn(&T) -> bool,
{
    let mut to_visit = PriorityQueue::new();
    to_visit.push(start, 0);
    let mut camefrom: HashMap<&T, &T> = HashMap::new();
    let mut g_score: HashMap<&T, u32> = HashMap::new();
    g_score.insert(start, 0);

    while !to_visit.is_empty() {
        let (current, _) = to_visit.pop().unwrap();
        if is_goal(current) {
            let mut current = current;
            let mut path = vec![current];
            while let Some(came) = camefrom.get(&current) {
                path.push(came);
                current = came;
            }
            return path.into_iter().rev().collect();
        }
        to_visit.remove(&current);
        for neighbor in &adj_list[current] {
            let tentative_gscore = g_score[&current] + weight(neighbor);
            if let Some(&old_score) = g_score.get(neighbor) {
                if tentative_gscore < old_score {
                    camefrom.insert(neighbor, current);
                    let h = heuristic(neighbor);
                    to_visit.push(neighbor, -(tentative_gscore as i32 + h as i32));
                    g_score.insert(neighbor, tentative_gscore);
                }
            } else {
                let h = heuristic(neighbor);
                g_score.insert(neighbor, tentative_gscore);
                camefrom.insert(neighbor, current);
                to_visit.push(neighbor, -(tentative_gscore as i32 + h as i32));
            }
        }
    }
    vec![]
}

fn neighbors(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if pos.0 > 0 {
        neighbors.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        neighbors.push((pos.0, pos.1 - 1));
    }
    neighbors.push((pos.0 + 1, pos.1));
    neighbors.push((pos.0, pos.1 + 1));
    neighbors
}

#[allow(clippy::too_many_lines)]
fn main() {
    let map: Vec<Vec<_>> = aoc_2023::include_data!(day17)
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut adj_list1: HashMap<_, Vec<_>> = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let pos = (x, y);
            for neigh in neighbors(pos) {
                if neigh.0 >= map[0].len() || neigh.1 >= map.len() {
                    continue;
                }
                adj_list1
                    .entry(Node {
                        pos,
                        cost: map[pos.1][pos.0],
                    })
                    .or_default()
                    .push(Node {
                        pos: neigh,
                        cost: map[neigh.1][neigh.0],
                    });
            }
        }
    }
    let mut adj_list2: HashMap<_, Vec<_>> = HashMap::new();
    for (&source, dests) in &adj_list1 {
        for &dest in dests {
            for &dest2 in &adj_list1[&dest] {
                if dest2 == source {
                    // can't go backwards
                    continue;
                }
                adj_list2
                    .entry((source, dest))
                    .or_default()
                    .push((dest, dest2));
            }
        }
    }
    let mut adj_list3: HashMap<_, Vec<_>> = HashMap::new();
    for (source, dests) in &adj_list2 {
        for dest in dests {
            for dest2 in &adj_list2[&dest] {
                // imagine source = (a, b); dest = (b, c); dest2 = (c, d)
                // We are constructing new_source = (a, b, c); new_dest = (b, c, d)
                let new_source = (source.0, dest.0, dest2.0);
                let new_dest = (source.1, dest.1, dest2.1);
                adj_list3.entry(new_source).or_default().push(new_dest);
            }
        }
    }
    let mut adj_list4: HashMap<_, Vec<_>> = HashMap::new();
    for (source, dests) in &adj_list3 {
        for dest in dests {
            for dest2 in &adj_list3[&dest] {
                // imagine source = (a, b, c); dest = (b, c, d); dest2 = (c, d, e)
                // We are constructing new_source = (a, b, c, d); new_dest = (b, c, d, e)
                let a = source.0;
                let b = dest.0;
                let c = dest2.0;
                let d = dest2.1;
                let e = dest2.2;
                let new_source: (Option<Node>, Option<Node>, Option<Node>, Option<Node>) =
                    (Some(a), Some(b), Some(c), Some(d));
                let new_dest: (Option<Node>, Option<Node>, Option<Node>, Option<Node>) =
                    (Some(b), Some(c), Some(d), Some(e));
                if a.pos.0 == b.pos.0
                    && b.pos.0 == c.pos.0
                    && c.pos.0 == d.pos.0
                    && d.pos.0 == e.pos.0
                {
                    // invalid horizontally
                    continue;
                }
                if a.pos.1 == b.pos.1
                    && b.pos.1 == c.pos.1
                    && c.pos.1 == d.pos.1
                    && d.pos.1 == e.pos.1
                {
                    // invalid vertically
                    continue;
                }

                adj_list4.entry(new_source).or_default().push(new_dest);
            }
        }
    }
    // println!(
    //     "{:?}",
    //     &adj_list3[&(
    //         Some(Node {
    //             cost: 3,
    //             pos: (3, 0)
    //         }),
    //         Some(Node {
    //             cost: 4,
    //             pos: (4, 0)
    //         }),
    //         Some(Node {
    //             cost: 3,
    //             pos: (5, 0)
    //         })
    //     )]
    // );
    let start_node = Node {
        cost: map[0][0],
        pos: (0, 0),
    };
    let keys_with_start: Vec<_> = adj_list4
        .keys()
        .filter(|(a, _, _, _)| a == &Some(start_node))
        .copied()
        .collect();
    for (a, b, c, d) in keys_with_start {
        adj_list4
            .entry((None, a, b, c))
            .or_default()
            .push((a, b, c, d));
        adj_list4
            .entry((None, None, a, b))
            .or_default()
            .push((None, a, b, c));
        adj_list4
            .entry((None, None, None, a))
            .or_default()
            .push((None, None, a, b));
    }
    let start_node: (Option<Node>, Option<Node>, Option<Node>, Option<Node>) =
        (None, None, None, Some(start_node));
    let dest = (map[0].len() - 1, map.len() - 1);
    let path = astar(
        &adj_list4,
        &start_node,
        |(_, _, _, c)| {
            if let Some(Node { pos, .. }) = c {
                pos == &dest
            } else {
                false
            }
        },
        |(_, _, _, c)| {let c = c.unwrap(); (c.pos.0.abs_diff(dest.0) + c.pos.1.abs_diff(dest.1)) as u32},
        |(_, _, _, c)| c.unwrap().cost,
    );
    let path: Vec<_> = path.iter().map(|(_, _, _, c)| c.unwrap()).collect();
    for elem in &path {
        println!("{:?}", elem.pos);
    }
    let cost = path
        .iter()
        .map(|elem| elem.cost as usize)
        .sum::<usize>()
        .checked_sub(2)
        .unwrap();
    println!("{cost}");
}
