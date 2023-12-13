use std::collections::{HashMap, HashSet};

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

type Graph = HashMap<Cave, Vec<Cave>>;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Cave {
    Start,
    End,
    Big(&'static str),
    Small(&'static str),
}

impl TryFrom<&'static str> for Cave {
    type Error = ();

    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        match s {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            _ => {
                if s.chars().next().ok_or(())?.is_ascii_lowercase() {
                    Ok(Cave::Small(s))
                } else {
                    Ok(Cave::Big(s))
                }
            }
        }
    }
}

fn find_paths(map: &Graph, can_visit_smalls_twice: bool) -> usize {
    let mut exploring: Vec<(HashSet<Cave>, Cave, bool)> = vec![(
        HashSet::from([Cave::Start]),
        Cave::Start,
        can_visit_smalls_twice,
    )];

    let mut path_count = 0;

    while let Some((path, last, can_visit_twice)) = exploring.pop() {
        let filtered_neighbors = map
            .get(&last)
            .unwrap()
            .iter()
            .filter(|&&n| n != Cave::Start);

        for neighbor in filtered_neighbors {
            if neighbor == &Cave::End {
                path_count += 1;
            } else if matches!(neighbor, Cave::Big(_)) || !path.contains(neighbor) {
                let mut path = path.clone();
                path.insert(*neighbor);
                exploring.push((path, *neighbor, can_visit_twice));
            } else if can_visit_twice {
                let mut path = path.clone();
                path.insert(*neighbor);
                exploring.push((path, *neighbor, false));
            }
        }
    }

    path_count
}

fn build_graph() -> Graph {
    let mut graph: Graph = HashMap::new();

    for line in INPUT.lines() {
        let mut comps = line.split('-');

        let begin: Cave = comps.next().unwrap().try_into().unwrap();
        let end: Cave = comps.next().unwrap().try_into().unwrap();

        graph.entry(begin).or_default().push(end);
        graph.entry(end).or_default().push(begin);
    }

    graph
}

fn part1() -> usize {
    let graph = build_graph();
    find_paths(&graph, false)
}

fn part2() -> usize {
    let graph = build_graph();
    find_paths(&graph, true)
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
        assert_eq!(part1(), 4754);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 143_562);
    }
}
