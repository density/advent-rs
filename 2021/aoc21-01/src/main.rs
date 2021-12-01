use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> usize {
    let ns: Vec<i64> = INPUT.lines().map(|line| line.parse().unwrap()).collect();

    ns.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part2() -> usize {
    let ns: Vec<u64> = INPUT.lines().map(|line| line.parse().unwrap()).collect();

    let i1 = ns.windows(3);
    let i2 = ns[1..].windows(3);

    i1.zip(i2)
        .filter(|(w1, w2)| w2.iter().sum::<u64>() > w1.iter().sum::<u64>())
        .count()
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
        assert_eq!(part1(), 1665);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1702);
    }
}
