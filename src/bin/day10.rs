#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[derive(PartialEq, Debug)]
enum Directions {
    North,
    South,
    East,
    West,
}

impl Directions {
    fn directions_for_tile(tile: char) -> Vec<Self> {
        match tile {
            '|' => vec![Self::North, Self::South],
            '-' => vec![Self::East, Self::West],
            'J' => vec![Self::North, Self::West],
            'L' => vec![Self::North, Self::East],
            '7' => vec![Self::South, Self::West],
            'F' => vec![Self::South, Self::East],
            'S' => vec![Self::North, Self::South, Self::East, Self::West],
            _ => vec![],
        }
    }
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<char>>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self {
            tiles: value.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

impl Grid {
    fn cleaned_board(&self) -> Vec<Vec<char>> {
        let loop_coords = self.loop_coordinates();
        self.tiles
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, &t)| {
                        if loop_coords.contains(&(x, y)) {
                            t
                        } else {
                            '.'
                        }
                    })
                    .collect()
            })
            .collect()
    }

    #[allow(clippy::similar_names)]
    fn enclosed_coords(&self) -> Vec<(usize, usize)> {
        let loop_coords = self.loop_coordinates();
        let mut enclosed = vec![];
        let clean_board: Vec<Vec<_>> = self.cleaned_board();
        let x_max = self.tiles.len();
        let y_max = self.tiles[0].len();
        for (x, y) in (0..x_max).flat_map(|x| (0..y_max).map(move |y| (x, y))) {
            if loop_coords.contains(&(x, y)) {
                continue;
            }
            let xs_before = 0..x;
            let mut horiz_symbols_before = vec![];
            let xs_after = x + 1..x_max;
            let mut horiz_symbols_after = vec![];
            let ys_before = 0..y;
            let mut vert_symbols_before = vec![];
            let ys_after = y + 1..y_max;
            let mut vert_symbols_after = vec![];
            // calculate if we are enclosed horiz
            for (range, target) in [
                (ys_before, &mut horiz_symbols_before),
                (ys_after, &mut horiz_symbols_after),
            ] {
                for cur_y in range {
                    let tile = clean_board[x][cur_y];
                    match (tile, target.last()) {
                        ('-' | '.', _) => (),
                        ('|', Some('|'))
                        | ('L', Some('J'))
                        | ('J', Some('L'))
                        | ('7', Some('F'))
                        | ('F', Some('7')) => {
                            target.pop();
                        }
                        ('L', Some('7'))
                        | ('F', Some('J'))
                        | ('7', Some('L'))
                        | ('J', Some('F')) => {
                            target.pop();
                            target.push('|');
                        }
                        (t, _) => target.push(t),
                    }
                }
            }
            for (range, target) in [
                (xs_before, &mut vert_symbols_before),
                (xs_after, &mut vert_symbols_after),
            ] {
                for cur_x in range {
                    let tile = clean_board[cur_x][y];
                    match (tile, target.last()) {
                        ('|' | '.', _) => (),
                        ('-', Some('-'))
                        | ('7', Some('J'))
                        | ('J', Some('7'))
                        | ('L', Some('F'))
                        | ('F', Some('L')) => {
                            target.pop();
                        }
                        ('L', Some('7'))
                        | ('F', Some('J'))
                        | ('7', Some('L'))
                        | ('J', Some('F')) => {
                            target.pop();
                            target.push('-');
                        }
                        (t, _) => target.push(t),
                    }
                }
            }
            if [
                horiz_symbols_before,
                horiz_symbols_after,
                vert_symbols_before,
                vert_symbols_after,
            ]
            .iter()
            .all(|s| s.len() % 2 != 0)
            {
                enclosed.push((x, y));
            }
        }
        enclosed
    }

    fn loop_coordinates(&self) -> Vec<(usize, usize)> {
        let mut cur_coords =
            self.tiles
                .iter()
                .enumerate()
                .find_map(|(idx, row)| {
                    Some((
                        idx,
                        row.iter().enumerate().find_map(|(ridx, &elem)| {
                            if elem == 'S' {
                                Some(ridx)
                            } else {
                                None
                            }
                        })?,
                    ))
                })
                .unwrap();
        let mut found_coords = vec![];
        loop {
            let valid_neighbors: Vec<(usize, usize)> =
                self.valid_neighbors(cur_coords.0, cur_coords.1);
            assert_eq!(
                valid_neighbors.len(),
                2,
                "There can only be exactly 2 neighbors in a valid loop"
            );
            found_coords.push(cur_coords);
            if let Some(&new_coords) = valid_neighbors.iter().find(|c| !found_coords.contains(c)) {
                cur_coords = new_coords;
            } else {
                break;
            }
        }
        found_coords
    }

    fn tile_at(&self, x: usize, y: usize) -> Option<&char> {
        self.tiles.get(x)?.get(y)
    }

    fn valid_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        Directions::directions_for_tile(self.tiles[x][y])
            .iter()
            .filter_map(|direction| match direction {
                Directions::North => {
                    if x > 0
                        && Directions::directions_for_tile(*self.tile_at(x - 1, y)?)
                            .contains(&Directions::South)
                    {
                        Some((x - 1, y))
                    } else {
                        None
                    }
                }
                Directions::South => {
                    if Directions::directions_for_tile(*self.tile_at(x + 1, y)?)
                        .contains(&Directions::North)
                    {
                        Some((x + 1, y))
                    } else {
                        None
                    }
                }
                Directions::East => {
                    if Directions::directions_for_tile(*self.tile_at(x, y + 1)?)
                        .contains(&Directions::West)
                    {
                        Some((x, y + 1))
                    } else {
                        None
                    }
                }
                Directions::West => {
                    if y > 0
                        && Directions::directions_for_tile(*self.tile_at(x, y - 1)?)
                            .contains(&Directions::East)
                    {
                        Some((x, y - 1))
                    } else {
                        None
                    }
                }
            })
            .collect()
    }
}

fn main() -> anyhow::Result<()> {
    let grid: Grid = aoc_2023::include_data!(day10).try_into()?;
    let enclosed = grid.enclosed_coords();
    // let mut clean_board = grid.cleaned_board();
    // for &(x, y) in &enclosed {
    //     clean_board[x][y] = 'I';
    // }
    // let printable: Vec<String> = clean_board
    //     .iter()
    //     .map(|r| r.iter().collect::<String>())
    //     .collect();
    // println!("{}", printable.join("\n"));

    println!("Day 10 result: {}", enclosed.len());
    Ok(())
}
