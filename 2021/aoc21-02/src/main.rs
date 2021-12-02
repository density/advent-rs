use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> i32 {
    let mut horiz = 0;
    let mut depth = 0;

    for line in INPUT.lines() {
        let mut words = line.split_ascii_whitespace();

        let action = words.next().unwrap();
        let magnitude: i32 = words.next().unwrap().parse().unwrap();

        match action {
            "forward" => horiz += magnitude,
            "down" => depth += magnitude,
            "up" => depth -= magnitude,
            _ => unreachable!(),
        }
    }

    horiz * depth
}

fn part2() -> i32 {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in INPUT.lines() {
        let mut words = line.split_ascii_whitespace();

        let action = words.next().unwrap();
        let magnitude: i32 = words.next().unwrap().parse().unwrap();

        match action {
            "forward" => {
                horiz += magnitude;
                depth += aim * magnitude;
            }
            "down" => aim += magnitude,
            "up" => aim -= magnitude,
            _ => unreachable!(),
        }
    }

    horiz * depth
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
        assert_eq!(part1(), 1488669);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1176514794);
    }
}
