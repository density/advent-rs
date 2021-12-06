use std::collections::VecDeque;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn simulate(steps: usize) -> usize {
    let mut fish: VecDeque<usize> = VecDeque::from([0; 9]);

    for f in INPUT
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
    {
        fish[f] += 1;
    }

    for _ in 0..steps {
        let new_fish = fish[0];

        fish.rotate_left(1);
        fish[6] += new_fish;
        fish[8] = new_fish;
    }

    fish.iter().sum()
}

fn part1() -> usize {
    simulate(80)
}

fn part2() -> usize {
    simulate(256)
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
        assert_eq!(part1(), 386536);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1732821262171);
    }
}
