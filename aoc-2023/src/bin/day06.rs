#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use anyhow::Context;
fn main() -> anyhow::Result<()> {
    let mut inp = aoc_helpers::include_data!(day6).lines();
    let time: u64 = inp
        .next()
        .context("Bad input")?
        .strip_prefix("Time:")
        .context("Bad input")?
        .replace(' ', "")
        .parse()?;
    let distance: u64 = inp
        .next()
        .context("Bad input")?
        .strip_prefix("Distance:")
        .context("Bad input")?
        .replace(' ', "")
        .parse()?;
    println!(
        "Day 06 result: {}",
        (1..=time).filter(|h| h * (time - h) > distance).count()
    );
    Ok(())
}
#[allow(dead_code)]
fn part1() -> anyhow::Result<()> {
    let mut inp = aoc_helpers::include_data!(day6).lines();
    let time: Vec<u64> = inp
        .next()
        .context("Bad input")?
        .strip_prefix("Time:")
        .context("Bad input")?
        .split_whitespace()
        .map(&str::parse)
        .collect::<anyhow::Result<_, _>>()?;
    let distance: Vec<u64> = inp
        .next()
        .context("Bad input")?
        .strip_prefix("Distance:")
        .context("Bad input")?
        .split_whitespace()
        .map(&str::parse)
        .collect::<anyhow::Result<_, _>>()?;
    let result: usize = time
        .iter()
        .zip(distance)
        .map(|(&time, distance)| (1..=time).filter(|h| h * (time - h) > distance).count())
        .product();
    println!("Day 6 result: {result}");
    Ok(())
}
