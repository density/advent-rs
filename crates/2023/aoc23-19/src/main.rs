use std::collections::HashMap;

use itertools::Itertools;

use hymns::runner::timed_run;
use workflow::Verdict::{Accept, Jump, Reject};
use workflow::Workflow;

use crate::workflow::AttributeRanges;

mod parser;
mod workflow;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> u64 {
    let (workflows, parts) = INPUT.split("\n\n").collect_tuple().unwrap();

    let workflows: HashMap<&str, Workflow> = workflows
        .lines()
        .map(|line| {
            let wf = parser::workflow(line);
            (wf.name, wf)
        })
        .collect();

    let mut total = 0;

    for part in parts.lines().map(parser::part) {
        let mut cur_workflow = &workflows["in"];
        loop {
            match cur_workflow.evaluate(&part) {
                Accept => {
                    total += part.attributes.iter().sum::<u64>();
                    break;
                }
                Reject => break,
                Jump(wf) => cur_workflow = &workflows[wf],
            };
        }
    }

    total
}

fn multiply_ranges(ranges: &AttributeRanges) -> u64 {
    ranges.iter().fold(1, |acc, r| {
        let count = (r.end() + 1).saturating_sub(*r.start());
        acc * count
    })
}

fn count_accepted(
    ranges: &AttributeRanges,
    workflows: &HashMap<&str, Workflow>,
    workflow_name: &str,
) -> u64 {
    let mut total = 0;

    let wf = &workflows[workflow_name];

    let mut current = ranges.clone();

    for condition in &wf.conditions {
        let ((decided, verdict), undecided) = condition.split_ranges(&current);

        match verdict {
            Accept => total += multiply_ranges(&decided),
            Reject => (),
            Jump(next) => total += count_accepted(&decided, workflows, next),
        }

        current = undecided;
    }

    match wf.catchall {
        Accept => total += multiply_ranges(&current),
        Reject => (),
        Jump(next) => total += count_accepted(&current, workflows, next),
    };

    total
}

fn part2() -> u64 {
    let workflows: HashMap<&str, Workflow> = INPUT
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let wf = parser::workflow(line);
            (wf.name, wf)
        })
        .collect();

    let ranges = [1..=4000, 1..=4000, 1..=4000, 1..=4000];
    count_accepted(&ranges, &workflows, "in")
}

fn main() {
    timed_run(1, part1);
    timed_run(2, part2);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 398_527);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 133_973_513_090_020);
    }
}
