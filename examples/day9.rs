use anyhow::Result;
use aoc23::read_lines;
use itertools::Itertools;

fn pairwise_diff(values: &[isize]) -> Vec<isize> {
    values
        .into_iter()
        .tuple_windows()
        .map(|(x, y)| y - x)
        .collect_vec()
}

fn extrapolate(values: &[isize]) -> (isize, isize) {
    if values.into_iter().all(|&v| v == 0) {
        (0, 0)
    } else {
        let diff = pairwise_diff(values);
        let (prev, next) = extrapolate(&diff);
        let first = values.first().unwrap();
        let last = values.last().unwrap();
        (first - prev, last + next)
    }
}

fn main() -> Result<()> {
    let lines = read_lines("input/day9.txt")?;

    let (sum_prev, sum_next) = lines
        .iter()
        .map(|s| {
            let values = s
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec();
            extrapolate(&values)
        })
        .reduce(|(sum_prev, sum_next), (prev, next)| (sum_prev + prev, sum_next + next))
        .unwrap();
    println!("Part I: {sum_next}");
    println!("Part II: {sum_prev}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(&[0,3,6,9,12,15], (-3, 18))]
    #[case(&[0,-3,-6,-9,-12,-15], (3, -18))]
    #[case(&[1, 3, 6, 10, 15, 21], (0, 28))]
    #[case(&[10, 13, 16, 21, 30, 45], (5, 68))]
    fn test_extrapolate(#[case] values: &[isize], #[case] expected: (isize, isize)) {
        let (prev, next) = extrapolate(values);
        assert_eq!(prev, expected.0);
        assert_eq!(next, expected.1);
    }
}
