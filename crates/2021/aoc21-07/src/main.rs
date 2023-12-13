use hymns::input::parse_char_delimited_numbers;
use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> i32 {
    let crabs: Vec<i32> = parse_char_delimited_numbers(INPUT, ',').collect();

    let (min, max) = crabs
        .iter()
        .fold((i32::MAX, i32::MIN), |(cur_min, cur_max), &crab| {
            (cur_min.min(crab), cur_max.max(crab))
        });

    (min..=max).fold(i32::MAX, |min_cost, distance| {
        min_cost.min(crabs.iter().map(|c| (c - distance).abs()).sum())
    })
}

fn part2() -> i32 {
    let crabs: Vec<i32> = parse_char_delimited_numbers(INPUT, ',').collect();

    let (min, max) = crabs
        .iter()
        .fold((i32::MAX, i32::MIN), |(cur_min, cur_max), &crab| {
            (cur_min.min(crab), cur_max.max(crab))
        });

    (min..=max).fold(i32::MAX, |min_cost, distance| {
        min_cost.min(
            crabs
                .iter()
                .map(|c| {
                    let steps = (c - distance).abs();

                    // Formula for sum of first n integers
                    steps * (steps + 1) / 2
                })
                .sum(),
        )
    })
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
        assert_eq!(part1(), 349_769);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 99_540_554);
    }
}
