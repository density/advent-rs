use std::cmp::Ordering;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> u64 {
    let mut have: HashSet<u64> = HashSet::new();
    let mut need: Vec<u64> = Vec::new();

    INPUT.lines().filter_map(|n| n.parse().ok()).for_each(|n| {
        have.insert(n);
        need.push(2020 - n);
    });

    need.into_iter()
        .find_map(|n| {
            if have.contains(&n) {
                Some(n * (2020 - n))
            } else {
                None
            }
        })
        .unwrap()
}

fn part2() -> u64 {
    let mut nums: Vec<u64> = INPUT.lines().filter_map(|n| n.parse().ok()).collect();
    nums.sort_unstable();

    for i in 0..nums.len() {
        let mut j = i + 1;
        let mut k = nums.len() - 1;

        while j < k {
            let total = nums[i] + nums[j] + nums[k];

            match total.cmp(&2020) {
                Ordering::Less => j += 1,
                Ordering::Equal => return nums[i] * nums[j] * nums[k],
                Ordering::Greater => k -= 1,
            }
        }
    }

    unreachable!()
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
        assert_eq!(part1(), 927684);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 292093004);
    }
}
