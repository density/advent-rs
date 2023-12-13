use hymns::input::parse_numbers_only;
use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn winning_hold_times(time: u64, distance: u64) -> u64 {
    let time = time as f64;
    let distance = distance as f64;

    let sqrt_discrim = (time * time - 4.0 * distance).sqrt();

    let right = (-time - sqrt_discrim) / -2.0 - 0.0001;
    let left = (-time + sqrt_discrim) / -2.0 + 0.0001;

    ((right.floor() - left.ceil()) + 1.0) as u64
}

fn part1() -> u64 {
    let mut lines = INPUT.lines();

    let times: Vec<u64> = parse_numbers_only(lines.next().unwrap(), false).collect();
    let distances: Vec<u64> = parse_numbers_only(lines.next().unwrap(), false).collect();

    times
        .into_iter()
        .zip(distances)
        .fold(1, |acc, (t, d)| acc * winning_hold_times(t, d))
}

fn part2() -> u64 {
    let mut lines = INPUT.lines().map(|line| line.replace(' ', ""));

    let time: u64 = parse_numbers_only(&lines.next().unwrap(), false)
        .next()
        .unwrap();
    let distance: u64 = parse_numbers_only(&lines.next().unwrap(), false)
        .next()
        .unwrap();

    winning_hold_times(time, distance)
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
        assert_eq!(part1(), 840336);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 41382569);
    }
}
