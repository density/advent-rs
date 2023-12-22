use std::collections::HashSet;

use hymns::bfs::bfs;
use itertools::Itertools;

use hymns::grid::{GPoint, Grid};
use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Direction::{Down, Left, Right, Up};
use hymns::vector2::{Direction, Point2};

use crate::Tile::{Backward, Empty, Forward, Horizontal, Vertical};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Vertical,
    Horizontal,
    Forward,
    Backward,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Beam {
    location: GPoint,
    facing: Direction,
}

impl Beam {
    fn new(location: GPoint, facing: Direction) -> Self {
        Self { location, facing }
    }
}

fn count_energized(grid: &Grid<Tile>, start_point: GPoint, start_direction: Direction) -> usize {
    let mut frontier = vec![Beam::new(start_point, start_direction)];

    let mut seen: HashSet<Beam> = HashSet::new();

    while !frontier.is_empty() {
        let mut new_beams = Vec::with_capacity(frontier.len());

        for mut beam in frontier.drain(..) {
            seen.insert(beam.clone());

            match grid[beam.location] {
                Vertical if matches!(beam.facing, Right | Left) => {
                    new_beams.push(Beam::new(beam.location, Down));
                    beam.facing = Up;
                }
                Horizontal if matches!(beam.facing, Up | Down) => {
                    new_beams.push(Beam::new(beam.location, Left));
                    beam.facing = Right;
                }
                Forward => {
                    beam.facing = match beam.facing {
                        Up => Right,
                        Down => Left,
                        Right => Up,
                        Left => Down,
                        _ => unreachable!(),
                    };
                }
                Backward => {
                    beam.facing = match beam.facing {
                        Up => Left,
                        Down => Right,
                        Right => Down,
                        Left => Up,
                        _ => unreachable!(),
                    };
                }
                _ => (),
            }

            new_beams.push(beam);
        }

        frontier.extend(new_beams.into_iter().filter_map(|beam| {
            grid.get_neighbor(&beam.location, beam.facing)
                .map(|p| Beam::new(p, beam.facing))
                .filter(|beam| !seen.contains(beam))
        }));
    }

    seen.into_iter().unique_by(|b| b.location).count()
}

fn count_energized2(grid: &Grid<Tile>, start_point: GPoint, start_direction: Direction) -> usize {
    let get_neighbors = |beam: &Beam| {
        let mut new_beams = vec![];

        let mut original_beam = beam.clone();

        match grid[beam.location] {
            Vertical if matches!(beam.facing, Right | Left) => {
                new_beams.push(Beam::new(beam.location, Down));
                original_beam.facing = Up;
            }
            Horizontal if matches!(beam.facing, Up | Down) => {
                new_beams.push(Beam::new(beam.location, Left));
                original_beam.facing = Right;
            }
            Forward => {
                original_beam.facing = match beam.facing {
                    Up => Right,
                    Down => Left,
                    Right => Up,
                    Left => Down,
                    _ => unreachable!(),
                };
            }
            Backward => {
                original_beam.facing = match beam.facing {
                    Up => Left,
                    Down => Right,
                    Right => Down,
                    Left => Up,
                    _ => unreachable!(),
                };
            }
            _ => (),
        }

        new_beams.push(original_beam);
        new_beams
    };

    bfs(Beam::new(start_point, start_direction), get_neighbors);

    todo!()
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Empty,
            '/' => Forward,
            '\\' => Backward,
            '-' => Horizontal,
            '|' => Vertical,
            _ => unreachable!(),
        }
    }
}

fn part1() -> usize {
    let grid: Grid<Tile> = INPUT.parse().unwrap();
    count_energized(&grid, p2!(0, 0), Right)
}

fn part2() -> usize {
    let grid: Grid<Tile> = INPUT.parse().unwrap();

    let mut max_cells = 0;

    for row in 0..grid.rows() {
        max_cells = max_cells
            .max(count_energized(&grid, p2!(0, row), Right))
            .max(count_energized(&grid, p2!(grid.cols() - 1, row), Left));
    }

    for col in 0..grid.cols() {
        max_cells = max_cells
            .max(count_energized(&grid, p2!(col, 0), Down))
            .max(count_energized(&grid, p2!(col, grid.rows() - 1), Up));
    }

    max_cells
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
        assert_eq!(part1(), 8021);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 8216);
    }
}
