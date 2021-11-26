use std::time::Instant;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

#[derive(Copy, Clone, Eq, PartialEq)]
enum Light {
    On,
    Off,
}

impl Light {
    fn turn_on(&mut self) {
        *self = Light::On;
    }

    fn turn_off(&mut self) {
        *self = Light::Off;
    }

    fn toggle(&mut self) {
        *self = match self {
            Light::On => Light::Off,
            Light::Off => Light::On,
        }
    }
}

fn part1() -> usize {
    let mut lights = [[Light::Off; 1000]; 1000];

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
                let light = &mut lights[x][y];

                match command {
                    "turn on" => light.turn_on(),
                    "turn off" => light.turn_off(),
                    "toggle" => light.toggle(),
                    _ => unreachable!(),
                }
            }
        }
    }

    lights
        .iter()
        .flatten()
        .filter(|&&light| light == Light::On)
        .count()
}

fn part2() -> u64 {
    todo!()
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
        assert_eq!(part2(), todo!());
    }
}
