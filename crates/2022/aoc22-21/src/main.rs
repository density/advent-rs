use std::collections::HashMap;

use regex::Regex;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

type MonkeyName = &'static str;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn execute(self, left: i64, right: i64) -> i64 {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Job {
    Constant(i64),
    Computation(MonkeyName, MonkeyName, Operation),
    Unknown,
}

impl Job {
    fn constant(&self) -> i64 {
        if let Job::Constant(n) = self {
            *n
        } else {
            panic!("Attempt to get num.")
        }
    }
}

fn read_jobs() -> HashMap<MonkeyName, Job> {
    let re = Regex::new(r"(?P<yeller>.+?): ((?P<num>\d+)|(?P<lhs>.+) (?P<operator>.) (?P<rhs>.+))")
        .unwrap();

    INPUT
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            let yeller = caps.name("yeller").unwrap().as_str();

            if let Some(m) = caps.name("num") {
                return (yeller, Job::Constant(m.as_str().parse().unwrap()));
            }

            let lhs = caps.name("lhs").unwrap().as_str();
            let rhs = caps.name("rhs").unwrap().as_str();

            match &caps["operator"] {
                "*" => (yeller, Job::Computation(lhs, rhs, Operation::Mul)),
                "+" => (yeller, Job::Computation(lhs, rhs, Operation::Add)),
                "-" => (yeller, Job::Computation(lhs, rhs, Operation::Sub)),
                "/" => (yeller, Job::Computation(lhs, rhs, Operation::Div)),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn simplify(jobs: &mut HashMap<MonkeyName, Job>, monkey: MonkeyName) -> Option<i64> {
    match jobs[monkey] {
        Job::Unknown => None,
        Job::Constant(n) => Some(n),
        Job::Computation(l_monkey, r_monkey, op) => {
            match (simplify(jobs, l_monkey), simplify(jobs, r_monkey)) {
                (None, None) => unreachable!(),
                (Some(l), Some(r)) => {
                    let result = op.execute(l, r);
                    jobs.insert(monkey, Job::Constant(result));
                    Some(result)
                }
                _ => None,
            }
        }
    }
}

fn reverse(target: i64, unknown_is_left: bool, known_value: i64, op: Operation) -> i64 {
    match op {
        Operation::Add => target - known_value,
        Operation::Mul => target / known_value,
        Operation::Sub => {
            if unknown_is_left {
                target + known_value
            } else {
                -(target - known_value)
            }
        }
        Operation::Div => {
            if unknown_is_left {
                target * known_value
            } else {
                known_value / target
            }
        }
    }
}

fn reverse_solve(jobs: &mut HashMap<MonkeyName, Job>, monkey: MonkeyName, target_value: i64) {
    match jobs[monkey] {
        Job::Constant(_) => unreachable!(),
        Job::Computation(left_monkey, right_monkey, op) => {
            let (monkey_name, unknown_is_left, known_value) =
                match (jobs[left_monkey], jobs[right_monkey]) {
                    (_, Job::Constant(n)) => (left_monkey, true, n),
                    (Job::Constant(n), _) => (right_monkey, false, n),
                    _ => unreachable!(),
                };

            let result = reverse(target_value, unknown_is_left, known_value, op);

            reverse_solve(jobs, monkey_name, result);
            jobs.insert(monkey_name, Job::Constant(result));
        }
        Job::Unknown => {
            jobs.insert(monkey, Job::Constant(target_value));
        }
    }
}

fn part1() -> i64 {
    simplify(&mut read_jobs(), "root").unwrap()
}

fn part2() -> i64 {
    let mut jobs = read_jobs();
    jobs.insert("humn", Job::Unknown);
    simplify(&mut jobs, "root");

    if let Job::Computation(left_monkey, right_monkey, _) = jobs["root"] {
        if let Job::Constant(n) = jobs[left_monkey] {
            reverse_solve(&mut jobs, right_monkey, n);
        } else if let Job::Constant(n) = jobs[right_monkey] {
            reverse_solve(&mut jobs, left_monkey, n);
        }
    } else {
        unreachable!()
    }

    jobs["humn"].constant()
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
        assert_eq!(part1(), 268_597_611_536_314);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 3_451_534_022_348);
    }
}
