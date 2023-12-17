use std::collections::HashSet;
use std::mem::swap;

use itertools::Itertools;

use hymns::grid::{GPoint, Grid};
use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::{Direction, Point2};

use self::Direction::{Down, Left, Right, Up};
use self::Tile::{DownLeft, DownRight, Ground, RightLeft, UpDown, UpLeft, UpRight};

const INPUT: &str = include_str!("../input.txt");

type Scan = Grid<Tile>;

#[derive(Debug, Copy, Clone)]
enum Tile {
    UpDown,
    RightLeft,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Ground,
}

impl Tile {
    fn is_accessible_from(self, direction: Direction) -> bool {
        match self {
            UpDown => direction == Up || direction == Down,
            RightLeft => direction == Right || direction == Left,
            UpRight => direction == Up || direction == Right,
            UpLeft => direction == Up || direction == Left,
            DownRight => direction == Down || direction == Right,
            DownLeft => direction == Down || direction == Left,
            Ground => false,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => UpDown,
            '-' => RightLeft,
            'L' => UpRight,
            'J' => UpLeft,
            '7' => DownLeft,
            'F' => DownRight,
            '.' => Ground,
            _ => unreachable!(),
        }
    }
}

fn neighbors_and_origin_dirs(point: &GPoint) -> Vec<(Direction, GPoint)> {
    let x = point.x;
    let y = point.y;

    vec![
        (Down, p2!(x, y.wrapping_sub(1))),
        (Up, p2!(x, y + 1)),
        (Left, p2!(x + 1, y)),
        (Right, p2!(x.wrapping_sub(1), y)),
    ]
}

fn accessible_neighbors(scan: &Scan, point: &GPoint) -> Vec<GPoint> {
    let movements = [
        (Down, p2!(point.x, point.y.wrapping_sub(1))),
        (Up, p2!(point.x, point.y + 1)),
        (Left, p2!(point.x + 1, point.y)),
        (Right, p2!(point.x.wrapping_sub(1), point.y)),
    ];

    movements
        .into_iter()
        .filter_map(|(origin_dir, neighbor)| {
            scan.get_value(&neighbor).and_then(|tile| {
                if tile.is_accessible_from(origin_dir)
                    && scan[*point].is_accessible_from(origin_dir.inverted())
                {
                    Some(neighbor)
                } else {
                    None
                }
            })
        })
        .collect_vec()
}

fn find_pipe(start: &GPoint, grid: &Scan) -> (usize, HashSet<GPoint>) {
    let mut seen = HashSet::new();
    let mut distance = 0;
    let mut frontier = vec![*start];
    let mut next_points = vec![];

    while !frontier.is_empty() {
        distance += 1;

        for point in frontier.drain(..) {
            seen.insert(point);

            next_points.extend(
                accessible_neighbors(grid, &point)
                    .into_iter()
                    .filter(|neighbor| !seen.contains(neighbor)),
            );
        }

        swap(&mut frontier, &mut next_points);
    }

    (distance - 1, seen)
}

fn load_grid() -> (GPoint, Scan) {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut tiles = vec![vec![]; grid.len()];

    let mut start = None;

    for (y, row) in grid.iter().enumerate() {
        for x in 0..row.len() {
            if grid[y][x] == 'S' {
                start = Some(p2!(x, y));

                let (dir1, dir2) = neighbors_and_origin_dirs(&p2!(x, y))
                    .iter()
                    .filter_map(|(dir, point)| {
                        if let Some(c) = grid.get(point.y).and_then(|row| row.get(point.x)) {
                            if Tile::from(*c).is_accessible_from(*dir) {
                                return Some(*dir);
                            }
                        }

                        None
                    })
                    .collect_tuple()
                    .unwrap();

                tiles[y].push(match (dir1, dir2) {
                    (Up, Down) | (Down, Up) => UpDown,
                    (Right, Left) | (Left, Right) => RightLeft,
                    (Up, Right) | (Right, Up) => UpRight,
                    (Up, Left) | (Left, Up) => UpLeft,
                    (Down, Right) | (Right, Down) => DownRight,
                    (Down, Left) | (Left, Down) => DownLeft,
                    _ => unreachable!(),
                });
            } else {
                tiles[y].push(Tile::from(grid[y][x]));
            }
        }
    }

    (start.unwrap(), Grid::new(tiles))
}

fn count_diagonal(start: &GPoint, grid: &Scan, pipe: &HashSet<GPoint>) -> usize {
    let mut count = 0;
    let mut inside = false;

    for (row, col) in (start.y..grid.rows()).zip(start.x..grid.cols()) {
        let point = p2!(col, row);

        if pipe.contains(&point) {
            if !matches!(grid[point], UpRight | DownLeft) {
                inside = !inside;
            }
        } else if inside {
            count += 1;
        }
    }

    count
}
fn part1() -> usize {
    let (start, grid) = load_grid();

    find_pipe(&start, &grid).0
}

fn part2() -> usize {
    let (start, grid) = load_grid();
    let pipe = find_pipe(&start, &grid).1;

    let mut count = 0;

    for col in 0..grid.cols() {
        count += count_diagonal(&p2!(col, 0), &grid, &pipe);
    }

    for row in 1..grid.rows() {
        count += count_diagonal(&p2!(0, row), &grid, &pipe);
    }

    count
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
        assert_eq!(part1(), 7107);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 281);
    }
}
