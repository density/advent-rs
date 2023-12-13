use std::collections::VecDeque;

use lazy_static::lazy_static;
use regex::Regex;

use hymns::runner::timed_run;

lazy_static! {
    static ref RE: Regex = Regex::new(r"Starting items: (?P<item_list>.+)\n  Operation: new = old (?P<operator>.) (?P<operand>.+)\n.+?(?P<divisor>\d+)\n.+?(?P<true_target>\d+)\n.+?(?P<false_target>\d+)").unwrap();
}

const INPUT: &str = include_str!("../input.txt");

type MonkeyId = usize;
type WorryLevel = u64;

struct Monkey {
    items: VecDeque<WorryLevel>,
    operation: Box<dyn Fn(WorryLevel) -> WorryLevel>,
    divisor: u64,
    true_target: MonkeyId,
    false_target: MonkeyId,
    inspection_count: usize,
    modulus: Option<u64>,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let caps = RE.captures(s).unwrap();

        let operation: Box<dyn Fn(u64) -> u64> = match (&caps["operator"], &caps["operand"]) {
            ("*", "old") => Box::new(|old| old * old),
            ("*", n) => {
                let n: u64 = n.parse().unwrap();
                Box::new(move |old| old * n)
            }
            ("+", n) => {
                let n: u64 = n.parse().unwrap();
                Box::new(move |old| old + n)
            }
            _ => unreachable!(),
        };

        Self {
            items: caps["item_list"]
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect(),
            operation,
            divisor: caps["divisor"].parse().unwrap(),
            true_target: caps["true_target"].parse().unwrap(),
            false_target: caps["false_target"].parse().unwrap(),
            inspection_count: 0,
            modulus: None,
        }
    }
}
impl Monkey {
    fn inspect_and_throw(&mut self) -> Vec<(MonkeyId, WorryLevel)> {
        self.items
            .drain(..)
            .map(|mut worry_level| {
                self.inspection_count += 1;
                worry_level = (self.operation)(worry_level);

                if let Some(modulus) = self.modulus {
                    worry_level %= modulus;
                } else {
                    worry_level /= 3;
                }

                if worry_level % self.divisor == 0 {
                    (self.true_target, worry_level)
                } else {
                    (self.false_target, worry_level)
                }
            })
            .collect()
    }
}

fn part1() -> usize {
    let mut monkeys: Vec<Monkey> = INPUT.split("\n\n").map(Into::into).collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let targets_and_worries = monkeys[i].inspect_and_throw();

            for (target, worry) in targets_and_worries {
                monkeys[target].items.push_back(worry);
            }
        }
    }

    monkeys.sort_unstable_by_key(|m| m.inspection_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspection_count)
        .product()
}

fn part2() -> usize {
    let mut monkeys: Vec<Monkey> = INPUT.split("\n\n").map(Into::into).collect();

    // works because the divisors are all co-prime
    // otherwise, need LCM
    let lcm = monkeys.iter().map(|m| m.divisor).product();

    for monkey in &mut monkeys {
        monkey.modulus = Some(lcm);
    }

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let targets_and_worries = monkeys[i].inspect_and_throw();

            for (target, worry) in targets_and_worries {
                monkeys[target].items.push_back(worry);
            }
        }
    }

    monkeys.sort_unstable_by_key(|m| m.inspection_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspection_count)
        .product()
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
        assert_eq!(part1(), 55930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 14636993466);
    }
}
