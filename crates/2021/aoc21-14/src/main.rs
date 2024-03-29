use hymns::more_itertools::{Counter, MoreItertools};
use std::collections::HashMap;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

type Rules = HashMap<(char, char), char>;

fn apply_rules(counter: Counter<(char, char)>, rules: &Rules) -> Counter<(char, char)> {
    let mut new_counter = Counter::new();

    for ((old_prefix, old_suffix), count) in counter {
        let insert = rules[&(old_prefix, old_suffix)];

        *new_counter.get_mut((old_prefix, insert)) += count;
        *new_counter.get_mut((insert, old_suffix)) += count;
    }

    new_counter
}

fn get_most_least_common2(counter: Counter<(char, char)>, last: char) -> (usize, usize) {
    let mut char_counts = Counter::new();

    for ((c1, _), count) in counter {
        *char_counts.get_mut(c1) += count;
    }
    *char_counts.get_mut(last) += 1;

    let mut sorted: Vec<_> = char_counts.into_values().collect();
    sorted.sort_unstable();

    (sorted[0], sorted[sorted.len() - 1])
}

fn read_input() -> (Counter<(char, char)>, Rules, char) {
    let mut line_iter = INPUT.lines();

    let first_line_bytes = line_iter.next().unwrap().as_bytes();

    let initial_counts: Counter<(char, char)> = first_line_bytes
        .windows(2)
        .map(|pair| (pair[0].into(), pair[1].into()))
        .collect_counter();

    line_iter.next(); // skip newline

    let rules: Rules = line_iter
        .map(|line| {
            let bytes = line.as_bytes();
            ((bytes[0].into(), bytes[1].into()), bytes[6].into())
        })
        .collect();

    (
        initial_counts,
        rules,
        (*first_line_bytes.last().unwrap()).into(),
    )
}

fn part1() -> usize {
    let (mut counter, rules, last) = read_input();

    for _ in 0..10 {
        counter = apply_rules(counter, &rules);
    }

    let (min, max) = get_most_least_common2(counter, last);

    max - min
}

fn part2() -> usize {
    let (mut counter, rules, last) = read_input();

    for _ in 0..40 {
        counter = apply_rules(counter, &rules);
    }

    let (min, max) = get_most_least_common2(counter, last);

    max - min
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
        assert_eq!(part1(), 2360);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2_967_977_072_188);
    }
}
