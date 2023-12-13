use std::ops::ControlFlow;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> i64 {
    INPUT.chars().fold(0, |acc, c| match c {
        ')' => acc - 1,
        '(' => acc + 1,
        _ => unreachable!(),
    })
}

fn part2() -> usize {
    let result: ControlFlow<usize, i64> = INPUT.chars().enumerate().try_fold(0, |acc, (pos, c)| {
        let new_floor = match c {
            ')' => acc - 1,
            '(' => acc + 1,
            _ => unreachable!(),
        };

        if new_floor == -1 {
            ControlFlow::Break(pos)
        } else {
            ControlFlow::Continue(new_floor)
        }
    });

    match result {
        ControlFlow::Break(pos) => pos + 1,
        ControlFlow::Continue(_) => unreachable!(),
    }
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
        assert_eq!(part1(), 232);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1783);
    }
}
