use hymns::all_equal::AllEqual;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use hymns::grid::Grid;
use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;

const INPUT: &str = include_str!("../input.txt");

type Point = Point2<usize>;

struct DijkstraQueue {
    node_queue: BinaryHeap<(Reverse<usize>, AllEqual<Point>)>,
    points_in_queue: HashSet<Point>,
}

impl DijkstraQueue {
    fn new() -> Self {
        Self {
            node_queue: BinaryHeap::new(),
            points_in_queue: HashSet::new(),
        }
    }

    fn push(&mut self, point: Point, distance: usize) {
        if self.points_in_queue.insert(point) {
            self.node_queue.push((Reverse(distance), AllEqual(point)));
        }
    }

    fn extract_min(&mut self) -> Option<Point> {
        let (_, point) = self.node_queue.pop()?;
        self.points_in_queue.remove(&point.0);
        Some(point.0)
    }
}

fn get_least_total_risk_path(grid: &Grid<u8>) -> usize {
    let mut dist: Vec<Vec<Option<usize>>> = vec![vec![None; grid.cols()]; grid.rows()];
    dist[0][0] = Some(0);

    let mut queue = DijkstraQueue::new();
    queue.push(p2!(0, 0), 0);

    while let Some(u) = queue.extract_min() {
        for v in grid.neighbor_coords(&u, false) {
            let alt = dist[u.x][u.y].unwrap_or(0) + usize::from(grid[v]);

            if alt < dist[v.x][v.y].unwrap_or(usize::MAX) {
                dist[v.x][v.y] = Some(alt);
                queue.push(v, alt);
            }
        }
    }

    dist[grid.rows() - 1][grid.cols() - 1].unwrap()
}

fn build_grid(expanded: bool) -> Grid<u8> {
    let data: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    if expanded {
        let num_rows = data[0].len();
        let num_cols = data.len();

        let mut expanded = vec![vec![0; data[0].len() * 5]; data.len() * 5];

        for (x, row) in expanded.iter_mut().enumerate() {
            for (y, elem) in row.iter_mut().enumerate() {
                let add = (x / num_rows) + (y / num_cols);

                let new_val = data[x % num_rows][y % num_cols] + u8::try_from(add).unwrap();
                *elem = if new_val > 9 { new_val % 9 } else { new_val };
            }
        }

        Grid::new(expanded)
    } else {
        Grid::new(data)
    }
}

fn part1() -> usize {
    let grid = build_grid(false);
    get_least_total_risk_path(&grid)
}

fn part2() -> usize {
    let grid = build_grid(true);
    get_least_total_risk_path(&grid)
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
        assert_eq!(part1(), 602);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2935);
    }
}
