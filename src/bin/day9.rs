fn find_next(pattern: Vec<i32>) -> i32 {
    let mut layers = vec![pattern];
    while layers.last().unwrap().iter().any(|&n| n != 0) {
        let mut next_layer = vec![];
        let pairs: Vec<&[i32]> = layers.last().unwrap().windows(2).collect();
        for p in pairs {
            next_layer.push(p[1] - p[0]);
        }
        layers.push(next_layer);
    }
    layers.reverse();
    let mut added_ns = vec![0];
    for l in &layers[1..] {
        // part 2
        added_ns.push(l.first().unwrap() - added_ns.last().unwrap());
        // for part 1
        // added_ns.push(l.last().unwrap() + added_ns.last().unwrap());
    }
    *added_ns.last().unwrap()
}

fn main() {
    let patterns: Vec<Vec<i32>> = aoc_2023::include_data!(day9)
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    let result = patterns.into_iter().map(find_next).sum::<i32>();
    println!("Day 9 result: {result}");
}
