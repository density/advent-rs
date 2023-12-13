use std::ops::RangeInclusive;

use hymns::input::parse_char_delimited_numbers;
use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn read_ranges() -> impl Iterator<Item = (RangeInclusive<u64>, RangeInclusive<u64>)> {
    INPUT.lines().map(|line| {
        let mut ranges = line.split(',').map(|range_str| {
            let mut it = parse_char_delimited_numbers(range_str, '-');
            RangeInclusive::new(it.next().unwrap(), it.next().unwrap())
        });

        (ranges.next().unwrap(), ranges.next().unwrap())
    })
}

fn part1() -> usize {
    read_ranges()
        .filter(|(r1, r2)| {
            (r1.start() <= r2.start() && r1.end() >= r2.end())
                || (r2.start() <= r1.start() && r2.end() >= r1.end())
        })
        .count()
}

fn part2() -> usize {
    read_ranges()
        .filter(|(r1, r2)| {
            (r1.end() >= r2.start() && r1.start() <= r2.end())
                || (r2.end() >= r1.start() && r2.start() <= r1.end())
        })
        .count()
}

fn main() {
    timed_run(1, part1);
    timed_run(2, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 444);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 801);
    }
}
