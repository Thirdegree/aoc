#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Self::Right,
            "L" => Self::Left,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Trench {
    dir: Direction,
    len: i32,
}

impl Trench {
    fn from_hex_string(hex: &str) -> Self {
        let dist_in_hex = &hex[1..6];
        let direction_as_digit = &hex[6..];
        Self {
            len: i32::from_str_radix(dist_in_hex, 16).unwrap(),
            dir: match direction_as_digit {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => unreachable!(),
            },
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<&str> for Trench {
    fn from(value: &str) -> Self {
        let mut ins = value.split_whitespace();
        let dir: Direction = ins.next().unwrap().into();
        let len = ins.next().unwrap().parse().unwrap();
        let color = ins.next().unwrap().to_string();
        let parens = &['(', ')'];
        let _color = color.trim_matches(&parens[..]).to_string();
        Self { dir, len }
    }
    // fn from(value: &str) -> Self {
    //     let mut ins = value.split_whitespace();
    //     let _dir: Direction = ins.next().unwrap().into();
    //     let _len: i32 = ins.next().unwrap().parse().unwrap();
    //     let color = ins.next().unwrap().to_string();
    //     let parens = &['(', ')'];
    //     let color = color.trim_matches(&parens[..]).to_string();
    //     Self::from_hex_string(&color)
    // }
}

#[derive(Debug)]
struct Problem {
    trenches: Vec<Trench>,
}

type Pos = (i32, i32);

#[derive(Debug)]
struct Path {
    points: Vec<(Trench, Pos)>,
    max_x: i32,
    min_x: i32,
    max_y: i32,
    min_y: i32,
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = self
            .grid()
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let s: String = row.iter().collect();
                format!("{:0>3}: ", i) + s.as_str()
            })
            .collect();
        f.write_str(lines.join("\n").as_str())
    }
}

fn neighbors(pos: (usize, usize)) -> Vec<(usize, usize)> {
    vec![
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ]
}

impl Path {
    #[allow(clippy::too_many_lines)]
    fn filled_grid_area(&self) -> Vec<u64> {
        println!("n_trenches: {}", self.points.len());
        let y_incr = i32::try_from(self.min_y.abs_diff(0)).unwrap();
        let x_incr = i32::try_from(self.min_x.abs_diff(0)).unwrap();
        let mut trench_by_row: Vec<Vec<(Direction, u64, bool)>> =
            vec![vec![]; (self.max_y + y_incr + 1) as usize];
        for (trench, starts_at) in &self.points {
            let starts_at = (
                (starts_at.0 + x_incr) as usize,
                (starts_at.1 + y_incr) as usize,
            );
            match trench.dir {
                Direction::Left | Direction::Right => (),
                Direction::Up => {
                    trench_by_row[starts_at.1 - trench.len as usize].push((
                        Direction::Up,
                        starts_at.0 as u64,
                        true,
                    ));
                    for row_number in starts_at.1 - trench.len as usize + 1..starts_at.1 {
                        trench_by_row[row_number].push((Direction::Up, starts_at.0 as u64, false));
                    }
                    trench_by_row[starts_at.1].push((Direction::Up, starts_at.0 as u64, true));
                }
                Direction::Down => {
                    trench_by_row[starts_at.1].push((Direction::Down, starts_at.0 as u64, true));
                    for row_number in starts_at.1 + 1..starts_at.1 + trench.len as usize {
                        trench_by_row[row_number].push((
                            Direction::Down,
                            starts_at.0 as u64,
                            false,
                        ));
                    }
                    trench_by_row[starts_at.1 + trench.len as usize].push((
                        Direction::Down,
                        starts_at.0 as u64,
                        true,
                    ));
                }
            }
        }
        let trench_by_row = trench_by_row
            .into_iter()
            .map(|mut row| {
                row.sort_by_key(|r| r.1);
                let mut new_row = vec![];
                let mut is_inside = false;
                for r in row {
                    let Some((last_dir, last_xpos, _last_is_terminus)) = new_row.last_mut() else {
                        new_row.push(r);
                        continue;
                    };
                    if last_dir != &r.0 {
                        new_row.push(r);
                        is_inside = !is_inside;
                        continue;
                    }
                    if !is_inside {
                        *last_xpos = r.1;
                    }
                    *_last_is_terminus = false;
                }
                new_row
            })
            .collect::<Vec<_>>();
        // for r in &trench_by_row {
        //     println!("{:?}", r);
        // }
        // println!();
        trench_by_row
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                let mut n_filled = 0;
                let mut last_found = None;
                let mut is_inside = false;
                if i == 128 {
                    println!("{:?}", row);
                }
                for (dir, xpos, mut is_terminus) in row {
                    let Some((last_dir, last_xpos, last_is_terminus)) = &mut last_found else {
                        last_found = Some((dir, xpos, is_terminus));
                        is_inside = true;
                        continue;
                    };
                    assert_ne!(last_dir, &dir, "fucky at i={i}");
                    if i == 128 {
                        println!(
                            "{:?} -> {:?} is_inside? {}",
                            (last_dir, &last_xpos, &last_is_terminus),
                            (&dir, xpos, is_terminus),
                            is_inside
                        );
                    }
                    if is_inside {
                        if i == 128 {
                            println!(
                                "might {} ({} - {} - 1)",
                                xpos - *last_xpos - 1,
                                xpos,
                                last_xpos
                            );
                        }
                        if !(is_terminus && *last_is_terminus) {
                            if i == 128 {
                                println!(
                                    "added {} ({} - {} - 1)",
                                    xpos - *last_xpos - 1,
                                    xpos,
                                    last_xpos
                                );
                            }
                            n_filled += (xpos - *last_xpos) - 1;
                        }
                    }
                    last_found = Some((dir, xpos, if !is_inside { false} else {is_terminus}));
                    is_inside = !is_inside;
                }
                if i == 128 {
                    println!("{}", n_filled);
                }
                n_filled
            })
            .collect()
        // let mut new_trench_by_row: Vec<(u64, Vec<_>)> = vec![];
        // for row in trench_by_row {
        //     if let Some((c, last)) = new_trench_by_row.last_mut() {
        //         if last == &row {
        //             *c += 1;
        //         } else {
        //             new_trench_by_row.push((1, row));
        //         }
        //     } else {
        //         new_trench_by_row.push((1, row));
        //     }
        // }
    }
    #[allow(clippy::cast_sign_loss)]
    fn grid(&self) -> Vec<Vec<char>> {
        let y_incr = i32::try_from(self.min_y.abs_diff(0)).unwrap();
        let x_incr = i32::try_from(self.min_x.abs_diff(0)).unwrap();
        let mut grid: Vec<Vec<char>> = (0..=self.max_y + y_incr)
            .map(|_| (0..=self.max_x + x_incr).map(|_| '.').collect())
            .collect();
        for points in self.points.windows(2) {
            let (_, first) = points[0];
            let (_, second) = points[1];
            if first.0 == second.0 {
                // vert
                for y in first.1.min(second.1)..=first.1.max(second.1) {
                    grid[(y + y_incr) as usize][(first.0 + x_incr) as usize] = '#';
                }
            } else {
                //horiz
                for x in first.0.min(second.0)..=first.0.max(second.0) {
                    grid[(first.1 + y_incr) as usize][(x + x_incr) as usize] = '#';
                }
            }
        }
        let (_, first) = self.points.last().unwrap();
        let (_, second) = self.points.first().unwrap();
        if first.0 == second.0 {
            // vert
            for y in first.1.min(second.1)..=first.1.max(second.1) {
                grid[(y + y_incr) as usize][(first.0 + x_incr) as usize] = '#';
            }
        } else {
            //horiz
            for x in first.0.min(second.0)..=first.0.max(second.0) {
                grid[(first.1 + y_incr) as usize][(x + x_incr) as usize] = '#';
            }
        }

        grid
    }
    fn filled_grid(&self) -> Vec<Vec<char>> {
        let mut grid = self.grid();
        let point_inside_grid = grid
            .iter()
            .enumerate()
            .find_map(|(idx, row)| {
                let xidx = row.windows(3).enumerate().find_map(|(idx, elems)| {
                    if matches!(elems, &['.', '#', '.']) {
                        Some(idx + 2)
                    } else {
                        None
                    }
                })?;
                Some((xidx, idx))
            })
            .unwrap();
        let mut queue = vec![point_inside_grid];
        while let Some(cur) = queue.pop() {
            for neighbor in neighbors(cur) {
                match grid[neighbor.1][neighbor.0] {
                    '.' => queue.push(neighbor),
                    '#' => (),
                    _ => unreachable!(),
                }
            }
            grid[cur.1][cur.0] = '#';
        }
        grid
    }
}

