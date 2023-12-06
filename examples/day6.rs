use anyhow::Result;
use std::ops::Range;

#[derive(Debug)]
struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn winning_push_times(&self) -> Range<usize> {
        // remaining = time - push_time
        // dist = push_time * remaining
        // ->
        // dist = push_time * (time - push_time)
        // 0 = - push_time ^ 2 + push_time * time - dist
        // ->
        // a = -1
        // b = time
        // c = -dist
        // (min_t, max_t) = (-b +- sqrt(b^2 - 4ac)) / 2a

        let time = self.time as f64;
        let dist = self.record_distance as f64;

        let sqrt_val = (time.powf(2.0) - 4.0 * dist).sqrt();
        let min_push_time = ((-time + sqrt_val) / -2.0 + 1.0).floor() as usize;
        let max_push_time = ((-time - sqrt_val) / -2.0).ceil() as usize;

        min_push_time..max_push_time
    }
}

fn number_of_ways_to_win(races: &[Race]) -> usize {
    races
        .into_iter()
        .map(|race| race.winning_push_times().len())
        .product()
}

fn main() -> Result<()> {
    let races_part1 = vec![
        Race {
            time: 44,
            record_distance: 283,
        },
        Race {
            time: 70,
            record_distance: 1134,
        },
        Race {
            time: 70,
            record_distance: 1134,
        },
        Race {
            time: 80,
            record_distance: 1491,
        },
    ];
    println!("Part I: {}", number_of_ways_to_win(&races_part1));

    let race_part2 = Race {
        time: 44707080,
        record_distance: 283113411341491,
    };
    println!("Part II: {}", race_part2.winning_push_times().len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::part1_first(Race {
            time: 7,
            record_distance: 9,
        }, 2..6)]
    #[case::part1_second(Race {
            time: 15,
            record_distance: 40,
        }, 4..12)]
    #[case::part1_third(Race {
            time: 30,
            record_distance: 200,
        }, 11..20)]
    #[case::part2(Race {
            time: 71530,
            record_distance: 940200,
        }, 14..71517)]
    fn race_winning_push_times(#[case] race: Race, #[case] expected_push_times: Range<usize>) {
        assert_eq!(race.winning_push_times(), expected_push_times);
    }

    #[test]
    fn part1_example() {
        let races = vec![
            Race {
                time: 7,
                record_distance: 9,
            },
            Race {
                time: 15,
                record_distance: 40,
            },
            Race {
                time: 30,
                record_distance: 200,
            },
        ];
        assert_eq!(number_of_ways_to_win(&races), 288);
    }
}
