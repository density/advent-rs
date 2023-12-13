use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> u64 {
    0
}

fn part2() -> u64 {
    0
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
        assert_eq!(part1(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
