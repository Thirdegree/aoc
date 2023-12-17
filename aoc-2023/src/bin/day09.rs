#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
fn find_next(pattern: Vec<i32>) -> i32 {
    let mut layers = vec![pattern];
    while layers.last().unwrap().iter().any(|&n| n != 0) {
        // Build each layer such that (for example):
        // 0   3   6   9  12  15
        //  turns into
        //   3   3   3   3   3
        //  which then turns into
        //     0   0   0   0
        //  at which point we are done.
        layers.push(
            layers
                .last()
                .unwrap()
                .windows(2)
                .map(|p| p[1] - p[0])
                .collect(),
        );
    }
    // Reverse the layers, because we need to construct the next set of numbers from the "top"
    // (zeros layer) to the bottom (inital layer)
    layers.reverse();
    let mut added_ns = vec![0];
    for l in &layers[1..] {
        // part 2
        added_ns.push(l.first().unwrap() - added_ns.last().unwrap());
        // for part 1
        // added_ns.push(l.last().unwrap() + added_ns.last().unwrap());
    }
    // Last is the base layer, aka the next number in the sequence we want to predict
    *added_ns.last().unwrap()
}

fn main() {
    let result: i32 = aoc_2023::include_data!(day9)
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .map(find_next)
        .sum();
    println!("Day 09 result: {result}");
}
