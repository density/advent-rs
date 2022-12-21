use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use hymns::p3;
use hymns::runner::timed_run;
use hymns::vector3::Point3;

const INPUT: &str = include_str!("../input.txt");

type Point = Point3<isize>;

fn build_cubes() -> HashSet<Point> {
    INPUT
        .lines()
        .map(|line| {
            let tup: (isize, isize, isize) = line
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            tup.into()
        })
        .collect()
}

fn part1() -> usize {
    let cubes = build_cubes();

    cubes
        .iter()
        .map(|c| {
            6 - c
                .neighbors(false)
                .iter()
                .filter(|neighbor| cubes.contains(neighbor))
                .count()
        })
        .sum()
}

fn part2() -> usize {
    let cubes = build_cubes();

    let begin = [
        isize::MAX,
        isize::MIN,
        isize::MAX,
        isize::MIN,
        isize::MAX,
        isize::MIN,
    ];

    let [min_x, max_x, min_y, max_y, min_z, max_z] = cubes.iter().fold(begin, |accum, p| {
        [
            accum[0].min(p.x - 1),
            accum[1].max(p.x + 1),
            accum[2].min(p.y - 1),
            accum[3].max(p.y + 1),
            accum[4].min(p.z - 1),
            accum[5].max(p.z + 1),
        ]
    });

    let mut visited: HashSet<Point3<isize>> = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(p3!(min_x, min_y, min_z));

    while let Some(cur) = queue.pop_front() {
        for neighbor in cur.neighbors(false) {
            if (min_x..=max_x).contains(&neighbor.x)
                && (min_y..=max_y).contains(&neighbor.y)
                && (min_z..=max_z).contains(&neighbor.z)
                && !cubes.contains(&neighbor)
                && !visited.contains(&neighbor)
            {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    cubes
        .iter()
        .map(|c| {
            c.neighbors(false)
                .iter()
                .filter(|neighbor| !cubes.contains(neighbor) && visited.contains(neighbor))
                .count()
        })
        .sum()
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
        assert_eq!(part1(), 4456);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2510);
    }
}
