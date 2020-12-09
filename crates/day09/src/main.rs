

use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn sums_to_target(nums: &[i64], target: i64) -> bool {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return true;
            }
        }
    }

    false
}

fn part1() -> i64 {
    let nums: Vec<i64> = INPUT
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    nums.windows(26)
        .find_map(|window| {
            let target = window[window.len() - 1];
            if sums_to_target(&window[..window.len() - 1], target) {
                None
            } else {
                Some(target)
            }
        })
        .unwrap()
}

fn part2() -> i64 {
    let nums: Vec<i64> = INPUT
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let target = 556543474;

    let mut lo = 0;
    let mut hi = 0;

    let mut total = 0;

    while total != target {
        if total < target {
            total += nums[hi];
            hi += 1;
        } else {
            total -= nums[lo];
            lo += 1;
        }
    }

    let slice = &nums[lo..=hi];
    slice.iter().min().unwrap() + slice.iter().max().unwrap()
}

fn main() {
    let start = Instant::now();
    println!("part 1: {}", part1());
    println!("part 1 took {}ms", (Instant::now() - start).as_millis());
    let start = Instant::now();
    println!("part 2: {}", part2());
    println!("part 2 took {}ms", (Instant::now() - start).as_millis());
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 556543474);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 76096372);
    }
}
