use anyhow::Result;
use aoc23::read_lines;
use regex::Regex;

fn main() -> Result<()> {
    let lines = read_lines("input/day1.txt")?;
    println!("Part I: {}", callibration_value_part1(&lines));
    println!("Part II: {}", callibration_value_part2(&lines));

    Ok(())
}

fn callibration_value_part1(lines: &[String]) -> u32 {
    lines
        .into_iter()
        .filter_map(|s| {
            let first = s.chars().find(|c| c.is_numeric())?.to_digit(10)?;
            let last = s.chars().rfind(|c| c.is_numeric())?.to_digit(10)?;
            Some(first * 10 + last)
        })
        .sum()
}

fn parse_digit(s: &str) -> Option<u32> {
    match s {
        "one" | "1" => Some(1),
        "two" | "2" => Some(2),
        "three" | "3" => Some(3),
        "four" | "4" => Some(4),
        "five" | "5" => Some(5),
        "six" | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine" | "9" => Some(9),
        _ => None,
    }
}

fn callibration_value_part2(lines: &[String]) -> u32 {
    const PATTERN: &str = "one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9";

    // Overlapping matches would be much shorter (just take the first and last match), but
    // regex crate doesn't support it. So we reverse the pattern and the string to get the
    // last match (as first match).
    let re = Regex::new(&format!("({})", PATTERN)).unwrap();
    let re_rev = Regex::new(&format!("({})", PATTERN.chars().rev().collect::<String>())).unwrap();

    lines
        .into_iter()
        .filter_map(|s| {
            let rev_s = s.chars().rev().collect::<String>();

            let first_match = re.find(s)?.as_str();
            let last_match_rev = re_rev.find(&rev_s)?.as_str();
            let last_match = last_match_rev.chars().rev().collect::<String>();

            let first = parse_digit(first_match)?;
            let last = parse_digit(&last_match)?;
            Some(first * 10 + last)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = callibration_value_part1(&[
            "1abc2".to_owned(),
            "pqr3stu8vwx".to_owned(),
            "a1b2c3d4e5f".to_owned(),
            "treb7uchet".to_owned(),
        ]);
        assert_eq!(result, 142);
    }

    #[test]
    fn part2_example() {
        let result = callibration_value_part2(&[
            "two1nine".to_owned(),
            "eightwothree".to_owned(),
            "abcone2threexyz".to_owned(),
            "xtwone3four".to_owned(),
            "4nineeightseven2".to_owned(),
            "zoneight234".to_owned(),
            "7pqrstsixteen".to_owned(),
        ]);
        assert_eq!(result, 281);
    }

    #[test]
    fn part2_test_overlapping() {
        let result = callibration_value_part2(&["two1nineight".to_owned()]);
        assert_eq!(result, 28);
    }
}
