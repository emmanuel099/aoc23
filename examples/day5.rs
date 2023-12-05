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

    let mapping_rule_combined = all_mapping_rules
        .into_iter()
        .rev()
        .reduce(|succ, r| r.merge_with_successor(&succ))
        .unwrap();

    let seeds: Vec<usize> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let seed_locations_min = seeds
        .into_iter()
        .map(|seed| mapping_rule_combined.resolve(seed))
        .min()
        .unwrap();
    println!("Part I: {}", seed_locations_min);

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
    let seed_locations_min = seed_ranges
        .into_iter()
        .flatten()
        .map(|seed| mapping_rule_combined.resolve(seed))
        .min()
        .unwrap();
    println!("Part II: {}", seed_locations_min);

    Ok(())
}

fn parse_mapping_rules(lines: &[String]) -> Result<MappingRules> {
    let mut rules = RangeMap::new();
    for s in lines {
        let rule: MappingRule = s.parse()?;
        rules.insert(rule.source.clone(), rule);
    }
    Ok(MappingRules { rules })
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn merge_with_successor(&self, successor: &MappingRules) -> MappingRules {
        let mut combined_rules = self.rules.clone();

        for (range, rule) in self.rules.iter() {
            let dest_range = rule.dest_start..rule.dest_start + range.len();
            let overlapping_ranges = successor.rules.overlapping(&dest_range);

            for (successor_range, successor_rule) in overlapping_ranges {
                let overlapping_range = (dest_range.start.max(successor_range.start))
                    ..(dest_range.end.min(successor_range.end));

                let source_start = range.start + overlapping_range.start - rule.dest_start;
                let source_range = source_start..source_start + overlapping_range.len();

                combined_rules.insert(
                    source_range.clone(),
                    MappingRule {
                        dest_start: successor_rule.dest_start + overlapping_range.start
                            - successor_range.start,
                        source: source_range,
                    },
                );
            }
        }

        for (range, rule) in successor.rules.iter() {
            let overlapping_ranges = self.rules.overlapping(range);

            let mut remaining_range = range.clone();

            for (overlapping_range, _) in overlapping_ranges {
                let range_before = remaining_range.start..overlapping_range.start;
                if !range_before.is_empty() {
                    combined_rules.insert(
                        range_before.clone(),
                        MappingRule {
                            dest_start: rule.dest_start + range_before.start - range.start,
                            source: range_before.clone(),
                        },
                    );
                    remaining_range = range_before.end..remaining_range.end;
                }

                remaining_range = overlapping_range.end..remaining_range.end;
            }

            if !remaining_range.is_empty() {
                combined_rules.insert(
                    remaining_range.clone(),
                    MappingRule {
                        dest_start: rule.dest_start + remaining_range.start - range.start,
                        source: remaining_range,
                    },
                );
            }
        }

        let mut combined_rules_fixed = RangeMap::new();
        for (range, mut rule) in combined_rules.into_iter() {
            if range != rule.source {
                rule.dest_start += range.start - rule.source.start;
                rule.source = range.clone();
            }
            combined_rules_fixed.insert(range, rule);
        }

        MappingRules {
            rules: combined_rules_fixed,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merging_of_mapping_rules() {
        let rules1 = {
            let mut rules = RangeMap::new();
            rules.insert(
                98..100,
                MappingRule {
                    dest_start: 50, // 50..52
                    source: 98..100,
                },
            );
            rules.insert(
                50..98,
                MappingRule {
                    dest_start: 52, // 52..100
                    source: 50..98,
                },
            );
            MappingRules { rules }
        };

        let rules2 = {
            let mut rules = RangeMap::new();
            rules.insert(
                15..52,
                MappingRule {
                    dest_start: 0,
                    source: 15..52,
                },
            );
            rules.insert(
                52..54,
                MappingRule {
                    dest_start: 37,
                    source: 52..54,
                },
            );
            rules.insert(
                0..15,
                MappingRule {
                    dest_start: 39,
                    source: 0..15,
                },
            );
            MappingRules { rules }
        };

        let rules_expected = {
            let mut rules = RangeMap::new();

            // succ 15..52
            // -> 15..50
            rules.insert(
                15..50,
                MappingRule {
                    dest_start: 0,
                    source: 15..50,
                },
            );
            // -> 50..52 (overlaps with 98..100)
            rules.insert(
                98..100,
                MappingRule {
                    dest_start: 35, // 0 + 50 - 15
                    source: 98..100,
                },
            );

            // succ 52..54 (partially overlaps with 50..98)
            rules.insert(
                50..52,
                MappingRule {
                    dest_start: 37,
                    source: 50..52,
                },
            );

            // rest of 50..98 from 52..98
            rules.insert(
                52..98,
                MappingRule {
                    dest_start: 54, // 52 + 52-50
                    source: 52..98,
                },
            );

            // succ 0..15
            rules.insert(
                0..15,
                MappingRule {
                    dest_start: 39,
                    source: 0..15,
                },
            );

            MappingRules { rules }
        };
        for n in 0..1000 {
            let a = rules2.resolve(rules1.resolve(n));
            let b = rules_expected.resolve(n);
            assert_eq!(a, b);
        }
        assert_eq!(rules1.merge_with_successor(&rules2), rules_expected);
    }

    #[test]
    fn test_merging_of_mapping_rules2() {
        let rules1 = {
            let mut rules = RangeMap::new();
            rules.insert(
                77..100,
                MappingRule {
                    dest_start: 45, // 45..68
                    source: 77..100,
                },
            );
            MappingRules { rules }
        };

        let rules2 = {
            let mut rules = RangeMap::new();
            rules.insert(
                55..69,
                MappingRule {
                    dest_start: 5, // 5..19
                    source: 55..69,
                },
            );
            MappingRules { rules }
        };

        let rules_expected = {
            let mut rules = RangeMap::new();

            // succ 55..69
            rules.insert(
                55..69,
                MappingRule {
                    dest_start: 5,
                    source: 55..69,
                },
            );

            // 77..100
            // -> 77..87
            rules.insert(
                77..87,
                MappingRule {
                    dest_start: 45,
                    source: 77..87,
                },
            );
            // -> 87..100 overlaps with succ 55..68
            rules.insert(
                87..100,
                MappingRule {
                    dest_start: 5,
                    source: 87..100,
                },
            );

            MappingRules { rules }
        };

        for n in 0..1000 {
            let a = rules2.resolve(rules1.resolve(n));
            let b = rules_expected.resolve(n);
            assert_eq!(a, b);
        }

        assert_eq!(rules1.merge_with_successor(&rules2), rules_expected);
    }
}
