#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

const fn fuel_cost(mass: usize) -> usize {
    (mass / 3).saturating_sub(2)
}

fn main() {
    println!(
        "Cost for 12: {}",
        aoc_helpers::include_data!(day01)
            .lines()
            .map(|l| {
                let mut mass = l.parse().unwrap();
                let mut tot_fuel = 0;
                loop {
                    mass = fuel_cost(mass).max(0);
                    if mass == 0 {
                        break;
                    }
                    tot_fuel += mass;
                }
                // println!("{} {}", l, tot_fuel);
                tot_fuel
            })
            .sum::<usize>()
    );
}
