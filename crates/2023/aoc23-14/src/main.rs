use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use itertools::Itertools;

use hymns::grid::Grid;
use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;

use crate::Direction::{East, North, South, West};
use crate::Surface::{Cube, Empty, Rounded};

const INPUT: &str = include_str!("../input.txt");

type Point = Point2<usize>;
type Dish = Grid<Surface>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Surface {
    Rounded,
    Cube,
    Empty,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Display for Surface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Rounded => 'O',
            Cube => '#',
            Empty => '.',
        };
        write!(f, "{c}")
    }
}

impl From<char> for Surface {
    fn from(value: char) -> Self {
        match value {
            'O' => Rounded,
            '#' => Cube,
            '.' => Empty,
            _ => unreachable!(),
        }
    }
}

fn move_point<'a>(
    direction: Direction,
    grid: &'a Dish,
    point: &'a Point,
) -> Box<dyn Iterator<Item = Point> + 'a> {
    match direction {
        North => Box::new((0..point.y).rev().map(|row| p2!(point.x, row))),
        South => Box::new(((point.y + 1)..grid.rows()).map(|row| p2!(point.x, row))),
        East => Box::new((point.x + 1..grid.cols()).map(|col| p2!(col, point.y))),
        West => Box::new((0..point.x).rev().map(|col| p2!(col, point.y))),
    }
}

fn iter_rows(direction: Direction, grid: &Dish) -> Box<dyn Iterator<Item = usize>> {
    if matches!(direction, North | West) {
        Box::new(0..grid.rows())
    } else {
        Box::new((0..grid.rows()).rev())
    }
}
fn iter_cols(direction: Direction, grid: &Dish) -> Box<dyn Iterator<Item = usize>> {
    if direction == East {
        Box::new((0..grid.cols()).rev())
    } else {
        Box::new(0..grid.cols())
    }
}

fn move_rocks(direction: Direction, grid: &mut Dish) {
    for row in iter_rows(direction, grid) {
        for col in iter_cols(direction, grid) {
            let old_point = p2!(col, row);

            if grid[old_point] != Rounded {
                continue;
            }

            let mut final_point = None;

            for moved in move_point(direction, grid, &old_point) {
                if grid[moved] == Empty {
                    final_point = Some(moved);
                } else {
                    break;
                }
            }

            if let Some(final_point) = final_point {
                grid.set_value(&old_point, Empty);
                grid.set_value(&final_point, Rounded);
            }
        }
    }
}

fn calculate_weight(grid: &Dish) -> usize {
    grid.iter_rows()
        .enumerate()
        .map(|(i, row)| {
            let distance = grid.rows() - i;
            row.iter().filter(|row| **row == Rounded).count() * distance
        })
        .sum()
}

fn part1() -> usize {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().map(Surface::from).collect_vec())
        .collect_vec();
    let mut grid = Grid::new(grid);

    move_rocks(North, &mut grid);

    calculate_weight(&grid)
}

fn part2() -> usize {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().map(Surface::from).collect_vec())
        .collect_vec();
    let mut grid = Grid::new(grid);

    let iterations = 1_000_000_000;
    let mut cycle = 0;
    let mut first_seen = HashMap::new();

    while cycle != iterations {
        if first_seen.contains_key(&grid) {
            let cycle_length = cycle - first_seen[&grid];
            cycle += ((iterations - 1 - cycle) / cycle_length) * cycle_length;
        }
        first_seen.insert(grid.clone(), cycle);

        for dir in [North, West, South, East] {
            move_rocks(dir, &mut grid);
        }

        cycle += 1;
    }

    calculate_weight(&grid)
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
        assert_eq!(part1(), 108_614);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 96447);
    }
}
