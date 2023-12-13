use std::collections::{HashSet, VecDeque};

use hymns::grid::Grid;
use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;

const INPUT: &str = include_str!("../input.txt");

fn build_grid(start: &mut Point2<usize>, end: &mut Point2<usize>) -> Grid<u8> {
    Grid::new(
        INPUT
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(x, &elev)| match elev {
                        b'S' => {
                            *start = p2!(x, y);
                            b'a'
                        }
                        b'E' => {
                            *end = p2!(x, y);
                            b'z'
                        }
                        _ => elev,
                    })
                    .collect()
            })
            .collect(),
    )
}

fn part1() -> u64 {
    let mut start = Point2::default();
    let mut end = Point2::default();

    let grid = build_grid(&mut start, &mut end);

    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    let mut seen = HashSet::new();

    while let Some((distance, point)) = queue.pop_front() {
        if point == end {
            return distance;
        }

        if !seen.insert(point) {
            continue;
        }

        for neighbor in grid.neighbor_coords(&point, false) {
            if grid[neighbor] <= grid[point] + 1 {
                queue.push_back((distance + 1, neighbor));
            }
        }
    }

    unreachable!()
}

fn part2() -> u64 {
    let mut start = Point2::default();

    let grid = build_grid(&mut Point2::default(), &mut start);

    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    let mut seen = HashSet::new();

    while let Some((distance, point)) = queue.pop_front() {
        if grid[point] == b'a' {
            return distance;
        }

        if !seen.insert(point) {
            continue;
        }

        for neighbor in grid.neighbor_coords(&point, false) {
            if grid[point] <= grid[neighbor] + 1 {
                queue.push_back((distance + 1, neighbor));
            }
        }
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
        assert_eq!(part1(), 383);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 377);
    }
}
