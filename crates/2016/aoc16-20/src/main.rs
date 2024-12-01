use hymns::runner::timed_run;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn load_ranges() -> Vec<(u32, u32)> {
    let mut ranges: Vec<(u32, u32)> = INPUT
        .lines()
        .map(|line| {
            let (lo, hi) = line.split('-').collect_tuple().unwrap();
            (lo.parse().unwrap(), hi.parse().unwrap())
        })
        .collect();
    ranges.sort_unstable();
    ranges
}

fn part1() -> u32 {
    let ranges = load_ranges();
    let mut end = 0;

    for (next_start, next_end) in ranges {
        if next_end <= end {
            continue;
        }

        if next_start <= end + 1 {
            end = next_end;
            continue;
        }

        return end + 1;
    }

    unreachable!()
}

fn part2() -> u32 {
    let ranges = load_ranges();

    let mut allowed = 0;
    let mut end = 0;

    for (next_start, next_end) in ranges {
        if next_end <= end {
            continue;
        }

        if next_start <= end + 1 {
            end = next_end;
            continue;
        }

        allowed += next_start - end - 1;
        end = next_end;
    }

    allowed + (u32::MAX - end)
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
        assert_eq!(part1(), 23_923_783);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 125);
    }
}
