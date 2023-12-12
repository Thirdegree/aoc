#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;
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

// interesting here is the next Spring | Unknown after (or at) the given idx
// so e.g. .??....???? -> vec![1, 1, 2, 7, 7, 7, 7, 7, 7, 7, 8, 9, 10] (all somes)
fn find_number_possible_group_locations(
    elems: &[Space],
    groups: &[SpringGroup],
    next_interesting_elem_for_idx: &[Option<usize>],
    cur_idx: usize,
) -> usize {
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
    let relevent_elems = &elems[cur_idx..];
    let cur_group = &groups[0];
    let mut interesting = HashMap::new();
    let mut tot = 0;
    let next_spring = relevent_elems
        .iter()
        .enumerate()
        .find_map(|(idx, elem)| match elem {
            Space::Spring => Some(idx),
            _ => None,
        })
        .unwrap_or(relevent_elems.len() - 1);
    // dbg!(cur_group, &elems[cur_idx..]);
    for n in 0..=next_spring {
        let interesting_scan_cur = n + cur_group.length + 1;
        if can_place_group_at(relevent_elems, cur_group, n) {
            if groups.len() == 1 {
                if interesting_scan_cur > relevent_elems.len()
                    || !relevent_elems[interesting_scan_cur..]
                        .iter()
                        .any(|s| matches!(s, Space::Spring))
                {
                    // println!("Got here - {next_spring}");
                    tot += 1;
                }

                continue;
            }
            if interesting_scan_cur >= relevent_elems.len() {
                break;
            }
            // println!("{next_interesting_elem_for_idx:?}; {interesting_scan_cur}; {cur_idx}");
            if let Some(entry) = next_interesting_elem_for_idx[interesting_scan_cur + cur_idx] {
                *interesting.entry(entry).or_insert(0) += 1;
            }
        }
    }
    for (found, occurances) in &interesting {
        let v_interesting = find_number_possible_group_locations(
            elems,
            &groups[1..],
            next_interesting_elem_for_idx,
            *found,
        );
        tot += v_interesting * occurances;
    }
    // dbg!(&interesting);
    // dbg!(&elems);
    // dbg!(cur_group, tot);
    tot
}

impl Row {
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
        let next_interesting_elem_for_idx = (0..elems.len())
            .map(|n| {
                elems[n..]
                    .iter()
                    .enumerate()
                    .find(|e| matches!(e.1, Space::Spring | Space::Unknown))
                    .map(|found| found.0 + n)
            })
            .collect::<Vec<_>>();
        find_number_possible_group_locations(&elems, &groups, &next_interesting_elem_for_idx, 0)
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
    for (n, row) in rows.into_iter().enumerate(){
        let thistx = tx.clone();
        pool.execute(move || {
            // pprint_rowvec(&row.elems);
            let c = row.number_possible_springgroup_locations();
            println!("Finished {n}; {c} possible solutions");
            thistx.send(c).unwrap();
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
