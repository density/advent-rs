use std::time::Instant;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn read_input() -> impl Iterator<Item = (String, usize, usize, usize, usize)> {
    let re = Regex::new(r"(?P<command>turn off|turn on|toggle) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)").unwrap();

    INPUT.lines().map(move |line| {
        let caps = re.captures(line).unwrap();

        let command = caps["command"].into();
        let x1: usize = caps["x1"].parse().unwrap();
        let y1: usize = caps["y1"].parse().unwrap();
        let x2: usize = caps["x2"].parse().unwrap();
        let y2: usize = caps["y2"].parse().unwrap();

        (command, x1, y1, x2, y2)
    })
}

fn part1() -> usize {
    let mut lights = vec![vec![false; 1000]; 1000];

    for (command, x1, y1, x2, y2) in read_input() {
        for row in lights.iter_mut().take(x2 + 1).skip(x1) {
            for light in row.iter_mut().take(y2 + 1).skip(y1) {
                *light = match command.as_ref() {
                    "turn on" => true,
                    "turn off" => false,
                    "toggle" => !*light,
                    _ => unreachable!(),
                }
            }
        }
    }

    lights.iter().flatten().filter(|&&light| light).count()
}

fn part2() -> u64 {
    let mut lights = vec![vec![0u64; 1000]; 1000];

    for (command, x1, y1, x2, y2) in read_input() {
        for light_row in lights.iter_mut().take(x2 + 1).skip(x1) {
            for light in light_row.iter_mut().take(y2 + 1).skip(y1) {
                match command.as_ref() {
                    "turn on" => *light += 1,
                    "turn off" => *light = light.saturating_sub(1),
                    "toggle" => *light += 2,
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
