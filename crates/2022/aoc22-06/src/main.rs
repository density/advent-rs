use std::collections::HashSet;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn find_message(length: usize) -> usize {
    INPUT
        .as_bytes()
        .windows(length)
        .position(|w| w.iter().collect::<HashSet<&u8>>().len() == length)
        .unwrap()
        + length
}

fn part1() -> usize {
    find_message(4)
}

fn part2() -> usize {
    find_message(14)
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
        assert_eq!(part1(), 1538);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2315);
    }
}
