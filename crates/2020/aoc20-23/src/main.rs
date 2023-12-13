use std::convert::TryInto;
use std::time::Instant;

const INPUT: &str = "368195742";

fn read_input() -> impl Iterator<Item = usize> + Clone {
    INPUT
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
}

fn build_cups(input_nums: impl Iterator<Item = usize>, count: usize) -> (Vec<usize>, usize) {
    let mut input_nums = input_nums.peekable();

    let first_cup = *input_nums.peek().unwrap();

    let mut cups = vec![0; count + 1];

    while let Some(cur) = input_nums.next() {
        if let Some(next) = input_nums.peek() {
            cups[cur] = *next;
        } else {
            cups[cur] = first_cup;
        }
    }

    (cups, first_cup)
}

fn successors(cups: &[usize], start: usize) -> [usize; 3] {
    let mut results = [0; 3];

    let mut current = cups[start];

    for result in &mut results {
        *result = current;
        current = cups[current];
    }

    results
}

fn run_game(cups: &mut [usize], mut current: usize, turns: usize) {
    let max_cup = *cups.iter().skip(1).max().unwrap();
    let min_cup = *cups.iter().skip(1).min().unwrap();

    for _ in 0..turns {
        let pick_up = successors(cups, current);

        let mut destination = current - 1;
        loop {
            while pick_up.contains(&destination) {
                destination -= 1;
            }

            if destination < min_cup {
                destination = max_cup;
            } else {
                break;
            }
        }

        cups[current] = cups[pick_up[2]];
        current = cups[pick_up[2]];

        let tmp = cups[destination];
        cups[destination] = pick_up[0];
        cups[pick_up[2]] = tmp;
    }
}

fn part1() -> String {
    let input_iter = read_input();

    let (mut cups, current) = build_cups(input_iter, INPUT.len());

    run_game(&mut cups, current, 100);

    cup_string(&cups)
}

fn part2() -> usize {
    let input_iter = read_input();

    let max = input_iter.clone().max().unwrap();

    let input_iter = input_iter.chain((max + 1)..=1_000_000);

    let (mut cups, current) = build_cups(input_iter, 1_000_000);

    run_game(&mut cups, current, 10_000_000);

    cups[1] * cups[cups[1]]
}

fn cup_string(cups: &[usize]) -> String {
    let mut s = String::new();

    let mut cur_cup = cups[1];

    while s.len() < cups.len() - 2 {
        s.push_str(&cur_cup.to_string());
        cur_cup = cups[cur_cup];
    }

    s
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
        assert_eq!(part1(), "95648732");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 192515314252);
    }
}
