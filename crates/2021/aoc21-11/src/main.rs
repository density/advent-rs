use std::collections::HashSet;
use std::iter::repeat_with;

use hymns::grid::Grid;
use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn step(grid: &mut Grid<u8>) -> usize {
    for octopus in grid.iter_values_mut() {
        *octopus += 1;
    }

    let mut flash_points = HashSet::new();

    loop {
        let mut flashed_or_affected_points = vec![];

        for (point, energy) in grid.iter_points_values() {
            if *energy > 9 && !flash_points.contains(&point) {
                flash_points.insert(point);
                flashed_or_affected_points.push(point);
                flashed_or_affected_points.extend(grid.all_neighbors(&point, true));
            }
        }

        if flashed_or_affected_points.is_empty() {
            break;
        }

        for point in flashed_or_affected_points {
            if flash_points.contains(&point) {
                let cell = &mut grid[point];

                if *cell > 9 {
                    *cell = 0;
                }
            } else {
                grid[point] += 1;
            }
        }
    }

    flash_points.len()
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

fn part1() -> usize {
    let mut grid = build_grid();

    repeat_with(|| step(&mut grid)).take(100).sum()
}

fn part2() -> usize {
    let mut grid = build_grid();

    #[allow(clippy::maybe_infinite_iter)]
    (1..)
        .find(|_| step(&mut grid) >= grid.rows() * grid.cols())
        .unwrap()
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
        assert_eq!(part1(), 1749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 285);
    }
}
