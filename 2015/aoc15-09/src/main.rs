use std::collections::{HashMap, HashSet};
use std::time::Instant;

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

struct Distances {
    distances: HashMap<(String, String), u64>,
    all_cities: HashSet<String>,
}

impl Distances {
    fn new() -> Self {
        Self {
            distances: HashMap::new(),
            all_cities: HashSet::new(),
        }
    }

    fn make_key(src: &str, dst: &str) -> (String, String) {
        if src < dst {
            (src.to_owned(), dst.to_owned())
        } else {
            (dst.to_owned(), src.to_owned())
        }
    }

    fn set_distance(&mut self, src: &str, dst: &str, distance: u64) {
        self.all_cities.insert(src.to_owned());
        self.all_cities.insert(dst.to_owned());
        self.distances.insert(Self::make_key(src, dst), distance);
    }

    fn get_distance(&self, src: &str, dst: &str) -> Option<u64> {
        self.distances.get(&Self::make_key(src, dst)).cloned()
    }
}

fn build_distances() -> Distances {
    let mut distances = Distances::new();

    for line in INPUT.lines() {
        let mut split = line.split_whitespace();

        let src = split.next().unwrap();
        split.next(); // to
        let dst = split.next().unwrap();
        split.next(); // =
        let distance = split.next().unwrap().parse().unwrap();

        distances.set_distance(src, dst, distance);
    }

    distances
}

fn all_path_lengths(distances: &Distances) -> impl Iterator<Item = u64> + '_ {
    distances
        .all_cities
        .iter()
        .permutations(distances.all_cities.len())
        .filter_map(|permutation| {
            permutation
                .iter()
                .tuple_windows()
                .try_fold(0, |acc, (src, dst)| {
                    distances.get_distance(src, dst).map(|d| acc + d)
                })
        })
}

fn part1() -> u64 {
    let distances = build_distances();
    all_path_lengths(&distances).min().unwrap()
}

fn part2() -> u64 {
    let distances = build_distances();
    all_path_lengths(&distances).max().unwrap()
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
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 251);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 898);
    }
}
