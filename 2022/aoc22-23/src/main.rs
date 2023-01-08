use std::collections::{HashMap, HashSet};

use hymns::counter::Counter;
use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;

const INPUT: &str = include_str!("../input.txt");

type Point = Point2<isize>;

type Grid = HashSet<Point>;

const NEIGHBORS: [[Point; 3]; 4] = [
    // NW, N, NE
    [p2!(-1, -1), p2!(0, -1), p2!(1, -1)],
    // SW, S, SE
    [p2!(-1, 1), p2!(0, 1), p2!(1, 1)],
    // NW, W, SW
    [p2!(-1, -1), p2!(-1, 0), p2!(-1, 1)],
    // NE, E, SE
    [p2!(1, -1), p2!(1, 0), p2!(1, 1)],
];

const MOVES: [Point; 4] = [
    // N
    p2!(0, -1),
    // S
    p2!(0, 1),
    // W
    p2!(-1, 0),
    // E
    p2!(1, 0),
];

fn build_grid() -> HashSet<Point> {
    let mut points = HashSet::new();
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                points.insert(p2!(x as isize, y as isize));
            }
        }
    }

    points
}

fn calc_moves(grid: &Grid, ts: usize) -> Grid {
    // let mut dead_points = HashSet::new();
    let mut old_point_to_new_point = HashMap::with_capacity(grid.len());

    // map each point to new point
    'elf: for elf in grid.iter().cloned() {
        if elf.neighbors(true, false).iter().all(|p| !grid.contains(p)) {
            // no neighbors - do nothing
            old_point_to_new_point.insert(elf, elf);
            continue;
        }

        for i in 0..4 {
            let move_idx = (ts + i) % 4;

            if NEIGHBORS[move_idx]
                .into_iter()
                .all(|delta| !grid.contains(&(elf + delta)))
            {
                // found a suitable place to move
                old_point_to_new_point.insert(elf, elf + MOVES[move_idx]);
                continue 'elf;
            }
        }

        // no place to move. stay put
        old_point_to_new_point.insert(elf, elf);
    }

    let counter = Counter::from_iter(old_point_to_new_point.values().cloned());

    old_point_to_new_point
        .into_iter()
        .map(|(old, new)| {
            if counter.get(&new).unwrap_or_default() > 1 {
                old
            } else {
                new
            }
        })
        .collect()
}

fn calc_empty(grid: &Grid) -> usize {
    let (min_x, max_x, min_y, max_y) = grid.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(min_x, max_x, min_y, max_y), p| {
            (
                min_x.min(p.x),
                max_x.max(p.x),
                min_y.min(p.y),
                max_y.max(p.y),
            )
        },
    );

    let total_squares: usize = ((max_x - min_x + 1) * (max_y - min_y + 1))
        .try_into()
        .unwrap();

    total_squares - grid.len()
}

fn part1() -> usize {
    let mut grid = build_grid();

    for round in 0..10 {
        grid = calc_moves(&grid, round);
    }

    calc_empty(&grid)
}

fn part2() -> usize {
    let mut grid = build_grid();

    for round in 0.. {
        let next_grid = calc_moves(&grid, round);

        if next_grid == grid {
            return round + 1;
        }
        grid = next_grid
    }

    unreachable!()
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
        assert_eq!(part1(), 4138);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1010);
    }
}
