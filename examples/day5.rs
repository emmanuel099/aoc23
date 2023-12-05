use anyhow::{anyhow, Result};
use aoc23::read_lines;
use itertools::Itertools;
use rangemap::RangeMap;
use std::{ops::Range, str::FromStr};

fn main() -> Result<()> {
    let lines = read_lines("input/day5.txt")?;

    let all_mapping_rules: Vec<_> = lines
        .split(|l| l.is_empty())
        .skip(1)
        .filter(|section| !section.is_empty())
        .map(|section| parse_mapping_rules(&section[1..]).unwrap())
        .collect();

    let seeds: Vec<usize> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let seed_locations: Vec<_> = seeds
        .into_iter()
        .map(|seed| resolve_seed_location(seed, &all_mapping_rules))
        .collect();
    println!("Part I: {}", seed_locations.iter().min().unwrap());

    let seed_ranges: Vec<Range<usize>> = lines[0]
        .split_whitespace()
        .skip(1)
        .tuples()
        .map(|(start, len)| {
            let start = start.parse().unwrap();
            let len: usize = len.parse().unwrap();
            start..start + len
        })
        .collect();
    let seed_locations: Vec<_> = seed_ranges
        .into_iter()
        .flatten()
        .map(|seed| resolve_seed_location(seed, &all_mapping_rules))
        .collect();
    println!("Part II: {}", seed_locations.iter().min().unwrap());

    Ok(())
}

fn resolve_seed_location(seed: usize, all_mapping_rules: &[MappingRules]) -> usize {
    all_mapping_rules
        .into_iter()
        .fold(seed, |src, mapping_rules| mapping_rules.resolve(src))
}

fn parse_mapping_rules(lines: &[String]) -> Result<MappingRules> {
    let mut rules = RangeMap::new();
    for s in lines {
        let rule: MappingRule = s.parse()?;
        rules.insert(rule.source.clone(), rule);
    }
    Ok(MappingRules { rules })
}

#[derive(Debug, Clone)]
struct MappingRules {
    rules: RangeMap<usize, MappingRule>,
}

impl MappingRules {
    pub fn resolve(&self, n: usize) -> usize {
        if let Some(rule) = self.rules.get(&n) {
            rule.resolve(n)
        } else {
            n
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MappingRule {
    dest_start: usize,
    source: Range<usize>,
}

impl MappingRule {
    pub fn resolve(&self, n: usize) -> usize {
        if self.source.contains(&n) {
            self.dest_start + n - self.source.start
        } else {
            n
        }
    }
}

impl FromStr for MappingRule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<_> = s.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(anyhow!("malformed mapping rule"));
        }
        let dest_start = parts[0].parse()?;
        let src_start = parts[1].parse()?;
        let src_len: usize = parts[2].parse()?;
        Ok(Self {
            dest_start,
            source: src_start..src_start + src_len,
        })
    }
}
