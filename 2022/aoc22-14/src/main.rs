use std::collections::HashSet;

use itertools::Itertools;

use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;

const INPUT: &str = include_str!("../input.txt");

type Point = Point2<isize>;

fn generate_points(s: &str) -> impl Iterator<Item = Point> + '_ {
    s.split(" -> ")
        .map(|p_str| {
            let (x, y) = p_str.split(',').collect_tuple().unwrap();

            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .tuple_windows()
        .flat_map(|(p1, p2)| {
            let x_iter;
            let y_iter;

            if p1.x == p2.x {
                let start = p1.y.min(p2.y);
                let end = p1.y.max(p2.y);

                x_iter = p1.x..=p1.x;
                y_iter = start..=end;
            } else {
                let start = p1.x.min(p2.x);
                let end = p1.x.max(p2.x);

                x_iter = start..=end;
                y_iter = p1.y..=p1.y;
            }

            x_iter.cartesian_product(y_iter).map(|(x, y)| p2!(x, y))
        })
}

fn generate_grid() -> (HashSet<Point>, isize) {
    let mut max_y = 0;
    let mut grid = HashSet::new();

    for line in INPUT.lines() {
        for point in generate_points(line) {
            max_y = max_y.max(point.y);
            grid.insert(point);
        }
    }

    (grid, max_y)
}

fn part1() -> usize {
    let (mut grid, max_y) = generate_grid();

    (0..)
        .find(|_| {
            let mut sand_loc = p2!(500, 0);

            loop {
                let candidates = [
                    p2!(sand_loc.x, sand_loc.y + 1),
                    p2!(sand_loc.x - 1, sand_loc.y + 1),
                    p2!(sand_loc.x + 1, sand_loc.y + 1),
                ];

                match candidates.into_iter().find(|p| !grid.contains(p)) {
                    None => {
                        grid.insert(sand_loc);
                        return false;
                    }
                    Some(new_loc) => {
                        if new_loc.y >= max_y {
                            return true;
                        }
                        sand_loc = new_loc;
                    }
                }
            }
        })
        .unwrap()
}

fn part2() -> usize {
    let (mut grid, max_y) = generate_grid();

    (0..)
        .find(|_| {
            let mut sand_loc = p2!(500, 0);

            loop {
                if grid.contains(&p2!(500, 0)) {
                    return true;
                }

                let candidates = [
                    p2!(sand_loc.x, sand_loc.y + 1),
                    p2!(sand_loc.x - 1, sand_loc.y + 1),
                    p2!(sand_loc.x + 1, sand_loc.y + 1),
                ];

                match candidates
                    .into_iter()
                    .find(|p| p.y < max_y + 2 && !grid.contains(p))
                {
                    None => {
                        grid.insert(sand_loc);
                        return false;
                    }
                    Some(new_loc) => {
                        sand_loc = new_loc;
                    }
                }
            }
        })
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
        assert_eq!(part1(), 674);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 24958);
    }
}
