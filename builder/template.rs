use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> u64 {
    todo!()
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
        assert_eq!(part1(), todo!());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), todo!());
    }
}
