#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    const fn add(&self, pos: Pos, length: i64) -> Pos {
        match self {
            Self::Down => (pos.0, pos.1 + length),
            Self::Up => (pos.0, pos.1 - length),
            Self::Right => (pos.0 + length, pos.1),
            Self::Left => (pos.0 - length, pos.1),
        }
    }
}

type Pos = (i64, i64);

#[derive(Debug)]
struct Trench {
    direction: Direction,
    start: Pos,
    end: Pos,
}

impl Trench {
    fn next_trench(&self, direction: Direction, length: i64) -> Self {
        match self.direction {
            Direction::Up | Direction::Down => {
                if matches!(direction, Direction::Up | Direction::Down) {
                    panic!("Invalid, same dir twice")
                }
            }
            Direction::Left | Direction::Right => {
                if matches!(direction, Direction::Left | Direction::Right) {
                    panic!("Invalid, same dir twice")
                }
            }
        }
        let end = direction.add(self.end, length);
        Self {
            direction,
            start: self.end,
            end,
        }
    }
}

#[allow(dead_code)]
const fn interpret_data_part_1(dir: Direction, length: i64, _color: &str) -> (Direction, i64) {
    (dir, length)
}
#[allow(dead_code, clippy::needless_pass_by_value)]
fn interpret_data_part_2(_dir: Direction, _length: i64, color: &str) -> (Direction, i64) {
    let direction = match &color[6..] {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => unreachable!(),
    };
    let length = i64::from_str_radix(&color[1..6], 16).unwrap();
    (direction, length)
}

// All credit to
// https://www.reddit.com/r/adventofcode/comments/18l8mao/2023_day_18_intuition_for_why_spoiler_alone/,
// I do not math good.
// Once I found that, first try was correct.
fn main() {
    let lines: Vec<(Direction, i64, &str)> = aoc_helpers::include_data!(day18)
        .lines()
        .map(|line| {
            let mut l = line.split_whitespace();
            let parens = ['(', ')'];
            (
                l.next().unwrap().into(),
                l.next().unwrap().parse().unwrap(),
                l.next().unwrap().trim_matches(&parens[..]),
            )
        })
        .collect();
    let mut trenches: Vec<Trench> = vec![];
    for (direction, length, color) in lines {
        let (direction, length) = interpret_data_part_2(direction, length, color);
        if let Some(last_trench) = trenches.last() {
            trenches.push(last_trench.next_trench(direction, length));
        } else {
            let end = direction.add((0, 0), length);
            trenches.push(Trench {
                direction,
                start: (0, 0),
                end,
            });
        }
    }
    let mut coords: Vec<_> = trenches.iter().map(|t| t.start).collect();
    coords.push(trenches[0].start);
    let shoelace: i64 = coords
        .windows(3)
        .map(|ts| {
            let t1 = ts[0];
            let t2 = ts[1];
            let t3 = ts[2];
            t2.1 * (t1.0 - t3.0)
        })
        .sum(); // shoelace formula, https://en.wikipedia.org/wiki/Shoelace_formula
    let perim_area = trenches
        .iter()
        .map(|t| match t.direction {
            Direction::Up | Direction::Down => i64::try_from(t.start.1.abs_diff(t.end.1)).unwrap(),
            Direction::Left | Direction::Right => {
                i64::try_from(t.start.0.abs_diff(t.end.0)).unwrap()
            }
        })
        .sum::<i64>();
    println!("Day 18 result: {}", (shoelace / 2) + (perim_area / 2) + 1); // magic bs, see reddit
                                                                          // post at top of main
}
