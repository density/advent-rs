use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn simulate_snake(length: usize) -> usize {
    let mut tail_locations = HashSet::new();

    let mut snake: Vec<Point2<isize>> = vec![p2!(0, 0); length];
    tail_locations.insert(snake[length - 1]);

    for line in INPUT
        .lines()
        .map(|l| l.split_ascii_whitespace().collect::<Vec<_>>())
    {
        let dir = line[0];
        let count: isize = line[1].parse().unwrap();

        for _ in 0..count {
            let head = &mut snake[0];
            match dir {
                "R" => head.x += 1,
                "L" => head.x -= 1,
                "U" => head.y += 1,
                "D" => head.y -= 1,
                _ => unreachable!(),
            }

            for i in 1..snake.len() {
                let prev = snake[i - 1];
                let cur = &mut snake[i];

                if (prev.x - cur.x).abs() > 1 || (prev.y - cur.y).abs() > 1 {
                    cur.x += (prev.x - cur.x).signum();
                    cur.y += (prev.y - cur.y).signum();
                }
            }

            tail_locations.insert(*snake.last().unwrap());
        }
    }

    tail_locations.len()
}

fn part1() -> usize {
    simulate_snake(2)
}

fn part2() -> usize {
    simulate_snake(10)
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
        assert_eq!(part1(), 6044);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2384);
    }
}
