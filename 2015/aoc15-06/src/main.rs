use std::time::Instant;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> usize {
    let mut lights = [[false; 1000]; 1000];

    let re = Regex::new(r"(?P<command>turn off|turn on|toggle) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)").unwrap();

    for line in INPUT.lines() {
        let caps = re.captures(line).unwrap();

        let command = &caps["command"];
        let x1: usize = caps["x1"].parse().unwrap();
        let y1: usize = caps["y1"].parse().unwrap();
        let x2: usize = caps["x2"].parse().unwrap();
        let y2: usize = caps["y2"].parse().unwrap();

        for x in x1..=x2 {
            for y in y1..=y2 {
                lights[x][y] = match command {
                    "turn on" => true,
                    "turn off" => false,
                    "toggle" => !lights[x][y],
                    _ => unreachable!(),
                }
            }
        }
    }

    lights.iter().flatten().filter(|&&light| light).count()
}

fn part2() -> u64 {
    let mut lights = vec![vec![0u64; 1000]; 1000];

    let re = Regex::new(r"(?P<command>turn off|turn on|toggle) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)").unwrap();

    for line in INPUT.lines() {
        let caps = re.captures(line).unwrap();

        let command = &caps["command"];
        let x1: usize = caps["x1"].parse().unwrap();
        let y1: usize = caps["y1"].parse().unwrap();
        let x2: usize = caps["x2"].parse().unwrap();
        let y2: usize = caps["y2"].parse().unwrap();

        for x in x1..=x2 {
            for y in y1..=y2 {
                match command {
                    "turn on" => lights[x][y] += 1,
                    "turn off" => lights[x][y] = lights[x][y].saturating_sub(1),
                    "toggle" => lights[x][y] += 2,
                    _ => unreachable!(),
                };
            }
        }
    }

    lights.iter().flatten().sum()
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
        assert_eq!(part1(), 400410);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 15343601);
    }
}
