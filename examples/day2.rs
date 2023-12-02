use anyhow::Result;
use aoc23::read_lines;
use std::str::FromStr;

fn main() -> Result<()> {
    let lines = read_lines("input/day2.txt")?;
    let games = parse_games(&lines);

    let possible_game_ids = possible_games_part1(&games);
    println!("Part I: {}", possible_game_ids.into_iter().sum::<usize>());

    println!("Part II: {}", sum_of_power_of_cube_sets_part2(&games));

    Ok(())
}

fn possible_games_part1(games: &[Vec<CubeSet>]) -> Vec<usize> {
    const RED_CUBES: usize = 12;
    const GREEN_CUBES: usize = 13;
    const BLUE_CUBES: usize = 14;

    games
        .iter()
        .enumerate()
        .filter_map(|(i, cube_sets)| {
            let all_within_bounds = cube_sets.iter().all(|set| {
                set.red <= RED_CUBES && set.green <= GREEN_CUBES && set.blue <= BLUE_CUBES
            });
            if all_within_bounds {
                let game_id = i + 1;
                Some(game_id)
            } else {
                None
            }
        })
        .collect()
}

fn sum_of_power_of_cube_sets_part2(games: &[Vec<CubeSet>]) -> usize {
    games
        .iter()
        .map(|cube_sets| {
            let (red, green, blue) = cube_sets.iter().fold((0, 0, 0), |(red, green, blue), set| {
                (red.max(set.red), green.max(set.green), blue.max(set.blue))
            });
            red * green * blue
        })
        .sum()
}

fn parse_games(lines: &[String]) -> Vec<Vec<CubeSet>> {
    lines
        .into_iter()
        .map(|s| {
            let (_, sets) = s.split_once(':').unwrap();
            sets.split(';').map(|s| s.parse().unwrap()).collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for CubeSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(s.split(',').map(|s| s.trim()).fold(
            CubeSet {
                red: 0,
                blue: 0,
                green: 0,
            },
            |mut agg, s| {
                let (n, color) = s.split_once(' ').unwrap();
                let n: usize = n.parse().unwrap();
                match color {
                    "red" => {
                        agg.red += n;
                    }
                    "blue" => {
                        agg.blue += n;
                    }
                    "green" => {
                        agg.green += n;
                    }
                    _ => {}
                }
                agg
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    fn example_games() -> Vec<Vec<CubeSet>> {
        vec![
            vec![
                CubeSet {
                    red: 4,
                    blue: 3,
                    green: 0,
                },
                CubeSet {
                    red: 1,
                    blue: 6,
                    green: 2,
                },
                CubeSet {
                    red: 0,
                    blue: 0,
                    green: 2,
                },
            ],
            vec![
                CubeSet {
                    red: 0,
                    blue: 1,
                    green: 2,
                },
                CubeSet {
                    red: 1,
                    blue: 4,
                    green: 3,
                },
                CubeSet {
                    red: 0,
                    blue: 1,
                    green: 1,
                },
            ],
            vec![
                CubeSet {
                    red: 20,
                    blue: 6,
                    green: 8,
                },
                CubeSet {
                    red: 4,
                    blue: 5,
                    green: 13,
                },
                CubeSet {
                    red: 1,
                    blue: 0,
                    green: 5,
                },
            ],
            vec![
                CubeSet {
                    red: 3,
                    blue: 6,
                    green: 1,
                },
                CubeSet {
                    red: 6,
                    blue: 0,
                    green: 3,
                },
                CubeSet {
                    red: 14,
                    blue: 15,
                    green: 3,
                },
            ],
            vec![
                CubeSet {
                    red: 6,
                    blue: 1,
                    green: 3,
                },
                CubeSet {
                    red: 1,
                    blue: 2,
                    green: 2,
                },
            ],
        ]
    }

    #[rstest]
    #[case("7 blue, 5 red", CubeSet{red:5, blue:7, green:0})]
    #[case("7 blue, 5 red, 1 green", CubeSet{red:5, blue:7, green:1})]
    fn parse_cube_set(#[case] input: &str, #[case] expected: CubeSet) {
        let cube_set: CubeSet = input.parse().unwrap();
        assert_eq!(cube_set, expected);
    }

    #[test]
    fn part1_example() {
        let games = example_games();
        let possible_games = possible_games_part1(&games);
        assert_eq!(possible_games, vec![1, 2, 5]);
    }

    #[test]
    fn part2_example() {
        let games = example_games();
        let possible_games = sum_of_power_of_cube_sets_part2(&games);
        assert_eq!(sum_of_power_of_cube_sets_part2(&EXAMPLE_GAMES), 2286);
    }
}
