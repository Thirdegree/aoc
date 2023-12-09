#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
fn main() {
    let mut inp = aoc_2023::include_data!(day6).lines();
    let time: u64 = inp
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance: u64 = inp
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let mut this_race_winning = 0;
    for h in 1..=time {
        if h * (time - h) > distance {
            this_race_winning += 1;
        }
    }
    println!("Day 6 result: {this_race_winning}");
}
