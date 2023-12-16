#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{fmt::Display, time::Duration};

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn next(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        Some(match self {
            Self::Left => (pos.0.checked_sub(1)?, pos.1),
            Self::Right => (pos.0 + 1, pos.1),
            Self::Up => (pos.0, pos.1.checked_sub(1)?),
            Self::Down => (pos.0, pos.1 + 1),
        })
    }
}

#[derive(Clone)]
struct Light {
    current_direction: Direction,
    current_possition: (usize, usize),
}

impl Light {
    fn next_step(&self, cur_space: Option<char>) -> Vec<Self> {
        let mut next = vec![];
        if let Some(space) = cur_space {
            match space {
                '|' if matches!(self.current_direction, Direction::Left | Direction::Right) => {
                    for dir in [Direction::Up, Direction::Down] {
                        if let Some(new_pos) = dir.next(self.current_possition) {
                            next.push(Self {
                                current_direction: dir,
                                current_possition: new_pos,
                            });
                        }
                    }
                }
                '-' if matches!(self.current_direction, Direction::Down | Direction::Up) => {
                    for dir in [Direction::Left, Direction::Right] {
                        if let Some(new_pos) = dir.next(self.current_possition) {
                            next.push(Self {
                                current_direction: dir,
                                current_possition: new_pos,
                            });
                        }
                    }
                }
                '/' => {
                    let new_direction = match self.current_direction {
                        Direction::Right => Direction::Up,
                        Direction::Up => Direction::Right,
                        Direction::Left => Direction::Down,
                        Direction::Down => Direction::Left,
                    };
                    if let Some(new_pos) = new_direction.next(self.current_possition) {
                        next.push(Self {
                            current_direction: new_direction,
                            current_possition: new_pos,
                        });
                    }
                }
                '\\' => {
                    let new_direction = match self.current_direction {
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Up => Direction::Left,
                    };
                    if let Some(new_pos) = new_direction.next(self.current_possition) {
                        next.push(Self {
                            current_direction: new_direction,
                            current_possition: new_pos,
                        });
                    }
                }
                _ => {
                    if let Some(new_pos) = self.current_direction.next(self.current_possition) {
                        next.push(Self {
                            current_direction: self.current_direction.clone(),
                            current_possition: new_pos,
                        });
                    }
                }
            }
        }
        next
    }
}

#[derive(Clone)]
struct Board {
    light: Vec<Light>,
    board: Vec<Vec<char>>,
    seen_directions: Vec<Vec<Vec<Direction>>>,
}

impl Board {
    fn edges(&self) -> Vec<(usize, usize)> {
        let x_edge = self.board[0].len() - 1;
        let y_edge = self.board.len() - 1;
        (0..self.board.len())
            .flat_map(|y| (0..self.board[0].len()).map(move |x| (x, y)))
            .filter(|&(x, y)| x == 0 || x == x_edge || y == 0 || y == y_edge)
            .collect()
    }
    fn start_at(&self, pos: (usize, usize)) -> Vec<Self> {
        let y_edge = self.board.len() - 1;
        let x_edge = self.board[0].len() - 1;
        let dirs = match pos {
            (0, 0) => vec![Direction::Down, Direction::Right],
            (0, y) if y == y_edge => vec![Direction::Up, Direction::Right],
            (x, 0) if x == x_edge => vec![Direction::Down, Direction::Left],
            (x, y) if y == y_edge && x == x_edge => vec![Direction::Up, Direction::Left],
            (0, _) => vec![Direction::Right],
            (x, _) if x == x_edge => vec![Direction::Left],
            (_, 0) => vec![Direction::Down],
            (_, y) if y == y_edge => vec![Direction::Up],
            _ => unreachable!(),
        };
        let mut boards = vec![];
        for dir in dirs {
            let mut new_board = self.clone();
            new_board.light.push(Light {
                current_possition: pos,
                current_direction: dir.clone(),
            });
            new_board.seen_directions[pos.1][pos.0].push(dir);
            boards.push(new_board);
        }
        boards
    }
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let seen_directions: Vec<Vec<_>> = value
            .lines()
            .map(|l| l.chars().map(|_| vec![]).collect())
            .collect();
        Self {
            light: vec![],
            board: value.lines().map(|l| l.chars().collect()).collect(),
            seen_directions,
        }
    }
}

impl Board {
    fn step(&mut self) -> bool {
        let mut all_new_light = vec![];
        let mut new_light = false;
        for light in &self.light {
            let next_lights = light.next_step(Some(
                self.board[light.current_possition.1][light.current_possition.0],
            ));
            for next_light in next_lights {
                if next_light.current_possition.1 >= self.board.len() {
                    continue;
                }
                if next_light.current_possition.0 >= self.board[0].len() {
                    continue;
                }
                if self.seen_directions[next_light.current_possition.1]
                    [next_light.current_possition.0]
                    .contains(&next_light.current_direction)
                {
                    continue;
                }
                new_light = true;
                self.seen_directions[next_light.current_possition.1]
                    [next_light.current_possition.0]
                    .push(next_light.current_direction.clone());
                all_new_light.push(next_light);
            }
        }
        self.light = all_new_light;
        new_light
    }
    #[allow(dead_code)]
    fn print_energized(&self) {
        let mut board = self.board.clone();

        for (line, seen) in board.iter_mut().zip(&self.seen_directions) {
            for (elem, dirs) in line.iter_mut().zip(seen) {
                if dirs.is_empty() {
                    *elem = '.';
                } else {
                    *elem = '#';
                }
            }
        }
        let lines: Vec<_> = board.iter().map(|l| l.iter().collect::<String>()).collect();
        println!("{}", lines.join("\n"));
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = self.board.clone();
        for light in &self.light {
            if matches!(
                lines[light.current_possition.1][light.current_possition.0],
                '.'
            ) {
                lines[light.current_possition.1][light.current_possition.0] =
                    match light.current_direction {
                        Direction::Up => '^',
                        Direction::Down => 'V',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    }
            }
        }
        let lines: Vec<_> = lines.iter().map(|l| l.iter().collect::<String>()).collect();
        f.write_str(lines.join("\n").as_str())
    }
}

fn main() {
    let board: Board = aoc_2023::include_data!(day16).into();
    let mut energized = 0;
    for corner in board.edges() {
        for mut board in board.start_at(corner) {
            while board.step() {}
            // board.print_energized();
            // println!();
            energized = energized.max(
                board
                    .seen_directions
                    .iter()
                    .flatten()
                    .filter(|l| !l.is_empty())
                    .count(),
            );
        }
    }
    // let b = &mut board.start_at((3, 0))[0];
    // while b.step() {
    //     println!("{b}");
    //     println!();
    //     std::thread::sleep(Duration::from_millis(100));
    // }
    // b.print_energized();
    println!("Day 16 result: {energized}");
    // board.print_energized();
}
