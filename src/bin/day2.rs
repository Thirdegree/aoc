// ID was used in part 1 but not part 2, but still part of the "Game" struct, but I don't want
// cargo yelling at me.
#[allow(dead_code)]
struct Game {
    id: usize,
    max_blue: usize,
    max_red: usize,
    max_green: usize,
}

fn parse_line(line: &str) -> Game {
    let (id_str, grabs) = line.split_once(':').unwrap();
    let id_num = id_str.strip_prefix("Game ").unwrap();
    let mut game = Game {
        id: id_num.parse().unwrap(),
        max_red: 0,
        max_blue: 0,
        max_green: 0,
    };
    for grab in grabs.split(';') {
        for marble in grab.split(',') {
            let marble = marble.trim();
            let (count, color) = marble.split_once(' ').unwrap();
            match color {
                "blue" => game.max_blue = game.max_blue.max(count.parse().unwrap()),
                "green" => game.max_green = game.max_green.max(count.parse().unwrap()),
                "red" => game.max_red = game.max_red.max(count.parse().unwrap()),
                _ => unreachable!(),
            }
        }
    }
    game
}

fn main() {
    let result: usize = include_str!("../day2/input.txt")
        .lines()
        .map(parse_line)
        .map(|game| game.max_green * game.max_blue * game.max_red)
        .sum();
    println!("Day 2 result: {result}")
}
