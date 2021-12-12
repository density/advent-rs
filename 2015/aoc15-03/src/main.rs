use hymns::vector2::Point2;
use std::collections::HashSet;
use std::mem;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> usize {
    let mut seen = HashSet::new();
    let mut coord = Point2::new(0i64, 0i64);

    seen.insert(coord);

    for c in INPUT.chars() {
        match c {
            '^' => coord.y += 1,
            '>' => coord.x += 1,
            'v' => coord.y -= 1,
            '<' => coord.x -= 1,
            _ => unreachable!(),
        };

        seen.insert(coord);
    }

    seen.len()
}

fn part2() -> usize {
    let mut seen = HashSet::new();

    let mut coord_to_move = Point2::new(0i64, 0i64);
    let mut other_coord = Point2::new(0i64, 0i64);

    seen.insert(coord_to_move);

    for c in INPUT.chars() {
        match c {
            '^' => coord_to_move.y += 1,
            '>' => coord_to_move.x += 1,
            'v' => coord_to_move.y -= 1,
            '<' => coord_to_move.x -= 1,
            _ => unreachable!(),
        };

        seen.insert(coord_to_move);

        mem::swap(&mut coord_to_move, &mut other_coord);
    }

    seen.len()
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
        assert_eq!(part1(), 2081);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2341);
    }
}
