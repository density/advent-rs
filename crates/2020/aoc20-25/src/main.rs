use std::iter::from_fn;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn iterate_keys(subject: u64) -> impl Iterator<Item = u64> {
    let mut cur = 1;
    from_fn(move || {
        cur *= subject;
        cur %= 20_201_227;

        Some(cur)
    })
}

fn calc_pub_key(subject: u64, loop_size: usize) -> u64 {
    iterate_keys(subject).nth(loop_size).unwrap()
}

fn reverse_engineer_loop_size(subject: u64, pub_key: u64) -> usize {
    iterate_keys(subject).position(|n| n == pub_key).unwrap()
}

fn part1() -> u64 {
    let pub_keys: Vec<u64> = INPUT.lines().map(|line| line.parse().unwrap()).collect();

    let loop1 = reverse_engineer_loop_size(7, pub_keys[1]);

    calc_pub_key(pub_keys[0], loop1)
}

fn main() {
    let start = Instant::now();
    println!("part 1: {}", part1());
    println!("part 1 took {}ms", start.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 181800);
    }
}
