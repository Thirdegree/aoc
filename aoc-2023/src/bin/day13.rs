#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
enum Point {
    Ash,
    Rock,
}

impl From<char> for Point {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Rock,
            '.' => Self::Ash,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Pattern {
    elems: Vec<Vec<Point>>,
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        Self {
            elems: value
                .lines()
                .map(|line| line.chars().map(Into::into).collect())
                .collect(),
        }
    }
}

// https://stackoverflow.com/a/64499219
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(IntoIterator::into_iter).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn find_horizontal_mirrored_line(elems: &Vec<Vec<Point>>) -> Option<usize> {
    let len = elems.len();
    for line in 0..elems.len() {
        let num_mirrored_lines = (len - (line + 1)).min(line + 1);
        if num_mirrored_lines == 0 {
            continue;
        }
        let mut num_smudges = 0;
        for n in 0..num_mirrored_lines {
            let top = &elems[line - n];
            let bottom = &elems[line + n + 1];
            num_smudges += top
                .iter()
                .zip(bottom)
                .filter(|(telem, belem)| telem != belem)
                .count();
        }
        // if num_smudges == 0 { // part 1
        if num_smudges == 1 {
            // part 2
            return Some(line);
        }
    }
    None
}

impl Pattern {
    fn find_vertical_mirrored_line(&self) -> Option<usize> {
        let elems = transpose(self.elems.clone());
        find_horizontal_mirrored_line(&elems)
    }
    fn find_horizontal_mirrored_line(&self) -> Option<usize> {
        find_horizontal_mirrored_line(&self.elems)
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self
            .elems
            .iter()
            .map(|line| {
                line.iter()
                    .map(|point| match point {
                        Point::Rock => '#',
                        Point::Ash => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        f.write_str(lines.join("\n").as_str())
    }
}

fn main() {
    let patterns: Vec<Pattern> = aoc_helpers::include_data!(day13)
        .split("\n\n")
        .map(Into::into)
        .collect();
    let mut tot = 0;
    for pattern in patterns {
        if let Some(r) = pattern.find_horizontal_mirrored_line() {
            tot += 100 * (r + 1);
        } else {
            let r = pattern.find_vertical_mirrored_line().unwrap();
            tot += r + 1;
        }
    }
    println!("Day 13 result: {tot}");
}
