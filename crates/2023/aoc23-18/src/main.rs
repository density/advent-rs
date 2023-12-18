use itertools::Itertools;

use hymns::geom::area_of_polygon;
use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;

const INPUT: &str = include_str!("../input.txt");

type Point = Point2<isize>;

fn total_points(boundary_count: usize, vertices: &[Point]) -> usize {
    let area = area_of_polygon(vertices);

    // Pick's theorem: https://en.wikipedia.org/wiki/Pick's_theorem
    let internal_points = area - boundary_count / 2 + 1;

    internal_points + boundary_count
}

struct DigInstruction {
    magnitude: usize,
    delta: Point,
}

impl DigInstruction {
    fn from_part1_str(s: &str) -> Self {
        let (dir, amt) = s.split_whitespace().take(2).collect_tuple().unwrap();
        let movement: isize = amt.parse().unwrap();

        Self {
            magnitude: usize::try_from(movement).unwrap(),
            delta: match dir {
                "R" => p2!(movement, 0),
                "L" => p2!(-movement, 0),
                "U" => p2!(0, -movement),
                "D" => p2!(0, movement),
                _ => unreachable!(),
            },
        }
    }

    fn from_part2_str(s: &str) -> Self {
        let trim: &[char] = &['(', ')', '#'];

        let color = s.split_whitespace().last().unwrap().trim_matches(trim);
        let (amt, dir) = color.split_at(5);

        let movement = isize::from_str_radix(amt, 16).unwrap();

        Self {
            magnitude: usize::try_from(movement).unwrap(),
            delta: match u8::from_str_radix(dir, 8).unwrap() {
                0 => p2!(movement, 0),
                1 => p2!(0, movement),
                2 => p2!(-movement, 0),
                3 => p2!(0, -movement),
                _ => unreachable!(),
            },
        }
    }
}

fn solve(from_str_fn: fn(&str) -> DigInstruction) -> usize {
    let mut vertices = vec![Point::origin()];

    let mut boundary: usize = 0;

    for line in INPUT.lines() {
        let instruction = from_str_fn(line);
        boundary += instruction.magnitude;
        vertices.push(*vertices.last().unwrap() + instruction.delta);
    }

    total_points(boundary, &vertices)
}

fn part1() -> usize {
    solve(DigInstruction::from_part1_str)
}

fn part2() -> usize {
    solve(DigInstruction::from_part2_str)
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
        assert_eq!(part1(), 42317);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 83_605_563_360_288);
    }
}
