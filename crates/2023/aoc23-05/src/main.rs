use itertools::Itertools;

use hymns::input::parse_numbers_only;
use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

type IntervalMap = Vec<(Interval, Interval)>;

fn parse_map(lines: &str) -> IntervalMap {
    lines
        .lines()
        .skip(1)
        .map(|line| {
            let (dest, src, len) = parse_numbers_only(line, false).collect_tuple().unwrap();

            (
                Interval::new(src, src + len),
                Interval::new(dest, dest + len),
            )
        })
        .sorted()
        .collect()
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Copy, Clone)]
struct Interval {
    start: u64,
    end: u64,
}

impl Interval {
    fn new(start: u64, end: u64) -> Self {
        debug_assert!(start < end);
        Self { start, end }
    }

    fn len(&self) -> u64 {
        self.end - self.start
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        if start < end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    fn remap_interval(&mut self, src: &Interval, dst: &Interval) {
        debug_assert_eq!(src.len(), dst.len());
        let offset = self.start - src.start;
        let length = self.len();

        self.start = dst.start + offset;
        self.end = self.start + length;
    }
}

fn remap_intervals(in_ranges: &[Interval], mapping: &IntervalMap) -> Vec<Interval> {
    let mut new_intervals = vec![];

    for interval in in_ranges {
        let mut last = interval.start;
        let mut found = false;

        for (src, dst) in mapping {
            if src.end <= interval.start || src.start >= interval.end {
                continue;
            }

            if last < src.start {
                new_intervals.push(Interval::new(last, src.start));
            }

            let mut intersection = interval.intersection(src).unwrap();
            intersection.remap_interval(src, dst);
            new_intervals.push(intersection);

            last = src.end;
            found = true;
        }

        if !found {
            new_intervals.push(*interval);
        } else if last < interval.end {
            new_intervals.push(Interval::new(last, interval.end));
        }
    }

    new_intervals
}

fn part1() -> u64 {
    let mut groups = INPUT.split("\n\n");

    let mut seed_ranges: Vec<Interval> = parse_numbers_only(groups.next().unwrap(), false)
        .map(|start| Interval::new(start, start + 1))
        .collect();

    for map in groups.map(parse_map) {
        seed_ranges = remap_intervals(&seed_ranges, &map);
    }

    seed_ranges
        .iter()
        .map(|interval| interval.start)
        .min()
        .unwrap()
}

fn part2() -> u64 {
    let mut groups = INPUT.split("\n\n");

    let mut seed_ranges: Vec<Interval> = parse_numbers_only(groups.next().unwrap(), false)
        .tuples()
        .map(|(start, len)| Interval::new(start, start + len))
        .collect();

    for map in groups.map(parse_map) {
        seed_ranges = remap_intervals(&seed_ranges, &map);
    }

    seed_ranges
        .iter()
        .map(|interval| interval.start)
        .min()
        .unwrap()
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
        assert_eq!(part1(), 486_613_012);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 56_931_769);
    }
}
