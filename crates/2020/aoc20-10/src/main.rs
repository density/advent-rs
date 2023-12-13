use std::collections::VecDeque;
use std::convert::TryFrom;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> u64 {
    let mut numbers: Vec<u64> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    numbers.sort_unstable();

    let mut prev = 0;
    let mut counts = [0; 2];

    for adapter in numbers {
        let diff = usize::try_from(adapter - prev).unwrap();

        let index = diff % 3;
        if index <= 1 {
            counts[index] += 1;
        }

        prev = adapter;
    }

    (counts[0] + 1) * counts[1]
}

fn part2() -> u64 {
    let mut all_adapters: Vec<u64> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    all_adapters.push(0);
    all_adapters.sort_unstable();

    let mut memo = VecDeque::with_capacity(3);
    memo.push_back(1);

    for i in 1..all_adapters.len() {
        let last_3_iter = all_adapters.iter().skip(i.saturating_sub(3)).take(3);
        let cur_adapter = all_adapters[i];

        let ways = memo
            .iter()
            .zip(last_3_iter)
            .fold(0, |acc, (memo_val, adapter)| {
                if cur_adapter - adapter <= 3 {
                    acc + memo_val
                } else {
                    acc
                }
            });

        memo.push_back(ways);
        if memo.len() > 3 {
            memo.pop_front();
        }
    }

    *memo.back().unwrap()
}

fn main() {
    let start = Instant::now();
    println!("part 1: {}", part1());
    println!("part 1 took {}ms", start.elapsed().as_millis());
    let start = Instant::now();
    println!("part 2: {}", part2());
    println!("part 2 took {}ms", start.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1755);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 4049565169664);
    }
}
