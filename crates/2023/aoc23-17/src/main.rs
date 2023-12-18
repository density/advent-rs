use std::hash::Hash;

use itertools::Itertools;

use hymns::grid::{GPoint, Grid};
use hymns::p2;
use hymns::pathfinding::a_star;
use hymns::runner::timed_run;
use hymns::vector2::{Direction, Point2, FOUR_NEIGHBORS};

const INPUT: &str = include_str!("../input.txt");

const STARTS: [State; 2] = [
    State {
        point: p2!(0, 0),
        dir: Direction::Right,
        consecutive: 0,
    },
    State {
        point: p2!(0, 0),
        dir: Direction::Down,
        consecutive: 0,
    },
];

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct State {
    point: GPoint,
    dir: Direction,
    consecutive: usize,
}

impl State {
    fn get_neighbors(
        &self,
        grid: &Grid<u8>,
        min_consecutive: usize,
        max_consecutive: usize,
    ) -> Vec<Self> {
        let dirs = if self.consecutive < min_consecutive {
            vec![self.dir]
        } else {
            FOUR_NEIGHBORS
                .into_iter()
                .filter(|&dir| {
                    dir != self.dir.inverted()
                        && !(self.consecutive == max_consecutive && self.dir == dir)
                })
                .collect_vec()
        };

        dirs.into_iter()
            .filter_map(|dir| {
                grid.get_neighbor(&self.point, dir).map(|neigh| Self {
                    point: neigh,
                    dir,
                    consecutive: if dir == self.dir {
                        self.consecutive + 1
                    } else {
                        1
                    },
                })
            })
            .collect_vec()
    }
}

fn solve<const MIN_CONSECUTIVE: usize, const MAX_CONSECUTIVE: usize>() -> usize {
    let grid: Grid<u8> = Grid::new(
        INPUT
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
                    .collect_vec()
            })
            .collect_vec(),
    );

    let goal = p2!(grid.cols() - 1, grid.rows() - 1);

    let (_, min_heat) = a_star(
        &STARTS,
        |p| p.point == goal && p.consecutive >= MIN_CONSECUTIVE,
        |_, q| usize::from(grid[q.point]),
        |state| state.get_neighbors(&grid, MIN_CONSECUTIVE, MAX_CONSECUTIVE),
        |p| p.point.manhattan_dist(&goal),
    )
    .unwrap();

    min_heat
}

fn part1() -> usize {
    solve::<0, 3>()
}

fn part2() -> usize {
    solve::<4, 10>()
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
        assert_eq!(part1(), 907);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1057);
    }
}
