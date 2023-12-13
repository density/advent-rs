use hymns::input::parse_numbers_only;
use hymns::runner::timed_run;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn calculate_history(row: Vec<i64>) -> (i64, i64) {
    let mut result = vec![row];

    loop {
        let next_row = result
            .last()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();

        if next_row.iter().all(|n| *n == 0) {
            break;
        }

        result.push(next_row);
    }

    let mut left = 0;
    let mut right = 0;

    for row in result.iter().rev() {
        left = row[0] - left;
        right += row.last().unwrap();
    }

    (left, right)
}

fn part1() -> i64 {
    INPUT
        .lines()
        .map(|line| {
            let report = parse_numbers_only(line, true).collect_vec();
            let (_, right) = calculate_history(report);
            right
        })
        .sum()
}

fn part2() -> i64 {
    INPUT
        .lines()
        .map(|line| {
            let nums = parse_numbers_only(line, true).collect_vec();
            let (left, _) = calculate_history(nums);
            left
        })
        .sum()
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
        assert_eq!(part1(), 1798691765);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1104);
    }
}
