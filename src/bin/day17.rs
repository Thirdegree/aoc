#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Hash, Eq, Debug, Copy, Clone)]
struct Node {
    cost: u32,
    pos: (usize, usize),
}

fn astar<T, G, W, W2>(
    adj_list: &HashMap<T, Vec<T>>,
    start: &T,
    is_goal: G,
    heuristic: W,
    weight: W2,
) -> Vec<T>
where
    T: Eq + std::hash::Hash + Clone + std::fmt::Debug,
    W: Fn(&T) -> u32,
    W2: Fn(&T) -> u32,
    G: Fn(&T) -> bool,
{
    let mut to_visit = HashSet::new();
    to_visit.insert(start.clone());
    let mut camefrom: HashMap<T, T> = HashMap::new();
    let mut g_score: HashMap<T, u32> = HashMap::new();
    g_score.insert(start.clone(), 0);
    let mut f_score: HashMap<T, u32> = HashMap::new();
    f_score.insert(start.clone(), heuristic(start));

    let mut i = 0;
    while !to_visit.is_empty() {
        i += 1;
        if i % 200 == 0 {
            println!("n to visit? {}", to_visit.len());
        }
        let current = to_visit
            .iter()
            .min_by_key(|elem| *f_score.get(elem).unwrap_or(&u32::MAX))
            .unwrap()
            .clone();
        if is_goal(&current) {
            let mut current = current;
            let mut path = vec![current.clone()];
            while let Some(came) = camefrom.get(&current) {
                path.push(came.clone());
                current = came.clone();
            }
            return path.iter().rev().cloned().collect();
        }
        to_visit.remove(&current);
        for neighbor in &adj_list[&current] {
            let tentative_gscore = g_score[&current] + weight(neighbor);
            if let Some(&old_score) = g_score.get(neighbor) {
                if tentative_gscore < old_score {
                    camefrom.insert(neighbor.clone(), current.clone());
                    to_visit.insert(neighbor.clone());
                    g_score.insert(neighbor.clone(), tentative_gscore);
                    f_score.insert(neighbor.clone(), tentative_gscore + heuristic(neighbor));
                }
            } else {
                g_score.insert(neighbor.clone(), tentative_gscore);
                f_score.insert(neighbor.clone(), tentative_gscore + heuristic(neighbor));
                camefrom.insert(neighbor.clone(), current.clone());
                to_visit.insert(neighbor.clone());
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
                let new_source: (Option<Node>, Option<Node>, Option<Node>, Option<Node>) =
                    (Some(source.0), Some(dest.0), Some(dest2.0), Some(dest2.1));
                let new_dest: (Option<Node>, Option<Node>, Option<Node>, Option<Node>) =
                    (Some(source.1), Some(dest.1), Some(dest2.1), Some(dest2.2));
                if source.0.pos.0 == dest.0.pos.0
                    && dest.0.pos.0 == dest2.0.pos.0
                    && dest2.0.pos.0 == dest2.1.pos.0
                    && dest2.1.pos.0 == dest2.2.pos.0
                {
                    // invalid horizontally
                    continue;
                }
                if source.0.pos.1 == dest.0.pos.1
                    && dest.0.pos.1 == dest2.0.pos.1
                    && dest2.0.pos.1 == dest2.1.pos.1
                    && dest2.1.pos.1 == dest2.2.pos.1
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
        println!("foo {a:?}, {b:?}, {c:?}, {d:?}");
        adj_list4
            .entry((None, a, b, c))
            .or_default()
            .push((a, b, c, d));
    }
    let keys_with_start: Vec<_> = adj_list4
        .keys()
        .filter(|(a, b, _, _)| (a, b) == (&None, &Some(start_node)))
        .copied()
        .collect();
    for (a, b, c, d) in keys_with_start {
        println!("{a:?}, {b:?}, {c:?}, {d:?}");
        adj_list4
            .entry((None, a, b, c))
            .or_default()
            .push((a, b, c, d));
    }
    let keys_with_start: Vec<_> = adj_list4
        .keys()
        .filter(|(a, b, c, _)| (a, b, c) == (&None, &None, &Some(start_node)))
        .copied()
        .collect();
    for (a, b, c, d) in keys_with_start {
        println!("ccccc");
        adj_list4
            .entry((None, a, b, c))
            .or_default()
            .push((a, b, c, d));
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
        |(_, _, _, c)| {
            let pos = c.unwrap().pos;
            (pos.0.abs_diff(dest.0) + pos.1.abs_diff(dest.1)) as u32
        },
        |(_, _, _, c)| c.unwrap().cost,
    );
    let path: Vec<_> = path.iter().map(|(_, _, _, c)| c.unwrap()).collect();
    println!("{path:?}");
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
