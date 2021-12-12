use std::collections::{HashMap, HashSet};
use std::time::Instant;

use hymns::vector2::Point2;

const INPUT: &str = include_str!("../input.txt");

fn calculate_coord(s: &str) -> Point2<i64> {
    let mut current = Point2::default();

    let mut char_iter = s.chars();

    while let Some(cur) = char_iter.next() {
        let next_move = match cur {
            'e' => Point2::new(1, 0),
            'w' => Point2::new(-1, 0),
            's' => match char_iter.next() {
                Some('e') => Point2::new(0, 1),
                Some('w') => Point2::new(-1, 1),
                _ => unreachable!(),
            },
            'n' => match char_iter.next() {
                Some('e') => Point2::new(1, -1),
                Some('w') => Point2::new(0, -1),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        current += next_move;
    }

    current
}

fn read_input() -> Vec<Point2<i64>> {
    INPUT.lines().map(|line| calculate_coord(line)).collect()
}

fn part1() -> usize {
    let coords = read_input();

    let mut blacks = HashSet::new();

    for coord in coords {
        if !blacks.insert(coord) {
            blacks.remove(&coord);
        }
    }

    blacks.len()
}

fn part2() -> usize {
    let directions: [Point2<i64>; 6] = [
        Point2::new(1, 0),
        Point2::new(-1, 0),
        Point2::new(0, 1),
        Point2::new(-1, 1),
        Point2::new(1, -1),
        Point2::new(0, -1),
    ];

    let coords = read_input();

    let mut tiles = HashMap::new();

    for coord in coords {
        match tiles.get_mut(&coord) {
            None => {
                tiles.insert(coord, true);
            }
            Some(is_black) => {
                *is_black = !*is_black;
            }
        };
    }

    for _ in 0..100 {
        let mut new_tiles = tiles.clone();

        for tile in new_tiles.keys() {
            for dir in &directions {
                let new_tile = *tile + *dir;
                tiles.entry(new_tile).or_insert(false);
            }
        }

        for (tile, is_black) in &tiles {
            let black_neighbors = directions
                .iter()
                .filter(|&&dir| {
                    let new_tile = dir + *tile;
                    *tiles.get(&new_tile).unwrap_or(&false)
                })
                .count();

            if *is_black {
                if black_neighbors == 0 || black_neighbors > 2 {
                    new_tiles.insert(*tile, false);
                }
            } else if black_neighbors == 2 {
                new_tiles.insert(*tile, true);
            }
        }

        tiles = new_tiles;
    }

    tiles.values().filter(|x| **x).count()
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
        assert_eq!(part1(), 473);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 4070);
    }
}
