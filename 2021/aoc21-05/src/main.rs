use std::collections::HashMap;
use std::iter::empty;
use std::time::Instant;

use hymns::vector2::Vector2;

const INPUT: &str = include_str!("../input.txt");

fn make_point(coord: &str) -> Vector2<i32> {
    let mut coords = coord.split(',').map(|c| c.parse().unwrap());
    let x: i32 = coords.next().unwrap();
    let y: i32 = coords.next().unwrap();

    Vector2::new(x, y)
}

fn generate_points_on_line_segment(
    start: &Vector2<i32>,
    end: &Vector2<i32>,
    include_diagonal: bool,
) -> Box<dyn Iterator<Item = Vector2<i32>>> {
    if start.x == end.x {
        let min_y = start.y.min(end.y);
        let max_y = start.y.max(end.y);
        let x = start.x;

        Box::new((min_y..=max_y).map(move |y| Vector2::new(x, y)))
    } else {
        let m = (end.y - start.y) / (end.x - start.x);

        if m.abs() == 1 && !include_diagonal {
            return Box::new(empty());
        }

        let xrange = if start.x < end.x {
            start.x..=end.x
        } else {
            end.x..=start.x
        };

        let b = start.y - m * start.x;

        Box::new(xrange.map(move |x| Vector2::new(x, m * x + b)))
    }
}

fn evaluate_lines(include_diagonal: bool) -> usize {
    let mut points = HashMap::new();

    for line in INPUT.lines() {
        let mut pairs = line.split(" -> ");

        let start = make_point(pairs.next().unwrap());
        let end = make_point(pairs.next().unwrap());

        for point in generate_points_on_line_segment(&start, &end, include_diagonal) {
            points
                .entry(point)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    points.into_iter().filter(|(_, count)| *count >= 2).count()
}

fn part1() -> usize {
    evaluate_lines(false)
}

fn part2() -> usize {
    evaluate_lines(true)
}

fn main() {
    let start = Instant::now();
    println!("part 1: {}", part1());
    println!("part 1 took {}ms", (Instant::now() - start).as_millis());

    let start = Instant::now();
    println!("part 2: {}", part2());
    println!("part 2 took {}ms", (Instant::now() - start).as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 7380);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 21373);
    }
}
