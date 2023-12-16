#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fmt::Display;

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

struct Light {
    current_direction: Direction,
    current_possition: (usize, usize),
}

impl Light {
    fn next_step(&self, cur_space: Option<char>) -> Vec<Self> {
        cur_space.map_or_else(Vec::new, |space| {
            match space {
                '|' if matches!(self.current_direction, Direction::Left | Direction::Right) => {
                    [Some(Direction::Up), Some(Direction::Down)]
                }

                '-' if matches!(self.current_direction, Direction::Down | Direction::Up) => {
                    [Some(Direction::Left), Some(Direction::Right)]
                }

                '/' => [
                    Some(match self.current_direction {
                        Direction::Right => Direction::Up,
                        Direction::Up => Direction::Right,
                        Direction::Left => Direction::Down,
                        Direction::Down => Direction::Left,
                    }),
                    None,
                ],
                '\\' => [
                    Some(match self.current_direction {
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Up => Direction::Left,
                    }),
                    None,
                ],
                _ => [Some(self.current_direction.clone()), None],
            }
            .into_iter()
            .flatten()
            .filter_map(|dir| {
                let new_pos = dir.next(self.current_possition)?;
                Some(Self {
                    current_direction: dir,
                    current_possition: new_pos,
                })
            })
            .collect()
        })
    }
}

struct Board {
    light: Vec<Light>,
    board: aoc_2023::TwoDArray<char>,
    seen_directions: aoc_2023::TwoDArray<Vec<Direction>>,
}

impl Board {
    fn reset_with(&mut self, light: Light) {
        self.seen_directions
            .rows_mut()
            .for_each(|row| row.iter_mut().for_each(Vec::clear));
        self.light.clear();
        self.seen_directions[light.current_possition].push(light.current_direction.clone());
        self.light.push(light);
    }
    fn edges(&self) -> Vec<(usize, usize)> {
        let x_edge = self.board.x_len() - 1;
        let y_edge = self.board.y_len() - 1;
        (0..self.board.x_len())
            .flat_map(|y| (0..self.board[0].len()).map(move |x| (x, y)))
            .filter(|&(x, y)| x == 0 || x == x_edge || y == 0 || y == y_edge)
            .collect()
    }
    fn start_at(&self, pos: (usize, usize)) -> Vec<Light> {
        let y_edge = self.board.y_len() - 1;
        let x_edge = self.board.x_len() - 1;
        let dirs = match pos {
            (0, 0) => [Some(Direction::Down), Some(Direction::Right)], // Top left
            (0, y) if y == y_edge => [Some(Direction::Up), Some(Direction::Right)], // Top right
            (x, 0) if x == x_edge => [Some(Direction::Down), Some(Direction::Left)], // bottom left
            (x, y) if y == y_edge && x == x_edge => [Some(Direction::Up), Some(Direction::Left)], // bottomright
            (0, _) => [Some(Direction::Right), None], // left side
            (x, _) if x == x_edge => [Some(Direction::Left), None], // right side
            (_, 0) => [Some(Direction::Down), None],  // top
            (_, y) if y == y_edge => [Some(Direction::Up), None], // bottom
            _ => unreachable!(),                      // NO
        };
        let mut lights = vec![];
        for dir in dirs.into_iter().flatten() {
            lights.push(Light {
                current_possition: pos,
                current_direction: dir,
            });
        }
        lights
    }
    fn step(&mut self) -> bool {
        let mut all_new_light = vec![];
        let mut new_light = false;
        for light in &self.light {
            let next_lights = light.next_step(Some(self.board[light.current_possition]));
            for next_light in next_lights {
                if !self.board.is_within_bounds(next_light.current_possition) {
                    continue;
                }
                if self.seen_directions[next_light.current_possition]
                    .contains(&next_light.current_direction)
                {
                    continue;
                }
                new_light = true;
                self.seen_directions[next_light.current_possition]
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

        for (line, seen) in board.rows_mut().zip(&self.seen_directions.elems) {
            for (elem, dirs) in line.iter_mut().zip(seen) {
                if dirs.is_empty() {
                    *elem = '.';
                } else {
                    *elem = '#';
                }
            }
        }
        let lines: Vec<_> = board.rows().map(|l| l.iter().collect::<String>()).collect();
        println!("{}", lines.join("\n"));
    }
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let seen_directions = value
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

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = self.board.clone();
        for light in &self.light {
            if matches!(lines[light.current_possition], '.') {
                lines[light.current_possition] = match light.current_direction {
                    Direction::Up => '^',
                    Direction::Down => 'V',
                    Direction::Left => '<',
                    Direction::Right => '>',
                }
            }
        }
        let lines: Vec<_> = lines.rows().map(|l| l.iter().collect::<String>()).collect();
        f.write_str(lines.join("\n").as_str())
    }
}

fn main() {
    let mut board: Board = aoc_2023::include_data!(day16).into();
    let mut energized = 0;
    for pos in board.edges() {
        for light in board.start_at(pos) {
            board.reset_with(light);
            while board.step() {}
            energized = energized.max(
                board
                    .seen_directions
                    .elems()
                    .filter(|l| !l.is_empty())
                    .count(),
            );
        }
    }
    println!("Day 16 result: {energized}");
}
