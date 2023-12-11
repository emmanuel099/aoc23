use anyhow::Result;
use aoc23::read_lines;
use itertools::Itertools;
use std::collections::HashSet;

trait AbsDiff<T> {
    type Output;

    fn abs_diff(&self, other: &T) -> Self::Output;
}

impl AbsDiff<usize> for usize {
    type Output = usize;

    fn abs_diff(&self, other: &usize) -> Self::Output {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn manhattan_distance(&self, other: &Position) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn find_galaxies(space: &[Vec<bool>]) -> Vec<Position> {
    space
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, is_galaxy)| {
                if *is_galaxy {
                    Some(Position { x, y })
                } else {
                    None
                }
            })
        })
        .collect()
}

fn expand_space(mut space: Vec<Vec<bool>>, factor: usize) -> Vec<Vec<bool>> {
    let width = space[0].len();
    let height = space.len();
    for y in (0..height).rev() {
        if space[y].iter().any(|&is_galaxy| is_galaxy) {
            continue;
        }

        space.insert(y, vec![false; width]);
    }

    let height = space.len();
    'row: for x in (0..width).rev() {
        for y in 0..height {
            if space[y][x] {
                continue 'row;
            }
        }

        for y in 0..height {
            space[y].insert(x, false);
        }
    }

    space
}

fn main() -> Result<()> {
    let lines = read_lines("input/day11.txt")?;

    let space = lines
        .into_iter()
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

    let space = expand_space(space, 1);

    /*for row in &space {
        for &is_galaxy in row {
            if is_galaxy {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("");
    }
    println!("");*/

    let galaxies = find_galaxies(&space);
    let sum_of_lengths: usize = galaxies
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| p1.manhattan_distance(p2))
        .sum();
    println!("Part I: {sum_of_lengths}");

    Ok(())
}
