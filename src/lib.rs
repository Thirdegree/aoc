#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct TwoDArray<T> {
    pub elems: Vec<Vec<T>>,
}

impl<T> Index<(usize, usize)> for TwoDArray<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.elems[index.1][index.0]
    }
}
impl<T> Index<usize> for TwoDArray<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl<T> IndexMut<(usize, usize)> for TwoDArray<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.elems[index.1][index.0]
    }
}

impl<T> FromIterator<Vec<T>> for TwoDArray<T> {
    fn from_iter<A: IntoIterator<Item = Vec<T>>>(iter: A) -> Self {
        Self {
            elems: iter.into_iter().collect(),
        }
    }
}

impl<T> TwoDArray<T> {
    #[must_use]
    pub fn y_len(&self) -> usize {
        self.elems.len()
    }
    #[must_use]
    pub fn x_len(&self) -> usize {
        self.elems.len()
    }
    #[must_use]
    pub fn is_within_bounds(&self, pos: (usize, usize)) -> bool {
        // no need to check > 0 because usize
        pos.0 < self.x_len() && pos.1 < self.y_len()
    }
    pub fn rows(&self) -> std::slice::Iter<Vec<T>> {
        self.elems.iter()
    }
    pub fn rows_mut(&mut self) -> std::slice::IterMut<Vec<T>> {
        self.elems.iter_mut()
    }
    pub fn elems(&self) -> std::iter::Flatten<std::slice::Iter<Vec<T>>> {
        self.elems.iter().flatten()
    }
}

#[macro_export]
macro_rules! include_data {
    ($day:expr) => {{
        // need the type spec or rust_analyzer is not happy with me
        // https://users.rust-lang.org/t/macro-return-type/58596/6
        let out: &str = include_str!(concat!("../", stringify!($day), "/input.txt"));
        out
    }};
    ($day:expr, sample) => {{
        let out: &str = include_str!(concat!("../", stringify!($day), "/sample.txt"));
        out
    }};
}

pub mod math {
    #[must_use]
    pub fn lcm(first: u64, second: u64) -> u64 {
        (first * second) / gcd(first, second)
    }

    fn gcd(first: u64, second: u64) -> u64 {
        let mut max = first;
        let mut min = second;
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }
        loop {
            let res = max % min;
            if res == 0 {
                return min;
            }
            max = min;
            min = res;
        }
    }
}
