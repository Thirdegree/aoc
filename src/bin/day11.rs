#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Point {
    Galaxy,
    Empty(usize, usize),
}

impl From<char> for Point {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Galaxy,
            '.' => Self::Empty(1, 1),
            _ => unreachable!(),
        }
    }
}

struct Universe {
    data: Vec<Vec<Point>>,
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        Self {
            data: value
                .lines()
                .map(|line| line.chars().map(Into::into).collect())
                .collect(),
        }
    }
}

impl Universe {
    fn expand(&mut self, expansion_factor: usize) {
        self.data.iter_mut().for_each(|row| {
            if row.iter().all(|p| matches!(p, Point::Empty(_, _))) {
                for p in row {
                    let Point::Empty(_, x) = p else {
                        unreachable!()
                    };
                    *x *= expansion_factor;
                }
            }
        });
        let nrows = self.data.len();
        let cols_to_expand: Vec<_> = (0..self.data[0].len())
            .filter(|&xidx| {
                (0..nrows).all(|yidx| matches!(self.data[yidx][xidx], Point::Empty(_, _)))
            })
            .collect();
        for &cidx in &cols_to_expand {
            for row in &mut self.data {
                match &mut row[cidx] {
                    Point::Empty(x, _) => *x *= expansion_factor,
                    Point::Galaxy => (),
                }
            }
        }
    }

    fn galexies(&self) -> Vec<(usize, usize)> {
        let mut galexies = vec![];
        let mut y_offset = 0;
        for row in &self.data {
            let mut x_offset = 0;
            for col in row {
                x_offset += match col {
                    Point::Galaxy => {
                        galexies.push((x_offset, y_offset));
                        1
                    }
                    Point::Empty(exp, _) => *exp,
                }
            }
            // Y offsets must always be the same on a given row, and if there is a galexy they must
            // always be 1
            y_offset += match row.first() {
                Some(Point::Empty(_, exp)) => *exp,
                Some(Point::Galaxy) => 1,
                None => unreachable!(),
            };
        }
        galexies
    }

    fn all_distances(&self) -> Vec<u64> {
        self.galexies()
            .iter()
            .combinations(2)
            .map(|perms| {
                let g1 = perms[0];
                let g2 = perms[1];
                (g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1)) as u64
            })
            .collect()
    }

    #[allow(dead_code)]
    fn pprint(&self) {
        for row in &self.data {
            let n = match row.get(0) {
                Some(Point::Empty(_, n)) => *n,
                Some(Point::Galaxy) => 1,
                None => unreachable!(),
            };
            for _ in 0..n {
                // this works because any row with a galexy must be n=1
                for p in row {
                    match p {
                        Point::Galaxy => print!("#"),
                        Point::Empty(n, _) => {
                            print!("{}", (0..*n).map(|_| '.').collect::<String>());
                        }
                    }
                }
                println!();
            }
        }
    }
}

fn main() {
    let mut universe: Universe = aoc_2023::include_data!(day11).into();
    // universe.expand(10); // part 1
    // Do not have to do it this way, only doing it to show I can!
    // 1,000 * 2 * 5 * 20 * 5 == 1,000,000
    universe.expand(1_000);
    universe.expand(2);
    universe.expand(5);
    universe.expand(20);
    universe.expand(5);
    // universe.pprint();
    println!(
        "Day 11 result: {:?}",
        universe.all_distances().iter().sum::<u64>()
    );
}
