#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

use priority_queue::PriorityQueue;

type Pos = (usize, usize);
type State = (Pos, Option<Direction>, usize);

#[derive(PartialEq, Hash, Eq, Debug, Copy, Clone)]
struct Node {
    cost: i32,
    pos: Pos,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turns(&self) -> Vec<Self> {
        match self {
            Self::Up | Self::Down => vec![Self::Left, Self::Right],
            Self::Left | Self::Right => vec![Self::Up, Self::Down],
        }
    }
    fn add(&self, pos: Pos, n_steps: usize) -> Option<Pos> {
        match self {
            Self::Up => Some((pos.0, pos.1.checked_sub(n_steps)?)),
            Self::Down => Some((pos.0, pos.1 + n_steps)),
            Self::Left => Some((pos.0.checked_sub(n_steps)?, pos.1)),
            Self::Right => Some((pos.0 + n_steps, pos.1)),
        }
    }
}

fn nexts(map: &Vec<Vec<i32>>, state: &State) -> Vec<(i32, State)> {
    let mut nexts = vec![];
    match &state.1 {
        // first step
        None => {
            for dir in [Direction::Down, Direction::Right] {
                if let Some(pos) = dir.add(state.0, 4) {
                    // dbg!(pos);
                    if pos.1 >= map.len() || pos.0 >= map[0].len() {
                        continue;
                    }
                    let w = match dir {
                        Direction::Up => {
                            map[pos.1][pos.0]
                                + map[pos.1 + 1][pos.0]
                                + map[pos.1 + 2][pos.0]
                                + map[pos.1 + 3][pos.0]
                        }
                        Direction::Down => {
                            map[pos.1][pos.0]
                                + map[pos.1 - 1][pos.0]
                                + map[pos.1 - 2][pos.0]
                                + map[pos.1 - 3][pos.0]
                        }
                        Direction::Left => {
                            map[pos.1][pos.0]
                                + map[pos.1][pos.0 + 1]
                                + map[pos.1][pos.0 + 2]
                                + map[pos.1][pos.0 + 3]
                        }
                        Direction::Right => {
                            map[pos.1][pos.0]
                                + map[pos.1][pos.0 - 1]
                                + map[pos.1][pos.0 - 2]
                                + map[pos.1][pos.0 - 3]
                        }
                    };
                    nexts.push((w, (pos, Some(dir), 4)));
                }
            }
        }
        // all other steps
        Some(dir) => {
            // 3 cases: After a turn, MUST go 4 spaces
            //          >=10, must turn,
            //          anything else, can do whatever it wants
            if state.2 < 10 {
                if let Some(pos) = dir.add(state.0, 1) {
                    if !(pos.1 >= map.len() || pos.0 >= map[0].len()) {
                        nexts.push((map[pos.1][pos.0], (pos, Some(dir.clone()), state.2 + 1)));
                    }
                }
            }
            for dir in dir.turns() {
                if let Some(pos) = dir.add(state.0, 4) {
                    if pos.1 >= map.len() || pos.0 >= map[0].len() {
                        continue;
                    }
                    // dbg!(state.0, &dir, pos);
                    let w = match dir {
                        Direction::Up => {
                            map[pos.1][pos.0]
                                + map[pos.1 + 1][pos.0]
                                + map[pos.1 + 2][pos.0]
                                + map[pos.1 + 3][pos.0]
                        }
                        Direction::Down => {
                            map[pos.1][pos.0]
                                + map[pos.1 - 1][pos.0]
                                + map[pos.1 - 2][pos.0]
                                + map[pos.1 - 3][pos.0]
                        }
                        Direction::Left => {
                            map[pos.1][pos.0]
                                + map[pos.1][pos.0 + 1]
                                + map[pos.1][pos.0 + 2]
                                + map[pos.1][pos.0 + 3]
                        }
                        Direction::Right => {
                            map[pos.1][pos.0]
                                + map[pos.1][pos.0 - 1]
                                + map[pos.1][pos.0 - 2]
                                + map[pos.1][pos.0 - 3]
                        }
                    };
                    nexts.push((w, (pos, Some(dir), 4)));
                }
            }
        }
    }
    nexts
}

fn astar(map: &Vec<Vec<i32>>, state: State, end: &Pos) -> (Vec<Pos>, i32) {
    let mut pqueue = PriorityQueue::new();
    pqueue.push(state.clone(), 0);
    let mut dists = HashMap::new();
    dists.insert(state, 0);
    let mut points_to: HashMap<State, State> = HashMap::new();
    while let Some(((pos, dir, steps), dist)) = pqueue.pop() {
        if &pos == end {
            let mut state = (pos, dir, steps);
            let mut path = vec![state.0];
            while let Some(new_state) = points_to.get(&state) {
                path.push(new_state.0);
                state = new_state.clone();
            }
            return (path.into_iter().rev().collect(), -dist);
        }
        for (cost, state) in nexts(map, &(pos, dir.clone(), steps)) {
            if -dist + cost < *dists.get(&state).unwrap_or(&i32::MAX) {
                points_to.insert(state.clone(), (pos, dir.clone(), steps));
                dists.insert(state.clone(), -dist + cost);
                pqueue.push(state, -(-dist + cost));
            }
        }
    }
    unreachable!()
}

#[allow(clippy::too_many_lines)]
fn main() {
    let map: Vec<Vec<_>> = aoc_2023::include_data!(day17)
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let (_path, result) = astar(&map, ((0, 0), None, 0), &(map[0].len() - 1, map.len() - 1));
    // println!("{path:?}");
    println!("Day 17 result: {result}");
}
