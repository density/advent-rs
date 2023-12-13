use std::cmp::Reverse;
use std::collections::{HashSet, VecDeque};

use hymns::grid::Grid;
use hymns::runner::timed_run;
use hymns::vector2::Point2;

const INPUT: &str = include_str!("../input.txt");

type Point = Point2<usize>;

fn find_low_points(grid: &Grid<u8>) -> Vec<Point> {
    grid.iter_points_values()
        .filter_map(|(point, height)| {
            if grid
                .neighbor_coords(&point, false)
                .into_iter()
                .all(|p| grid[p] > *height)
            {
                Some(point)
            } else {
                None
            }
        })
        .collect()
}

fn trace_basin(grid: &Grid<u8>, start: &Point) -> usize {
    let mut seen = HashSet::new();

    let mut frontier = VecDeque::new();
    frontier.push_front(*start);

    while let Some(current) = frontier.pop_front() {
        seen.insert(current);

        for neighbor in grid.neighbor_coords(&current, false) {
            if grid[neighbor] != 9 && !seen.contains(&neighbor) {
                frontier.push_back(neighbor);
            }
        }
    }

    seen.len()
}

fn build_grid() -> Grid<u8> {
    let map: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    Grid::new(map)
}

fn part1() -> u32 {
    let grid = build_grid();

    find_low_points(&grid)
        .into_iter()
        .map(|p| u32::from(grid[p] + 1))
        .sum()
}

fn part2() -> usize {
    let grid = build_grid();

    let mut sizes = vec![];

    for p in find_low_points(&grid) {
        let mut seen = HashSet::new();
        seen.insert(p);

        sizes.push(trace_basin(&grid, &p));
    }

    sizes.sort_unstable_by_key(|&n| Reverse(n));

    sizes.into_iter().take(3).product()
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
        assert_eq!(part1(), 633);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1_050_192);
    }
}
