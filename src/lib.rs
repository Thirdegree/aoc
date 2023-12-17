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
    use priority_queue::PriorityQueue;
    use std::collections::HashMap;

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
    pub fn dijkstra<T, A, G, W, Fe>(start: &T, edge_gen: Fe, is_goal: G, weight: W) -> (Vec<T>, i32)
    where
        T: Eq + std::hash::Hash + Clone + std::fmt::Debug,
        W: for<'a> Fn(&'a A) -> Option<i32>,
        G: Fn(&T) -> bool,
        Fe: for<'a> Fn(&'a W, &'a T) -> Vec<(i32, T)>,
    {
        astar(start, edge_gen, is_goal, weight, |_| 0)
    }
    pub fn astar<T, A, G, W, W2, Fe>(
        start: &T,
        edge_gen: Fe,
        is_goal: G,
        weight: W,
        heuristic: W2,
    ) -> (Vec<T>, i32)
    where
        T: Eq + std::hash::Hash + Clone + std::fmt::Debug,
        W: for<'a> Fn(&'a A) -> Option<i32>,
        W2: Fn(&T) -> i32,
        G: Fn(&T) -> bool,
        Fe: for<'a> Fn(&'a W, &'a T) -> Vec<(i32, T)>,
    {
        let mut to_visit = PriorityQueue::new();
        to_visit.push(start.clone(), 0);
        let mut camefrom: HashMap<T, T> = HashMap::new();
        let mut g_score: HashMap<T, i32> = HashMap::new();
        g_score.insert(start.clone(), 0);

        while let Some((current, fscore)) = to_visit.pop() {
            if is_goal(&current) {
                let mut current = current;
                let mut path = vec![current.clone()];
                while let Some(came) = camefrom.get(&current) {
                    let came = came.clone();
                    path.push(came.clone());
                    current = came;
                }
                return (path.into_iter().rev().collect(), -fscore);
            }
            to_visit.remove(&current);
            for (weight, neighbor) in edge_gen(&weight, &current) {
                let tentative_gscore = g_score[&current] + weight;
                if let Some(&old_score) = g_score.get(&neighbor) {
                    if tentative_gscore < old_score {
                        camefrom.insert(neighbor.clone(), current.clone());
                        let h = heuristic(&neighbor);
                        to_visit.push(neighbor.clone(), -(tentative_gscore + h));
                        g_score.insert(neighbor.clone(), tentative_gscore);
                    }
                } else {
                    let h = heuristic(&neighbor);
                    g_score.insert(neighbor.clone(), tentative_gscore);
                    camefrom.insert(neighbor.clone(), current.clone());
                    to_visit.push(neighbor.clone(), -(tentative_gscore + h));
                }
            }
        }
        unreachable!()
    }
}
