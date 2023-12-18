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
    use hashbrown::HashMap;
    use priority_queue::PriorityQueue;

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
    struct SearchState<T>
    where
        T: std::hash::Hash + Eq,
    {
        to_visit: PriorityQueue<T, i32>,
        came_from: HashMap<T, T>,
        g_score: HashMap<T, i32>,
    }
    impl<T: std::hash::Hash + Eq> SearchState<T> {
        fn new() -> Self {
            Self {
                to_visit: PriorityQueue::new(),
                came_from: HashMap::new(),
                g_score: HashMap::new(),
            }
        }
    }

    pub fn dijkstra<T, A, G, W, Fe>(start: &T, edge_gen: Fe, is_goal: G, weight: W) -> (Vec<T>, i32)
    where
        T: Eq + std::hash::Hash + Clone + std::fmt::Debug,
        W: Fn(&A) -> Option<i32>,
        G: Fn(&T) -> bool,
        Fe: Fn(&W, &T) -> Vec<(i32, T)>,
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
        // Some of this could be more flexible (e.g. weight return type)
        // But it worked with the absolute nonsense T I used in day 17
        // So I'm fairly confident in it in general
        T: Eq + std::hash::Hash + Clone + std::fmt::Debug,
        W: Fn(&A) -> Option<i32>,
        W2: Fn(&T) -> i32,
        G: Fn(&T) -> bool,
        Fe: Fn(&W, &T) -> Vec<(i32, T)>,
    {
        let mut search_state = SearchState::new();
        search_state.to_visit.push(start.clone(), 0);
        search_state.g_score.insert(start.clone(), 0);

        while let Some((current, fscore)) = search_state.to_visit.pop() {
            if is_goal(&current) {
                let mut current = current;
                let mut path = vec![current.clone()];
                while let Some(came) = search_state.came_from.get(&current) {
                    let came = came.clone();
                    path.push(came.clone());
                    current = came;
                }
                return (path.into_iter().rev().collect(), -fscore);
            }
            search_state.to_visit.remove(&current);
            for (weight, neighbor) in edge_gen(&weight, &current) {
                let tentative_gscore = search_state.g_score[&current] + weight;
                if let Some(&old_score) = search_state.g_score.get(&neighbor) {
                    if tentative_gscore < old_score {
                        search_state
                            .came_from
                            .insert(neighbor.clone(), current.clone());
                        let h = heuristic(&neighbor);
                        search_state
                            .to_visit
                            .push(neighbor.clone(), -(tentative_gscore + h));
                        search_state
                            .g_score
                            .insert(neighbor.clone(), tentative_gscore);
                    }
                } else {
                    let h = heuristic(&neighbor);
                    search_state
                        .g_score
                        .insert(neighbor.clone(), tentative_gscore);
                    search_state
                        .came_from
                        .insert(neighbor.clone(), current.clone());
                    search_state
                        .to_visit
                        .push(neighbor.clone(), -(tentative_gscore + h));
                }
            }
        }
        unreachable!()
    }
}
