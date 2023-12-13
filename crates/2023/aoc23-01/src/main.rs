use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");
const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part1() -> u64 {
    INPUT
        .lines()
        .map(|line| {
            let line = line.as_bytes();

            let tens: u64 = line
                .iter()
                .find_map(|c| {
                    if c.is_ascii_digit() {
                        Some(u64::from(c - b'0'))
                    } else {
                        None
                    }
                })
                .unwrap();

            let ones: u64 = line
                .iter()
                .rev()
                .find_map(|c| {
                    if c.is_ascii_digit() {
                        Some(u64::from(c - b'0'))
                    } else {
                        None
                    }
                })
                .unwrap();

            tens * 10 + ones
        })
        .sum()
}

fn part2() -> u64 {
    let mut result = 0;

    let numbers = (1..=9).zip(DIGITS).chain((1..=9).zip(NUMBERS));

    for line in INPUT.lines() {
        let tens = numbers
            .clone()
            .filter_map(|(value, needle)| line.find(needle).map(|idx| (idx, value)))
            .min_by_key(|(idx, _)| *idx)
            .map(|(_, value)| value)
            .unwrap();

        let ones = numbers
            .clone()
            .filter_map(|(value, needle)| line.rfind(needle).map(|idx| (idx, value)))
            .max_by_key(|(idx, _)| *idx)
            .map(|(_, value)| value)
            .unwrap();

        result += tens * 10 + ones;
    }

    result
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
        assert_eq!(part1(), 55208);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 54578);
    }
}
