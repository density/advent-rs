use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::{Point2, Rotation};
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> isize {
    let mut dir: Point2<isize> = p2!(0, 1);

    let mut location = Point2::default();

    for movement in INPUT.split(", ") {
        let (turn, count) = movement.split_at(1);

        let count: isize = count.parse().unwrap();

        match turn {
            "L" => dir.rotate(Rotation::Left90),
            "R" => dir.rotate(Rotation::Right90),
            _ => unreachable!(),
        }

        location += dir * count;
    }

    location.manhattan_dist(&Point2::default())
}

fn part2() -> isize {
    let mut dir: Point2<isize> = p2!(0, 1);

    let mut seen = HashSet::new();

    let mut location = Point2::default();

    for movement in INPUT.split(", ") {
        let (turn, count) = movement.split_at(1);

        let count: isize = count.parse().unwrap();

        match turn {
            "L" => dir.rotate(Rotation::Left90),
            "R" => dir.rotate(Rotation::Right90),
            _ => unreachable!(),
        }

        for _ in 0..count {
            location += dir;

            if !seen.insert(location) {
                return location.manhattan_dist(&Point2::default());
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
        assert_eq!(part1(), 271);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 153);
    }
}
