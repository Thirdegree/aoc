#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use anyhow::Context;
fn main() -> anyhow::Result<()> {
    let mut inp = aoc_2023::include_data!(day6).lines();
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
        "Day 6 result: {}",
        (1..=time).filter(|h| h * (time - h) > distance).count()
    );
    Ok(())
}
