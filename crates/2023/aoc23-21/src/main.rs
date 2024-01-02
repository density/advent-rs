use crate::Plot::{Empty, Rock, Start};
use hymns::grid::Grid;
use hymns::runner::timed_run;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::mem;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Eq, PartialEq)]
enum Plot {
    Empty,
    Rock,
    Start,
}

impl From<char> for Plot {
    fn from(value: char) -> Self {
        match value {
            '.' => Empty,
            '#' => Rock,
            'S' => Start,
            _ => unreachable!(),
        }
    }
}

impl Display for Plot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Empty => '.',
            Rock => '#',
            Start => 'S',
        };

        write!(f, "{c}")
    }
}

fn part1() -> usize {
    let mut grid: Grid<Plot> = INPUT.parse().unwrap();

    let loc = grid.iter_points().find(|p| grid[p] == Start).unwrap();
    grid[loc] = Empty;

    let mut locs = HashSet::from([loc]);
    let mut next_locs = HashSet::new();

    for _ in 0..64 {
        for cur_loc in locs.drain() {
            let open_neigh = grid
                .all_neighbors(&cur_loc, false)
                .into_iter()
                .filter(|p| grid[p] == Empty);

            next_locs.extend(open_neigh);
        }

        mem::swap(&mut locs, &mut next_locs);
    }

    locs.len()
}

fn part2() -> u64 {
    0
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
        assert_eq!(part1(), 3671);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
