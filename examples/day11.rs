use anyhow::Result;
use aoc23::read_lines;
use itertools::Itertools;

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

fn find_galaxies(space: &[Vec<bool>], distances: &[Vec<usize>]) -> Vec<Position> {
    space
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, is_galaxy)| {
                if *is_galaxy {
                    let h = distances[y].iter().take(x).sum();
                    let v = distances.iter().map(|row| row[x]).take(y).sum();
                    Some(Position { x: h, y: v })
                } else {
                    None
                }
            })
        })
        .collect()
}

fn expand_space(
    space: &[Vec<bool>],
    mut distances: Vec<Vec<usize>>,
    factor: usize,
) -> Vec<Vec<usize>> {
    let width = space[0].len();
    let height = space.len();

    for y in 0..height {
        if space[y].iter().any(|&is_galaxy| is_galaxy) {
            continue;
        }

        distances[y] = vec![factor; width];
    }

    'row: for x in 0..width {
        for y in 0..height {
            if space[y][x] {
                continue 'row;
            }
        }

        for y in 0..height {
            distances[y][x] = factor;
        }
    }

    distances
}

fn main() -> Result<()> {
    let lines = read_lines("input/day11.txt")?;

    let space = lines
        .into_iter()
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

    let orig_distances = vec![vec![1; space[0].len()]; space.len()];

    let distances_2 = expand_space(&space, orig_distances.clone(), 2);
    let galaxies = find_galaxies(&space, &distances_2);
    let sum_of_lengths: usize = galaxies
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| p1.manhattan_distance(p2))
        .sum();
    println!("Part I: {sum_of_lengths}");

    let distances_1000000 = expand_space(&space, orig_distances.clone(), 1000000);
    let galaxies = find_galaxies(&space, &distances_1000000);
    let sum_of_lengths: usize = galaxies
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| p1.manhattan_distance(p2))
        .sum();
    println!("Part II: {sum_of_lengths}");

    Ok(())
}
