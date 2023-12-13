use std::iter::successors;
use std::time::Instant;

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn look_and_say(input: &str) -> String {
    let mut s = String::new();

    for (key, group) in &input.chars().group_by(|&c| c) {
        s.push_str(&group.count().to_string());
        s.push(key);
    }

    s
}

fn part1() -> usize {
    successors(Some(INPUT.to_string()), |s| Some(look_and_say(s)))
        .nth(40)
        .unwrap()
        .len()
}

fn part2() -> usize {
    successors(Some(INPUT.to_string()), |s| Some(look_and_say(s)))
        .nth(50)
        .unwrap()
        .len()
}

fn main() {
    let start = Instant::now();
    println!("part 1: {}", part1());
    println!("part 1 took {}ms", start.elapsed().as_millis());

    let start = Instant::now();
    println!("part 2: {}", part2());
    println!("part 2 took {}ms", start.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 492982);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 6989950);
    }
}
