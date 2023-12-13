use std::collections::BinaryHeap;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> u64 {
    INPUT
        .split("\n\n")
        .map(|inventory| inventory.lines().map(|s| s.parse::<u64>().unwrap()).sum())
        .max()
        .unwrap()
}

fn part2() -> i64 {
    let elf_totals = INPUT.split("\n\n").map(|inventory| {
        inventory
            .lines()
            .map(|s| s.parse::<i64>().unwrap())
            .sum::<i64>()
    });

    let mut heap: BinaryHeap<i64> = BinaryHeap::new();

    for total in elf_totals {
        heap.push(-total);

        if heap.len() > 3 {
            heap.pop();
        }
    }

    -heap.iter().sum::<i64>()
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
        assert_eq!(part1(), 71934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 211_447);
    }
}
