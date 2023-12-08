use hymns::math::lcm;
use hymns::runner::timed_run;
use itertools::Itertools;
use std::collections::HashMap;
use std::mem;

const INPUT: &str = include_str!("../input.txt");

type Node = &'static str;

enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

struct Network {
    instructions: Box<dyn Iterator<Item = Direction>>,
    nodes: HashMap<Node, (Node, Node)>,
}

impl Network {
    fn new(s: &'static str) -> Self {
        let (instructions, network) = s.split("\n\n").collect_tuple().unwrap();

        Self {
            instructions: Box::new(instructions.chars().map(Direction::from).cycle()),
            nodes: network
                .lines()
                .filter_map(|net| {
                    net.split(|c: char| !char::is_ascii_alphanumeric(&c))
                        .filter(|s| !s.is_empty())
                        .collect_tuple()
                        .map(|(start, left, right)| (start, (left, right)))
                })
                .collect(),
        }
    }
}

fn part1() -> u64 {
    let mut cur = "AAA";

    let mut network = Network::new(INPUT);

    for step in 0..u64::MAX {
        if cur == "ZZZ" {
            return step;
        }

        let (left, right) = network.nodes[cur];
        cur = match network.instructions.next().unwrap() {
            Direction::Left => left,
            Direction::Right => right,
        };
    }

    unreachable!()
}

fn part2() -> u64 {
    let mut network = Network::new(INPUT);

    let mut steps = 0;

    let mut cur_nodes = network
        .nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .collect_vec();

    let mut step_counts = Vec::with_capacity(cur_nodes.len());
    let mut new_nodes = Vec::with_capacity(cur_nodes.len());

    while !cur_nodes.is_empty() {
        let dir = network.instructions.next().unwrap();
        steps += 1;

        for node in cur_nodes.iter() {
            let (left, right) = network.nodes[node];

            let next_node = match dir {
                Direction::Left => left,
                Direction::Right => right,
            };

            if next_node.ends_with('Z') {
                step_counts.push(steps);
            } else {
                new_nodes.push(next_node);
            }
        }

        mem::swap(&mut cur_nodes, &mut new_nodes);
        new_nodes.clear();
    }

    step_counts.into_iter().reduce(lcm).unwrap()
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
        assert_eq!(part1(), 16043);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 15726453850399);
    }
}
