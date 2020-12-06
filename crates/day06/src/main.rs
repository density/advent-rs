use itertools::Itertools;
use std::collections::HashSet;
use std::convert::TryFrom;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> u64 {
    INPUT
        .split("\n\n")
        .map(|answers| {
            u64::try_from(
                answers
                    .lines()
                    .flat_map(|line| line.chars())
                    .collect::<HashSet<_>>()
                    .len(),
            )
            .unwrap()
        })
        .sum()
}

fn part2() -> u64 {
    INPUT
        .split("\n\n")
        .map(|answers| {
            u64::try_from(
                answers
                    .lines()
                    .map(|line| line.chars().collect::<HashSet<_>>())
                    .fold1(|seen, next| seen.intersection(&next).cloned().collect())
                    .unwrap()
                    .len(),
            )
            .unwrap()
        })
        .sum()
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 6763);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 3512);
    }
}
