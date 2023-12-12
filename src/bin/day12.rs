#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::{HashMap, VecDeque};
use std::sync::mpsc;

use threadpool::ThreadPool;

use anyhow::anyhow;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct SpringGroup {
    length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Space {
    Spring,
    Empty,
    Unknown,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '?' => Self::Unknown,
            '.' => Self::Empty,
            '#' => Self::Spring,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Row {
    elems: Vec<Space>,
    groups: Vec<SpringGroup>,
}

impl TryFrom<&str> for Row {
    type Error = anyhow::Error;

    // part1
    // fn try_from(value: &str) -> Result<Self, Self::Error> {
    //     let (row, groups) = value.split_once(' ').ok_or_else(|| anyhow!("Bad input"))?;
    //     let row: Vec<Space> = row.chars().map(Into::into).collect();
    //     let groups: Vec<_> = groups
    //         .split(',')
    //         .map(|n| SpringGroup {
    //             length: n.parse().unwrap(),
    //         })
    //         .collect();
    //     Ok(Self { elems: row, groups })
    // }
    // part 2
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (row, groups) = value.split_once(' ').ok_or_else(|| anyhow!("Bad input"))?;
        let row: Vec<Space> = row.chars().map(Into::into).collect();
        let mut row = vec![row; 5];
        let mut joined_rows = vec![];
        for r in &mut row {
            joined_rows.append(r);
            joined_rows.push(Space::Unknown);
        }
        joined_rows.pop();
        let groups: Vec<_> = groups
            .split(',')
            .map(|n| SpringGroup {
                length: n.parse().unwrap(),
            })
            .collect();
        let groups_len = groups.len();
        Ok(Self {
            elems: joined_rows,
            groups: groups.into_iter().cycle().take(groups_len * 5).collect(),
        })
    }
}

fn can_place_group_at(elems: &[Space], group: &SpringGroup, idx: usize) -> bool {
    if idx + group.length > elems.len() {
        // springgroup would overrun the end of elems
        return false;
    }
    if idx > 0 && matches!(elems.get(idx - 1), Some(Space::Spring) | None) {
        // elem before springgroup is a spring, so springgroup can't be here
        return false;
    }
    if idx + group.length < elems.len()
        && matches!(elems.get(idx + group.length), Some(Space::Spring) | None)
    {
        // elem after springgroup is a spring, so sppringgroup can't be here
        return false;
    }
    if elems[idx..idx + group.length]
        .iter()
        .any(|s| matches!(s, Space::Empty))
    {
        return false;
    }
    true
}

fn is_board_valid_so_far(
    elems: &[Space],
    set_groups_pos: &[usize],
    set_groups: &[SpringGroup],
    cur_idx: usize,
) -> bool {
    elems[..cur_idx]
        .iter()
        .enumerate()
        .filter_map(|(idx, s)| match s {
            Space::Spring => Some(idx),
            _ => None,
        })
        .all(|idx| {
            set_groups
                .iter()
                .zip(set_groups_pos)
                .any(|(g, &gidx)| (gidx..gidx + g.length).contains(&idx))
        })
}

fn board_might_be_valid(elems: &[Space], groups: &[SpringGroup]) -> bool {
    elems
        .iter()
        .filter(|s| matches!(s, Space::Spring | Space::Unknown))
        .count()
        >= groups.iter().map(|g| g.length).sum()
}

fn find_number_possible_group_locations(elems: &[Space], groups: &[SpringGroup]) -> usize {
    // (placed groups (no idx), next_interesting_index) => memoixed remaining index (offset +
    // interesing)
    // e.g. .??....???? 1, 1, 1
    // ((1,), 7) => [(0, 3), (1, 3), (0, 2)]  which works if 1 goes in 1 or 2

    // Such that key.0.len() + value.len() == groups.len()
    if groups.is_empty() {
        // Technically can never happen, the case where group.len() == 1 is implicitly the base
        // case below.
        return 0;
    }
    // if !board_might_be_valid(elems, groups) {
    //     return 0;
    // }
    let cur_group = &groups[0];
    let mut interesting = HashMap::new();
    let mut tot = 0;
    let next_spring = elems
        .iter()
        .enumerate()
        .find_map(|(idx, elem)| match elem {
            Space::Spring => Some(idx),
            _ => None,
        })
        .unwrap_or(elems.len() - 1);
    for n in 0..=next_spring {
        let interesting_scan_cur = n + cur_group.length + 1;
        if can_place_group_at(elems, cur_group, n) {
            if groups.len() == 1 {
                if interesting_scan_cur > elems.len()
                    || !elems[interesting_scan_cur..]
                        .iter()
                        .any(|s| matches!(s, Space::Spring))
                {
                    tot += 1;
                }

                continue;
            }
            if interesting_scan_cur >= elems.len() {
                break;
            }
            if let Some(entry) = elems[interesting_scan_cur..]
                .iter()
                .enumerate()
                .find(|e| matches!(e.1, Space::Unknown | Space::Spring))
            {
                *interesting
                    .entry(entry.0 + interesting_scan_cur)
                    .or_insert(0) += 1;
            }
        }
    }
    for (found, occurances) in &interesting {
        let v_interesting = find_number_possible_group_locations(&elems[*found..], &groups[1..]);
        tot += v_interesting * occurances;
    }
    // dbg!(&interesting);
    // dbg!(&elems);
    // dbg!(cur_group, tot);
    tot
}

fn find_possible_boards(elems: &[Space], groups: &[SpringGroup]) -> Vec<Vec<Space>> {
    let elems = elems.to_vec();
    let mut valid_boards: VecDeque<(usize, Vec<usize>, &[SpringGroup])> = VecDeque::new();
    valid_boards.push_back((0, vec![], groups));
    let mut completed_boards = vec![];
    let not_empty_idx_for_group: HashMap<&SpringGroup, Vec<usize>> = groups
        .iter()
        .map(|g| {
            (
                g,
                elems
                    .windows(g.length)
                    .enumerate()
                    .filter(|(_, spaces)| {
                        spaces
                            .iter()
                            .all(|s| matches!(s, Space::Unknown | Space::Spring))
                    })
                    .map(|(sidx, _)| sidx)
                    .collect(),
            )
        })
        .collect();

    let mut i = 0;
    let board_len = elems.len();
    while let Some((s_idx, cur_board, remaining_groups)) = valid_boards.pop_front() {
        i += 1;
        if i % 1_000_000 == 0 {
            println!("{i}: n_valid_boards_in_queue: {}", valid_boards.len());
        }
        for &possible_idx in not_empty_idx_for_group.get(&remaining_groups[0]).unwrap() {
            if possible_idx < s_idx {
                continue;
            }
            if can_place_group_at(&elems, &remaining_groups[0], possible_idx) {
                let mut new_board = cur_board.clone();
                new_board.push(possible_idx);

                let new_sidx = possible_idx + remaining_groups[0].length + 1;
                let new_remaining_groups = &remaining_groups[1..];

                if new_remaining_groups.is_empty() {
                    completed_boards.push(new_board);
                } else if new_sidx >= board_len {
                    continue;
                } else if is_board_valid_so_far(
                    &elems,
                    &new_board,
                    &groups[..new_board.len()],
                    new_sidx,
                ) {
                    valid_boards.push_back((new_sidx, new_board, new_remaining_groups));
                }
            }
        }
    }
    completed_boards
        .into_iter()
        .map(|b| {
            let mut new_board = elems.clone();
            for (idx, g) in b.iter().zip(groups) {
                for point in &mut new_board[*idx..*idx + g.length] {
                    *point = Space::Spring;
                }
            }
            new_board
        })
        .collect()
}

impl Row {
    fn possible_springgroup_starts(&self) -> Vec<Vec<Space>> {
        find_possible_boards(&self.elems, &self.groups)
    }
    fn number_possible_springgroup_locations(&self) -> usize {
        let midpoint = self.groups.len() / 2;
        let (elems, groups) = if self.groups[..midpoint]
            .iter()
            .map(|g| g.length)
            .sum::<usize>()
            < self.groups[midpoint..]
                .iter()
                .map(|g| g.length)
                .sum::<usize>()
        {
            (
                self.elems.clone().into_iter().rev().collect(),
                self.groups.clone().into_iter().rev().collect(),
            )
        } else {
            (self.elems.clone(), self.groups.clone())
        };
        find_number_possible_group_locations(&elems, &groups)
    }
}

#[allow(dead_code)]
fn pprint_rowvec(elems: &[Space]) {
    let out = elems
        .iter()
        .map(|s| match s {
            Space::Unknown => '?',
            Space::Empty => '.',
            Space::Spring => '#',
        })
        .collect::<String>();
    println!("{out}");
}

fn main() -> anyhow::Result<()> {
    let rows: Vec<Row> = aoc_2023::include_data!(day12)
        .lines()
        .map(TryInto::try_into)
        .collect::<anyhow::Result<_, _>>()?;
    // for row in &rows[..1] {
    //     pprint_rowvec(&row.elems);
    //     println!("{:?}", row.groups);
    //     for solution in row.possible_springgroup_starts() {
    //         pprint_rowvec(&solution);
    //     }
    //     println!();
    // }
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(100);
    for (n, row) in rows.into_iter().enumerate() {
        let thistx = tx.clone();
        pool.execute(move || {
            let c = row.number_possible_springgroup_locations();
            println!("Finished {n}; {c} possible solutions");
            thistx
                .send(c)
                .unwrap();
        });
    }
    pool.join();
    drop(tx);
    println!("Day 12 result: {}", rx.iter().sum::<usize>());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    fn spaces(v: &str) -> Vec<Space> {
        v.chars().map(Into::into).collect()
    }
    #[test]
    fn test_set_springgroup_at_full_simple() {
        let elems = vec![Space::Unknown];
        assert!(can_place_group_at(&elems, &SpringGroup { length: 1 }, 0));
    }
    #[test]
    fn test_set_springgroup_at_full_longer() {
        let elems = vec![Space::Unknown; 3];
        assert!(can_place_group_at(&elems, &SpringGroup { length: 3 }, 0));
    }
    #[test]
    fn test_set_springgroup_middle() {
        let elems = vec![Space::Empty, Space::Unknown, Space::Unknown];
        assert!(can_place_group_at(&elems, &SpringGroup { length: 2 }, 1));
    }
    #[test]
    fn test_set_springgroup_example_1_step_1() {
        let elems: Vec<Space> = spaces("???.###");
        assert!(can_place_group_at(&elems, &SpringGroup { length: 1 }, 0));
    }
    #[test]
    fn test_set_springgroup_example_1_step_2() {
        let elems: Vec<Space> = spaces("#.?.###");
        assert!(can_place_group_at(&elems, &SpringGroup { length: 1 }, 2));
    }
}
