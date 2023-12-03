use anyhow::Result;
use aoc23::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::{collections::BTreeMap, ops::Range};

fn main() -> Result<()> {
    let lines = read_lines("input/day3.txt")?;
    let symbol_map = SymbolMap::parse_lines(&lines);
    let numbers = find_numbers(&lines);

    let adjacent_numbers: Vec<_> = numbers
        .clone()
        .into_iter()
        .filter_map(|(x, y, n)| {
            if symbol_map.is_symbol_adjacent(x, y) {
                Some(n)
            } else {
                None
            }
        })
        .collect();
    println!("Part I: {}", adjacent_numbers.iter().sum::<usize>());

    let mut gear_map: BTreeMap<(usize, usize), Vec<usize>> = BTreeMap::new();
    for (x, y, n) in numbers {
        let adjacent_symbols = symbol_map.adjacent_symbols(x, y);
        for (pos, c) in adjacent_symbols {
            if c != '*' {
                continue;
            }
            gear_map.entry(pos).or_default().push(n);
        }
    }
    let gear_ratio_sum: usize = gear_map
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum();
    println!("Part II: {}", gear_ratio_sum);

    Ok(())
}

fn find_numbers(lines: &[String]) -> Vec<(Range<usize>, usize, usize)> {
    let re = Regex::new(r"\d+").unwrap();

    lines
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            re.find_iter(line).map(move |m| {
                let x = m.range();
                let n: usize = m.as_str().parse().unwrap();
                (x, y, n)
            })
        })
        .collect()
}

struct SymbolMap {
    map: BTreeMap<(usize, usize), char>,
}

impl SymbolMap {
    pub fn parse_lines(lines: &[String]) -> Self {
        let map = lines
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .into_iter()
                    .enumerate()
                    .filter_map(move |(x, c)| {
                        if c != '.' && !c.is_numeric() {
                            Some(((x, y), c))
                        } else {
                            None
                        }
                    })
            })
            .collect();
        Self { map }
    }

    pub fn adjacent_symbols(&self, x: Range<usize>, y: usize) -> Vec<((usize, usize), char)> {
        let x_boundary = x.start.saturating_sub(1)..=x.end;
        let y_boundary = y.saturating_sub(1)..=y + 1;

        x_boundary
            .cartesian_product(y_boundary)
            .filter_map(move |pos| self.map.get(&pos).map(|c| (pos, *c)))
            .collect()
    }

    pub fn is_symbol_adjacent(&self, x: Range<usize>, y: usize) -> bool {
        !self.adjacent_symbols(x, y).is_empty()
    }
}
