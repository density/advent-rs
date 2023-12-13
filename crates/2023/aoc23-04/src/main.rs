use hymns::runner::timed_run;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../input.txt");

struct Card {
    win_count: usize,
}

impl Card {
    fn new(line: &str) -> Self {
        let start = line.find(':').unwrap() + 1;
        let (winning, hand) = line[start..].split_once(" | ").unwrap();

        let winning: Vec<u64> = winning
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let win_count = hand
            .split_whitespace()
            .filter(|n| {
                let n: u64 = n.parse().unwrap();
                winning.contains(&n)
            })
            .count();

        Self { win_count }
    }
}

fn build_cards() -> Vec<Card> {
    INPUT.lines().map(Card::new).collect()
}

fn part1() -> usize {
    build_cards()
        .iter()
        .map(|c| {
            if c.win_count == 0 {
                0
            } else {
                2_usize.pow(c.win_count.saturating_sub(1).try_into().unwrap())
            }
        })
        .sum()
}

fn part2() -> usize {
    let cards = build_cards();

    let mut to_process = (0..cards.len()).collect::<VecDeque<_>>();

    let mut card_count = cards.len();

    while let Some(card) = to_process.pop_front() {
        let win_count = cards[card].win_count;

        let new_cards = (card + 1)..(cards.len().min(card + win_count + 1));
        card_count += new_cards.len();
        to_process.extend(new_cards);
    }

    card_count
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
        assert_eq!(part1(), 28538);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 9425061);
    }
}