impl Problem {
    fn path(&self) -> Path {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut cur_point: (i32, i32) = (0, 0);
        let mut path = vec![];
        for trench in &self.trenches {
            path.push((trench.clone(), cur_point));
            cur_point = match trench.dir {
                Direction::Up => {
                    let new_point = (cur_point.0, cur_point.1 - trench.len);
                    min_y = min_y.min(new_point.1);
                    new_point
                }
                Direction::Down => {
                    let new_point = (cur_point.0, cur_point.1 + trench.len);
                    max_y = max_y.max(new_point.1);
                    new_point
                }
                Direction::Left => {
                    let new_point = (cur_point.0 - trench.len, cur_point.1);
                    min_x = min_x.min(new_point.0);
                    new_point
                }
                Direction::Right => {
                    let new_point = (cur_point.0 + trench.len, cur_point.1);
                    max_x = max_x.max(new_point.0);
                    new_point
                }
            };
        }
        Path {
            points: path,
            max_x,
            min_x,
            max_y,
            min_y,
        }
    }
}

fn main() {
    let grid = Problem {
        trenches: aoc_helpers::include_data!(day18)
            .lines()
            .map(Into::into)
            .collect(),
    };
    let path = grid.path();
    println!("{path}");
    let expected: Vec<u64> = path
        .filled_grid()
        .iter()
        .map(|r| r.iter().filter(|&&c| c == '#').count() as u64)
        .collect();
    let expected_diff_per_row: Vec<_> = path
        .grid()
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .collect();
    let result = path.filled_grid_area();
    for (i, ((expected, actual), expected_diff)) in expected
        .iter()
        .zip(result)
        .zip(expected_diff_per_row)
        .enumerate()
    {
        if *expected != actual + expected_diff as u64 {
            println!("Diff @ {i}; expected {expected}, found {actual}");
        }
    }
    // let filled_grid = path.filled_grid();
    // let lines: Vec<String> = filled_grid
    //     .iter()
    //     .map(|line| line.iter().collect())
    //     .collect();
    // println!("{}", lines.join("\n"));
    // println!("Day 18 result: {result}");
}
