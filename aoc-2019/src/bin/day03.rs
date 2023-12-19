#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::{iproduct, Itertools};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn manhattan_distance(&self, other: &Self) -> usize {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y))
            .try_into()
            .unwrap()
    }
}
#[derive(Debug)]
struct Segment {
    start: Pos,
    end: Pos,
}

impl Segment {
    fn intersects(&self, other: &Self) -> Option<Pos> {
        if (self.start.x == self.end.x && other.start.x == other.end.x)
            || (self.start.y == self.end.y && other.start.y == other.end.y)
        {
            // going the same direction, can't intersect
            return None;
        }
        let (horiz, vert) = if self.start.x == self.end.x {
            // own x's are the same, so it's vertical
            (other, self)
        } else {
            (self, other)
        };
        if horiz.start.y < vert.start.y.min(vert.end.y) {
            // horiz is lower than entire segment, can't intersect
            return None;
        }
        if horiz.start.y > vert.start.y.max(vert.end.y) {
            // horiz is higher than entire segment, can't intersect
            return None;
        }
        if vert.start.x < horiz.start.x.min(horiz.end.x) {
            // vert is to the left of entire segment, can't intersect
            return None;
        }
        if vert.start.x > horiz.start.x.max(horiz.end.x) {
            // vert is to the right of entire segment, can't intersect
            return None;
        }
        if vert.start.x == 0 && horiz.start.y == 0 {
            // origin, doesn't count
            return None;
        }
        Some(Pos {
            x: vert.start.x,
            y: horiz.start.y,
        })
    }
}

#[derive(Debug)]
struct Wire {
    segments: Vec<Segment>,
}

#[allow(clippy::fallible_impl_from)]
impl From<&str> for Wire {
    fn from(value: &str) -> Self {
        let mut pos = Pos { x: 0, y: 0 };
        let mut segments = vec![];
        for segment in value.split(',') {
            let mut segment = segment.chars();
            let dir = segment.next().unwrap();
            let count: i32 = segment.collect::<String>().parse().unwrap();
            let new_pos = match dir {
                'U' => Pos {
                    x: pos.x,
                    y: pos.y - count,
                },
                'D' => Pos {
                    x: pos.x,
                    y: pos.y + count,
                },
                'L' => Pos {
                    x: pos.x - count,
                    y: pos.y,
                },
                'R' => Pos {
                    x: pos.x + count,
                    y: pos.y,
                },
                _ => unreachable!(),
            };
            segments.push(Segment {
                start: pos,
                end: new_pos.clone(),
            });
            pos = new_pos;
        }
        Self { segments }
    }
}

fn main() {
    let wires: Vec<Wire> = aoc_helpers::include_data!(day03)
        .lines()
        .map(Into::into)
        .collect();
    let wire_1 = &wires[0];
    let wire_2 = &wires[1];
    let result = iproduct!(&wire_1.segments, &wire_2.segments)
        .filter_map(|(s1, s2)| {
            s1.intersects(s2)
                .map(|point| point.manhattan_distance(&Pos { x: 0, y: 0 }))
        })
        .min()
        .unwrap();
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_segment_intersects() {
        let vert = Segment {
            start: Pos { x: 5, y: 5 },
            end: Pos { x: 5, y: 20 },
        };
        let horiz = Segment {
            start: Pos { x: 3, y: 10 },
            end: Pos { x: 10, y: 10 },
        };
        assert_eq!(vert.intersects(&horiz), Some(Pos { x: 5, y: 10 }));
    }
    #[test]
    fn test_segment_intersects_too_high() {
        let vert = Segment {
            start: Pos { x: 5, y: 5 },
            end: Pos { x: 5, y: 20 },
        };
        let horiz = Segment {
            start: Pos { x: 7, y: 10 },
            end: Pos { x: 10, y: 10 },
        };
        assert_eq!(vert.intersects(&horiz), None);
    }
}
