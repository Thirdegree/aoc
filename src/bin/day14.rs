#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{collections::HashSet, fmt::Display};

#[derive(Clone, Hash, Eq, PartialEq)]
enum Space {
    Round,
    Square,
    Empty,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Round,
            '#' => Self::Square,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

struct Board {
    elems: Vec<Vec<Space>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<_> = self
            .elems
            .iter()
            .map(|row| {
                row.iter()
                    .map(|space| match space {
                        Space::Round => 'O',
                        Space::Square => '#',
                        Space::Empty => '.',
                    })
                    .collect::<String>()
            })
            .collect();
        f.write_str(lines.join("\n").as_str())
    }
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        Self {
            elems: value
                .lines()
                .map(|l| l.chars().map(Into::into).collect())
                .collect(),
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Board {
    fn load(&self) -> usize {
        let n_rows = self.elems.len();
        self.elems
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                let n_rocks = row.iter().filter(|&s| matches!(s, Space::Round)).count();
                n_rocks * (n_rows - idx)
            })
            .sum()
    }
    fn cycle(&mut self) {
        self.tilt(&Direction::North);
        self.tilt(&Direction::West);
        self.tilt(&Direction::South);
        self.tilt(&Direction::East);
    }
    #[allow(
        clippy::too_many_lines,
        clippy::cognitive_complexity,
        clippy::needless_range_loop
    )]
    fn tilt(&mut self, direction: &Direction) {
        match direction {
            Direction::North => {
                let mut free_spaces = vec![None; self.elems[0].len()];
                let n_rows = self.elems.len();
                let n_cols = self.elems[0].len();
                for yidx in 0..n_rows {
                    for xidx in 0..n_cols {
                        let elem = &self.elems[yidx][xidx];
                        match elem {
                            Space::Square => free_spaces[xidx] = None,
                            Space::Empty => {
                                if free_spaces[xidx].is_none() {
                                    free_spaces[xidx] = Some(yidx);
                                }
                            }
                            Space::Round => {
                                if let Some(new_y) = free_spaces[xidx] {
                                    free_spaces[xidx] = Some(new_y + 1);
                                    self.elems[new_y][xidx] = Space::Round;
                                    self.elems[yidx][xidx] = Space::Empty;
                                }
                            }
                        }
                    }
                }
            }
            Direction::South => {
                let mut free_spaces = vec![None; self.elems[0].len()];
                let n_rows = self.elems.len();
                let n_cols = self.elems[0].len();
                for yidx in (0..n_rows).rev() {
                    for xidx in 0..n_cols {
                        let elem = &self.elems[yidx][xidx];
                        match elem {
                            Space::Square => free_spaces[xidx] = None,
                            Space::Empty => {
                                if free_spaces[xidx].is_none() {
                                    free_spaces[xidx] = Some(yidx);
                                }
                            }
                            Space::Round => {
                                if let Some(new_y) = free_spaces[xidx] {
                                    free_spaces[xidx] = Some(new_y - 1);
                                    self.elems[new_y][xidx] = Space::Round;
                                    self.elems[yidx][xidx] = Space::Empty;
                                }
                            }
                        }
                    }
                }
            }
            Direction::West => {
                let mut free_spaces = vec![None; self.elems.len()];
                let n_rows = self.elems.len();
                let n_cols = self.elems[0].len();
                for xidx in 0..n_cols {
                    for yidx in 0..n_rows {
                        let elem = &self.elems[yidx][xidx];
                        match elem {
                            Space::Square => free_spaces[yidx] = None,
                            Space::Empty => {
                                if free_spaces[yidx].is_none() {
                                    free_spaces[yidx] = Some(xidx);
                                }
                            }
                            Space::Round => {
                                if let Some(new_x) = free_spaces[yidx] {
                                    free_spaces[yidx] = Some(new_x + 1);
                                    self.elems[yidx][new_x] = Space::Round;
                                    self.elems[yidx][xidx] = Space::Empty;
                                }
                            }
                        }
                    }
                }
            }
            Direction::East => {
                let mut free_spaces = vec![None; self.elems.len()];
                let n_rows = self.elems.len();
                let n_cols = self.elems[0].len();
                for xidx in (0..n_cols).rev() {
                    for yidx in 0..n_rows {
                        let elem = &self.elems[yidx][xidx];
                        match elem {
                            Space::Square => free_spaces[yidx] = None,
                            Space::Empty => {
                                if free_spaces[yidx].is_none() {
                                    free_spaces[yidx] = Some(xidx);
                                }
                            }
                            Space::Round => {
                                if let Some(new_x) = free_spaces[yidx] {
                                    free_spaces[yidx] = Some(new_x - 1);
                                    self.elems[yidx][new_x] = Space::Round;
                                    self.elems[yidx][xidx] = Space::Empty;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut board: Board = aoc_2023::include_data!(day14, sample).into();
    // guessing that total number of enountered boardstates is < 2000, purely for performance
    // benefits
    let mut known_boardstates = HashSet::with_capacity(1_000);
    let mut till_first_cycle_starts = None;
    let mut main_cycle_len = None;
    let target = 1_000_000_000;
    for i in 0.. {
        board.cycle();
        if known_boardstates.contains(&board.elems) {
            if let Some(first_cycle) = till_first_cycle_starts {
                main_cycle_len = Some(i - first_cycle);
                break;
            }
            till_first_cycle_starts = Some(i);
            known_boardstates.clear();
        }
        known_boardstates.insert(board.elems.clone());
    }
    let till_first_cycle_starts = till_first_cycle_starts.unwrap();
    let main_cycle_len = main_cycle_len.unwrap();
    let target_minus_init = target - till_first_cycle_starts;
    let remaining = target_minus_init % main_cycle_len;

    // -1 because we've done till_first_cycle_starts + main_cycle_len + 1 by a "quirk" of how the loop above was written
    // (by which I mean, confusingly)
    for _ in 0..remaining - 1 {
        board.cycle();
    }
    // println!("{board}");
    println!("Day 14 result: {}", board.load());
}
