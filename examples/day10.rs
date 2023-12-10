use anyhow::Result;
use aoc23::read_lines;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    StartingPosition,
}

impl Tile {
    pub fn adjacent_positions(&self, pos: Position) -> Vec<Position> {
        match self {
            Tile::VerticalPipe => vec![
                Position {
                    y: pos.y - 1,
                    ..pos
                },
                Position {
                    y: pos.y + 1,
                    ..pos
                },
            ],
            Tile::HorizontalPipe => vec![
                Position {
                    x: pos.x - 1,
                    ..pos
                },
                Position {
                    x: pos.x + 1,
                    ..pos
                },
            ],
            Tile::NorthEastBend => vec![
                Position {
                    y: pos.y - 1,
                    ..pos
                },
                Position {
                    x: pos.x + 1,
                    ..pos
                },
            ],
            Tile::NorthWestBend => vec![
                Position {
                    y: pos.y - 1,
                    ..pos
                },
                Position {
                    x: pos.x - 1,
                    ..pos
                },
            ],
            Tile::SouthEastBend => vec![
                Position {
                    y: pos.y + 1,
                    ..pos
                },
                Position {
                    x: pos.x + 1,
                    ..pos
                },
            ],
            Tile::SouthWestBend => vec![
                Position {
                    y: pos.y + 1,
                    ..pos
                },
                Position {
                    x: pos.x - 1,
                    ..pos
                },
            ],
            Tile::Ground => {
                vec![]
            }
            Tile::StartingPosition => {
                vec![
                    Position {
                        x: pos.x - 1,
                        ..pos
                    },
                    Position {
                        x: pos.x + 1,
                        ..pos
                    },
                    Position {
                        y: pos.y - 1,
                        ..pos
                    },
                    Position {
                        y: pos.y + 1,
                        ..pos
                    },
                ]
            }
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::VerticalPipe),
            '-' => Ok(Tile::HorizontalPipe),
            'L' => Ok(Tile::NorthEastBend),
            'J' => Ok(Tile::NorthWestBend),
            '7' => Ok(Tile::SouthWestBend),
            'F' => Ok(Tile::SouthEastBend),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::StartingPosition),
            _ => Err(()),
        }
    }
}

fn expand_path(grid: &[Vec<Tile>], start: Position, next: Position) -> Vec<Position> {
    let mut path = Vec::new();

    let mut visited = HashSet::new();

    let mut current = start;
    let mut next = next;
    loop {
        if visited.contains(&next) {
            path.push(next);
            break path;
        }
        let adjacent_positions = grid[next.y][next.x].adjacent_positions(next);
        if adjacent_positions.contains(&current) {
            path.push(next);
            visited.insert(current);
            let next_next = adjacent_positions
                .into_iter()
                .filter(|&pos| pos != current)
                .exactly_one()
                .unwrap();
            current = next;
            next = next_next;
        } else {
            break vec![];
        }
    }
}

fn find_loop(grid: &[Vec<Tile>]) -> Vec<Position> {
    let start = grid
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &tile)| tile == Tile::StartingPosition)
                .map(|(x, _)| Position { x, y })
        })
        .exactly_one()
        .unwrap();

    Tile::StartingPosition
        .adjacent_positions(start)
        .into_iter()
        .filter_map(|next| {
            let path = expand_path(grid, start, next);
            if path.ends_with(&[start]) {
                Some(path)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn parse_grid(lines: &[String]) -> Vec<Vec<Tile>> {
    let mut grid = lines
        .into_iter()
        .map(|line| {
            let mut tiles = line.chars().map(|c| c.try_into().unwrap()).collect_vec();
            tiles.insert(0, Tile::Ground);
            tiles.push(Tile::Ground);
            tiles
        })
        .collect_vec();
    grid.insert(0, vec![Tile::Ground; grid[0].len()]);
    grid.push(vec![Tile::Ground; grid[0].len()]);
    grid
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileState {
    Boundary,
    Outside,
    Unknown,
}

fn tiles_in_loop(grid: &[Vec<Tile>], loop_path: &[Position]) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut tile_state = vec![vec![TileState::Unknown; width * 2]; height * 2];

    for (pos, next) in loop_path.into_iter().circular_tuple_windows() {
        tile_state[pos.y * 2][pos.x * 2] = TileState::Boundary;
        if next.y > pos.y {
            tile_state[pos.y * 2 + 1][pos.x * 2] = TileState::Boundary;
        }
        if next.y < pos.y {
            tile_state[pos.y * 2 - 1][pos.x * 2] = TileState::Boundary;
        }
        if next.x > pos.x {
            tile_state[pos.y * 2][pos.x * 2 + 1] = TileState::Boundary;
        }
        if next.x < pos.x {
            tile_state[pos.y * 2][pos.x * 2 - 1] = TileState::Boundary;
        }
    }

    let mut queue_outside = Vec::new();
    queue_outside.push(Position { x: 0, y: 0 });
    tile_state[0][0] = TileState::Outside;

    while let Some(pos) = queue_outside.pop() {
        let mut neighbours = Vec::with_capacity(4);
        if pos.x > 0 {
            neighbours.push(Position {
                x: pos.x - 1,
                ..pos
            });
        }
        if pos.x < width * 2 - 1 {
            neighbours.push(Position {
                x: pos.x + 1,
                ..pos
            });
        }
        if pos.y > 0 {
            neighbours.push(Position {
                y: pos.y - 1,
                ..pos
            });
        }
        if pos.y < height * 2 - 1 {
            neighbours.push(Position {
                y: pos.y + 1,
                ..pos
            });
        }
        for neighbour in neighbours {
            if tile_state[neighbour.y][neighbour.x] == TileState::Unknown {
                tile_state[neighbour.y][neighbour.x] = TileState::Outside;
                queue_outside.push(neighbour);
            }
        }
    }

    /*for row in &tile_state {
        for &tile in row {
            match tile {
                TileState::Boundary => print!("B"),
                TileState::Outside => print!("O"),
                TileState::Unknown => print!("."),
            }
        }
        println!("");
    }
    println!("");
    println!("");*/

    let mut tile_state2 = vec![vec![TileState::Unknown; width]; height];
    for y in 0..height * 2 {
        for x in 0..width * 2 {
            if tile_state[y][x] == TileState::Boundary {
                tile_state2[y / 2][x / 2] = TileState::Boundary;
            }
            if tile_state2[y / 2][x / 2] == TileState::Unknown {
                tile_state2[y / 2][x / 2] = tile_state[y][x];
            }
        }
    }

    /*for row in &tile_state2 {
        for &tile in row {
            match tile {
                TileState::Boundary => print!("B"),
                TileState::Outside => print!("O"),
                TileState::Unknown => print!("."),
            }
        }
        println!("");
    }*/

    tile_state2
        .into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|&tile| tile == TileState::Unknown)
                .count()
        })
        .sum()
}

fn main() -> Result<()> {
    let lines = read_lines("input/day10.txt")?;
    let grid = parse_grid(&lines);
    //println!("{grid:#?}");

    let loop_path = find_loop(&grid);
    //println!("path: {loop_path:?}");
    println!("Part I: {}", loop_path.len() / 2);

    println!("Part II: {}", tiles_in_loop(&grid, &loop_path));

    Ok(())
}
