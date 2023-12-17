#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use std::str::FromStr;

// ID was used in part 1 but not part 2, but still part of the "Game" struct, but I don't want
// cargo yelling at me.
#[allow(dead_code)]
#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    max_blue: u32,
    max_red: u32,
    max_green: u32,
}

impl FromStr for Game {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_str, grabs) = s.split_once(':').unwrap();
        let id_num = id_str.strip_prefix("Game ").unwrap();
        let mut game = Self {
            id: id_num.parse()?,
            max_red: 0,
            max_blue: 0,
            max_green: 0,
        };
        for grab in grabs.split(';') {
            for marble in grab.split(',') {
                let marble = marble.trim();
                let (count, color) = marble.split_once(' ').unwrap();
                match color {
                    "blue" => game.max_blue = game.max_blue.max(count.parse()?),
                    "green" => game.max_green = game.max_green.max(count.parse()?),
                    "red" => game.max_red = game.max_red.max(count.parse()?),
                    _ => unreachable!(),
                }
            }
        }
        Ok(game)
    }
}

fn main() {
    let result: u32 = aoc_helpers::include_data!(day2)
        .lines()
        .filter_map(|line| line.parse().ok())
        .map(|game: Game| game.max_green * game.max_blue * game.max_red)
        .sum();
    println!("Day 02 result: {result}");
}

#[allow(dead_code)]
fn part1() {
    let result = aoc_helpers::include_data!(day2)
        .lines()
        .filter_map(|line| line.parse().ok())
        .filter_map(|game: Game| {
            if game.max_green <= 13 && game.max_blue <= 14 && game.max_red <= 12 {
                Some(game.id)
            } else {
                None
            }
        })
        .sum::<u32>();
    println!("Day 2 result: {result}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_line() {
        assert_eq!(
            Ok(Game {
                id: 5,
                max_red: 1,
                max_blue: 2,
                max_green: 3
            }),
            "Game 5: 1 red; 2 blue, 3 green".parse()
        );
    }
    #[test]
    fn test_parse_game_with_missing_counts() {
        assert_eq!(
            Ok(Game {
                id: 11,
                max_red: 0,
                max_blue: 0,
                max_green: 200
            }),
            "Game 11: 200 green".parse()
        );
    }
}
