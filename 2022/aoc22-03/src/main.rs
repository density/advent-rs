use std::collections::HashSet;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn priority(item: u8) -> u64 {
    match item {
        b'A'..=b'Z' => u64::from(item - b'A') + 27,
        b'a'..=b'z' => u64::from(item - b'a') + 1,
        _ => unreachable!(),
    }
}

fn part1() -> u64 {
    let mut total: u64 = 0;

    for rucksack in INPUT.lines().map(|l| l.as_bytes()) {
        let mid = rucksack.len() / 2;

        let first_half: HashSet<u8> = HashSet::from_iter(rucksack[..mid].iter().cloned());
        let second_half: HashSet<u8> = HashSet::from_iter(rucksack[mid..].iter().cloned());

        let common_element = first_half.intersection(&second_half).next().unwrap();

        total += priority(*common_element);
    }

    total
}

fn part2() -> u64 {
    let mut score: u64 = 0;

    let lines: Vec<_> = INPUT.lines().map(|line| line.as_bytes()).collect();

    for group in lines.chunks(3) {
        let sets: Vec<HashSet<u8>> = group
            .iter()
            .map(|g| HashSet::from_iter(g.iter().cloned()))
            .collect();

        let common = sets[0]
            .iter()
            .find(|c| sets[1].contains(c) && sets[2].contains(c))
            .unwrap();

        score += priority(*common);
    }

    score
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
        assert_eq!(part1(), 7581);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2525);
    }
}
