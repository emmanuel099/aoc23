use anyhow::{anyhow, Result};
use aoc23::read_lines;
use std::{collections::HashSet, str::FromStr};

fn main() -> Result<()> {
    let lines = read_lines("input/day4.txt")?;
    let cards = parse_cards(&lines);

    let matching_numbers_per_card: Vec<_> = cards.iter().map(|c| c.mathing_numbers()).collect();

    let total_points: usize = matching_numbers_per_card
        .iter()
        .map(|&n| if n > 0 { 2usize.pow(n as u32 - 1) } else { 0 })
        .sum();
    println!("Part I: {total_points}");

    let scratchcards = matching_numbers_per_card
        .iter()
        .rfold(Vec::with_capacity(cards.len()), |mut scratchcards, &n| {
            let suffix_sum: usize = (0..n.min(scratchcards.len()))
                .map(|i| scratchcards[i])
                .sum();
            scratchcards.insert(0, suffix_sum + 1);
            scratchcards
        })
        .to_vec();
    let scratchcards_total: usize = scratchcards.iter().sum();
    println!("Part II: {scratchcards_total}");

    Ok(())
}

fn parse_cards(lines: &[String]) -> Vec<Card> {
    lines
        .into_iter()
        .map(|s| {
            let (_, numbers) = s.split_once(':').unwrap();
            numbers.parse().unwrap()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Card {
    winning_numbers: HashSet<usize>,
    own_numbers: HashSet<usize>,
}

impl Card {
    pub fn mathing_numbers(&self) -> usize {
        let overlap: std::collections::hash_set::Intersection<
            '_,
            usize,
            std::collections::hash_map::RandomState,
        > = self.winning_numbers.intersection(&self.own_numbers);
        overlap.count()
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (winning_numbers, own_numbers) = s.split_once('|').ok_or(anyhow!("malformed input"))?;
        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(|s| s.trim())
            .filter_map(|s| s.parse().ok())
            .collect();
        let own_numbers = own_numbers
            .split_whitespace()
            .map(|s| s.trim())
            .filter_map(|s| s.parse().ok())
            .collect();
        Ok(Card {
            winning_numbers,
            own_numbers,
        })
    }
}
