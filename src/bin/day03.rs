#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use std::collections::HashMap;

struct Schematic {
    board: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Part {
    row: usize,
    span: (usize, usize), // inclusive
    symbols: Vec<Symbol>,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Symbol {
    symbol: char,
    location: (usize, usize), // row, col
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        Self {
            board: value.lines().map(|l| l.chars().collect()).collect(),
        }
    }
}

impl Schematic {
    fn adjacent_symbols(&self, part: &Part) -> Vec<Symbol> {
        let min_row = if part.row > 0 { part.row - 1 } else { 0 };
        let min_col = if part.span.0 > 0 { part.span.0 - 1 } else { 0 };
        let max_row = (part.row + 1).min(self.board.len() - 1);
        let max_col = (part.span.1 + 1).min(self.board[0].len() - 1);
        let mut symbols = vec![];
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                if !(self.board[row][col] == '.' || self.board[row][col].is_ascii_digit()) {
                    symbols.push(Symbol {
                        symbol: self.board[row][col],
                        location: (row, col),
                    });
                }
            }
        }
        symbols
    }
    fn find_parts(&self) -> Vec<Part> {
        self.board
            .iter()
            .enumerate()
            .flat_map(|(idx, line)| {
                let mut parts = vec![];
                let mut cur_part_start: Option<usize> = None;
                for (cidx, char) in line.iter().enumerate() {
                    if (!char.is_ascii_digit() || cidx == self.board[0].len() - 1)
                        && cur_part_start.is_some()
                    {
                        // Just hit the END of a part span (either no longer digit, or end of a line)
                        let mut part = Part {
                            row: idx,
                            span: (cur_part_start.unwrap(), cidx - 1),
                            symbols: vec![],
                        };
                        let symbols = self.adjacent_symbols(&part);
                        part.symbols = symbols;
                        if !part.symbols.is_empty() {
                            parts.push(part);
                        }
                        cur_part_start = None;
                    } else if char.is_ascii_digit() && cur_part_start.is_none() {
                        // Just hit the START of a part span
                        cur_part_start = Some(cidx);
                    }
                }
                parts
            })
            .collect()
    }
    fn part_number(&self, part: &Part) -> u32 {
        let row = &self.board[part.row];
        row[part.span.0..=part.span.1]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap()
    }
    fn gear_pairs(parts: &[Part]) -> Vec<(&Part, &Part)> {
        let mut shared_symbols = HashMap::new();
        for (symbol, part) in parts
            .iter()
            .flat_map(|p| p.symbols.iter().map(move |s| (s, p)))
        {
            if symbol.symbol != '*' {
                continue;
            }
            shared_symbols
                .entry(symbol)
                .or_insert_with(Vec::new)
                .push(part);
        }
        shared_symbols
            .values()
            .filter_map(|parts| {
                if parts.len() == 2 {
                    Some((parts[0], parts[1]))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn main() {
    let board: Schematic = aoc_2023::include_data!(day3).into();
    let parts = board.find_parts();
    let gear_pairs = Schematic::gear_pairs(&parts);
    println!(
        "Day 3 result: {}",
        gear_pairs
            .iter()
            .map(|(g1, g2)| { board.part_number(g1) * board.part_number(g2) })
            .sum::<u32>()
    );
    // part 1
    // println!("Day 3 result: {}", parts.iter().map(|p| board.part_number(p)).sum::<u32>());
}
