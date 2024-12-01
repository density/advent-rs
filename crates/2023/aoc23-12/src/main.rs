use hymns::input::parse_numbers_only;
use hymns::runner::timed_run;
use itertools::Itertools;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::iter::repeat;

const INPUT: &str = include_str!("../input.txt");

type MemoKey = (usize, usize, usize);

struct Solver<'a> {
    memo: HashMap<MemoKey, usize>,
    line: &'a [char],
    groupings: &'a [usize],
}

impl Solver<'_> {
    fn solve(&mut self, line_off: usize, group_off: usize, hash_count: usize) -> usize {
        let key = (line_off, group_off, hash_count);

        if let Some(count) = self.memo.get(&key) {
            return *count;
        }

        let mut count = 0;

        if line_off == self.line.len() {
            // if we made it to the end of the line, completed all the groupings, and haven't placed
            // any hashes that don't belong to a group yet, then we've successfully found an arragement that works.
            count = (group_off == self.groupings.len() && hash_count == 0).into();
        } else {
            // if we encounter a hash or an unknown, increment the hash count for the current group
            // (place a hash)
            if matches!(self.line[line_off], '#' | '?') {
                // place a hash
                count += self.solve(line_off + 1, group_off, hash_count + 1);
            }

            // if we encounter a . or an unknown...
            if matches!(self.line[line_off], '.' | '?') {
                if hash_count == 0 {
                    // .. and we haven't placed a hash yet, don't place one here.
                    count += self.solve(line_off + 1, group_off, 0);
                } else if self.groupings.get(group_off) == Some(&hash_count) {
                    // ... and we've placed all the hashes for this group, then we've
                    // completed the group. move on to the next group, for which we haven't placed any hashes yet
                    count += self.solve(line_off + 1, group_off + 1, 0);
                }
            }
        }

        self.memo.insert(key, count);

        count
    }
}

fn run<const REPEATS: usize>() -> usize {
    let cases = INPUT
        .lines()
        .map(|line| {
            let (springs, groupings) = line.split_whitespace().collect_tuple().unwrap();

            let mut springs = repeat(springs).take(REPEATS).join("?");
            // add a . to the end to simplify logic
            springs.push('.');

            let parsed = parse_numbers_only::<usize>(groupings, false).collect_vec();
            let counts = repeat(parsed).take(REPEATS).flatten().collect_vec();

            (springs, counts)
        })
        .collect_vec();

    // parallelize because why not
    cases
        .into_par_iter()
        .map(|(springs, counts)| {
            Solver {
                memo: HashMap::new(),
                line: &springs.chars().collect_vec(),
                groupings: &counts,
            }
            .solve(0, 0, 0)
        })
        .sum()
}

fn part1() -> usize {
    run::<1>()
}

fn part2() -> usize {
    run::<5>()
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
        assert_eq!(part1(), 7251);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2_128_386_729_962);
    }
}
