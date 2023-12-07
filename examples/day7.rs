use anyhow::{anyhow, Result};
use aoc23::read_lines;
use itertools::Itertools;
use std::{collections::HashMap, hash::Hash, str::FromStr};

mod part1 {
    use super::*;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    enum Card {
        Number(u32),
        T,
        J,
        Q,
        K,
        A,
    }

    impl TryFrom<char> for Card {
        type Error = anyhow::Error;

        fn try_from(c: char) -> Result<Self> {
            match c {
                'A' => Ok(Card::A),
                'K' => Ok(Card::K),
                'Q' => Ok(Card::Q),
                'J' => Ok(Card::J),
                'T' => Ok(Card::T),
                '2'..='9' => Ok(Card::Number(
                    c.to_digit(10).ok_or(anyhow!("invalid number"))?,
                )),
                _ => Err(anyhow!("{c} is not a valid card")),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Hand {
        HighCard(Vec<Card>),
        OnePair(Vec<Card>),
        TwoPair(Vec<Card>),
        ThreeOfAKind(Vec<Card>),
        FullHouse(Vec<Card>),
        FourOfAKind(Vec<Card>),
        FiveOfAKind(Vec<Card>),
    }

    fn count_distinct<T>(values: &[T]) -> HashMap<&T, usize>
    where
        T: PartialEq + Eq + Hash,
    {
        let mut distincts: HashMap<&T, usize> = HashMap::new();
        for v in values {
            distincts.entry(v).and_modify(|n| *n += 1).or_insert(1);
        }
        distincts
    }

    impl FromStr for Hand {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self> {
            let cards = s
                .chars()
                .map(|c| c.try_into())
                .collect::<Result<Vec<Card>>>()?;
            if cards.len() != 5 {
                return Err(anyhow!("expected 5 cards but was {}", cards.len()));
            }

            let distinct_cards = count_distinct(&cards);
            match distinct_cards.len() {
                1 => Ok(Hand::FiveOfAKind(cards)),
                2 => {
                    if distinct_cards.values().contains(&4) {
                        Ok(Hand::FourOfAKind(cards))
                    } else {
                        Ok(Hand::FullHouse(cards))
                    }
                }
                3 => {
                    if distinct_cards.values().contains(&3) {
                        Ok(Hand::ThreeOfAKind(cards))
                    } else {
                        Ok(Hand::TwoPair(cards))
                    }
                }
                4 => Ok(Hand::OnePair(cards)),
                5 => Ok(Hand::HighCard(cards)),
                _ => Err(anyhow!("unexpected deck {s}")),
            }
        }
    }

    pub fn solve(lines: &[String]) {
        let hands = lines
            .iter()
            .map(|s| {
                let (hand, bid) = s.split_once(' ').unwrap();
                (hand.parse::<Hand>().unwrap(), bid.parse().unwrap())
            })
            .sorted_by(|(h1, _), (h2, _)| h1.cmp(h2))
            .collect_vec();

        let total_winnings: usize = hands
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum();
        println!("Part I: {}", total_winnings);
    }
}

mod part2 {
    use super::*;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    enum Card {
        J,
        Number(u32),
        T,
        Q,
        K,
        A,
    }

    impl TryFrom<char> for Card {
        type Error = anyhow::Error;

        fn try_from(c: char) -> Result<Self> {
            match c {
                'A' => Ok(Card::A),
                'K' => Ok(Card::K),
                'Q' => Ok(Card::Q),
                'J' => Ok(Card::J),
                'T' => Ok(Card::T),
                '2'..='9' => Ok(Card::Number(
                    c.to_digit(10).ok_or(anyhow!("invalid number"))?,
                )),
                _ => Err(anyhow!("{c} is not a valid card")),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Hand {
        HighCard(Vec<Card>),
        OnePair(Vec<Card>),
        TwoPair(Vec<Card>),
        ThreeOfAKind(Vec<Card>),
        FullHouse(Vec<Card>),
        FourOfAKind(Vec<Card>),
        FiveOfAKind(Vec<Card>),
    }

    fn count_distinct<T>(values: &[T]) -> HashMap<&T, usize>
    where
        T: PartialEq + Eq + Hash,
    {
        let mut distincts: HashMap<&T, usize> = HashMap::new();
        for v in values {
            distincts.entry(v).and_modify(|n| *n += 1).or_insert(1);
        }
        distincts
    }

    impl FromStr for Hand {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self> {
            let cards = s
                .chars()
                .map(|c| c.try_into())
                .collect::<Result<Vec<Card>>>()?;
            if cards.len() != 5 {
                return Err(anyhow!("expected 5 cards but was {}", cards.len()));
            }

            let mut distinct_cards = count_distinct(&cards);
            let joker_count = distinct_cards.remove(&Card::J).unwrap_or_default();
            match distinct_cards.len() {
                0 | 1 => Ok(Hand::FiveOfAKind(cards)), // 0 is all jokers
                2 => {
                    if distinct_cards.values().contains(&(4 - joker_count)) {
                        Ok(Hand::FourOfAKind(cards))
                    } else {
                        Ok(Hand::FullHouse(cards))
                    }
                }
                3 => {
                    if distinct_cards.values().contains(&(3 - joker_count)) {
                        Ok(Hand::ThreeOfAKind(cards))
                    } else {
                        Ok(Hand::TwoPair(cards))
                    }
                }
                4 => Ok(Hand::OnePair(cards)),
                5 => Ok(Hand::HighCard(cards)),
                _ => Err(anyhow!("unexpected deck {s}")),
            }
        }
    }

    pub fn solve(lines: &[String]) {
        let hands = lines
            .iter()
            .map(|s| {
                let (hand, bid) = s.split_once(' ').unwrap();
                (hand.parse::<Hand>().unwrap(), bid.parse().unwrap())
            })
            .sorted_by(|(h1, _), (h2, _)| h1.cmp(h2))
            .collect_vec();

        let total_winnings: usize = hands
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum();
        println!("Part II: {}", total_winnings);
    }
}

fn main() -> Result<()> {
    let lines = read_lines("input/day7.txt")?;
    part1::solve(&lines);
    part2::solve(&lines);

    Ok(())
}
