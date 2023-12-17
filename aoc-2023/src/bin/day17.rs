#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

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

fn nexts<F>(weight: &F, state: &State) -> Vec<(i32, State)>
where
    F: Fn(&Pos) -> Option<i32>,
{
    let mut nexts = vec![];
    let next_dirs = state
        .1
        .as_ref()
        .map_or_else(|| vec![Direction::Down, Direction::Right], Direction::turns);
    if let Some(dir) = &state.1 {
        if state.2 < 10 {
            if let Some(pos) = dir.add(state.0, 1) {
                let new_state = (pos, Some(dir.clone()), state.2 + 1);
                if let Some(w) = weight(&pos) {
                    nexts.push((w, new_state));
                }
            }
        }
    }
    for dir in next_dirs {
        if let Some(pos) = dir.add(state.0, 4) {
            // dbg!(pos);
            // ok soooo this looks a lil groes, but doing it differently would have been
            // worse
            let w = || {
                Some(match dir {
                    Direction::Up => {
                        weight(&pos)?
                            + weight(&(pos.0, pos.1 + 1))?
                            + weight(&(pos.0, pos.1 + 2))?
                            + weight(&(pos.0, pos.1 + 3))?
                    }
                    Direction::Down => {
                        weight(&pos)?
                            + weight(&(pos.0, pos.1 - 1))?
                            + weight(&(pos.0, pos.1 - 2))?
                            + weight(&(pos.0, pos.1 - 3))?
                    }
                    Direction::Left => {
                        weight(&pos)?
                            + weight(&(pos.0 + 1, pos.1))?
                            + weight(&(pos.0 + 2, pos.1))?
                            + weight(&(pos.0 + 3, pos.1))?
                    }
                    Direction::Right => {
                        weight(&pos)?
                            + weight(&(pos.0 - 1, pos.1))?
                            + weight(&(pos.0 - 2, pos.1))?
                            + weight(&(pos.0 - 3, pos.1))?
                    }
                })
            };
            if let Some(w) = w() {
                nexts.push((w, (pos, Some(dir), 4)));
            }
        }
    }

    nexts
}

fn main() {
    let map: Vec<Vec<_>> = aoc_helpers::include_data!(day17)
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| i32::try_from(c.to_digit(10).unwrap()).unwrap())
                .collect()
        })
        .collect();
    let dest = (map[0].len() - 1, map.len() - 1);
    let (_path, result) = aoc_helpers::math::astar(
        &((0, 0), None, 0),
        nexts,
        |elem| elem.0 == dest,
        move |pos: &Pos| Some(*map.get(pos.1)?.get(pos.0)?),
        |(pos, _, _)| i32::try_from(pos.0.abs_diff(dest.0) + pos.1.abs_diff(dest.1)).unwrap(),
    );
    // println!("{path:?}");
    println!("Day 17 result: {result}");
}
