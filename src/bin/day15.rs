#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[derive(Debug)]
enum Op {
    Focal(usize),
    Remove,
}

#[derive(Debug)]
struct Lense {
    label: String,
    op: Op,
    box_number: usize,
}

#[allow(clippy::fallible_impl_from)]
impl From<&str> for Lense {
    fn from(value: &str) -> Self {
        let (label, op): (String, String) = value.chars().partition(char::is_ascii_alphabetic);
        let mut op = op.chars();
        let op = match op.next() {
            Some('=') => Op::Focal(op.collect::<String>().parse().unwrap()),
            Some('-') => Op::Remove,
            _ => unreachable!(),
        };
        Self {
            label: label.to_string(),
            op,
            box_number: hasher(&label),
        }
    }
}

fn hasher(value: &str) -> usize {
    let mut acc = 0;
    for c in value.chars() {
        acc += c as usize;
        acc *= 17;
        acc %= 256;
    }
    acc
}

fn main() {
    let lenses: Vec<Lense> = aoc_2023::include_data!(day15)
        .trim()
        .split(',')
        .map(Into::into)
        .collect();
    let mut boxes: Vec<Vec<&Lense>> = vec![];
    for _ in 0..256 {
        boxes.push(vec![]);
    }
    for lense in &lenses {
        let cur_box = &mut boxes[lense.box_number];
        let lense_idx = cur_box
            .iter()
            .enumerate()
            .find(|(_, &l)| l.label == lense.label);

        match lense.op {
            Op::Focal(_) => {
                if let Some((idx, _)) = lense_idx {
                    cur_box[idx] = lense;
                } else {
                    cur_box.push(lense);
                }
            }
            Op::Remove => {
                if let Some((idx, _)) = lense_idx {
                    cur_box.remove(idx);
                }
            }
        }
    }
    let result = boxes
        .iter()
        .enumerate()
        .flat_map(|(box_id, lenses)| {
            lenses.iter().enumerate().map(move |(lense_id, lense)| {
                (1 + box_id)
                    * (1 + lense_id)
                    * match lense.op {
                        Op::Focal(n) => n,
                        Op::Remove => unreachable!(),
                    }
            })
        })
        .sum::<usize>();
    println!("Day 15 result: {result}");
}
