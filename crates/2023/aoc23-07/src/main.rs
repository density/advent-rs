use hymns::more_itertools::{Counter, MoreItertools};
use hymns::runner::timed_run;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Hand {
    category: Category,
    cards: [Card; 5],
    bid: u64,
}

impl Hand {
    fn new(s: &str, bid: u64, use_joker: bool) -> Self {
        let cards: [Card; 5] = s
            .chars()
            .map(|c| match c {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => {
                    if use_joker {
                        Card::Joker
                    } else {
                        Card::Jack
                    }
                }
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => unreachable!(),
            })
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();

        Self {
            category: Self::category(cards),
            bid,
            cards,
        }
    }

    fn category(cards: [Card; 5]) -> Category {
        let mut counts: Counter<Card> = cards.iter().copied().collect_counter();

        let joker_count = counts.remove(&Card::Joker).unwrap_or_default();

        let counts = counts.into_values().collect_counter();

        if counts[&5] == 1 {
            return Category::FiveOfAKind;
        }

        if counts[&4] == 1 {
            match joker_count {
                0 => return Category::FourOfAKind,
                1 => return Category::FiveOfAKind,
                _ => unreachable!(),
            }
        }

        if counts[&3] == 1 {
            match joker_count {
                0 => {
                    if counts[&2] == 1 {
                        return Category::FullHouse;
                    }
                    return Category::ThreeOfAKind;
                }
                1 => return Category::FourOfAKind,
                2 => return Category::FiveOfAKind,
                _ => unreachable!(),
            }
        }

        if counts[&2] == 2 {
            match joker_count {
                0 => return Category::TwoPair,
                1 => return Category::FullHouse,
                _ => unreachable!(),
            }
        }

        if counts[&2] == 1 {
            match joker_count {
                0 => return Category::OnePair,
                1 => return Category::ThreeOfAKind,
                2 => return Category::FourOfAKind,
                3 => return Category::FiveOfAKind,
                _ => unreachable!(),
            }
        }

        match joker_count {
            0 => Category::HighCard,
            1 => Category::OnePair,
            2 => Category::ThreeOfAKind,
            3 => Category::FourOfAKind,
            4 | 5 => Category::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

fn part1() -> u64 {
    let hands: Vec<Hand> = INPUT
        .lines()
        .map(|line| {
            let (hand_str, bid) = line.split_whitespace().collect_tuple().unwrap();
            Hand::new(hand_str, bid.parse().unwrap(), false)
        })
        .sorted()
        .collect();

    (1_u64..)
        .zip(hands)
        .fold(0, |acc, (rank, hand)| acc + rank * hand.bid)
}

fn part2() -> u64 {
    let hands: Vec<Hand> = INPUT
        .lines()
        .map(|line| {
            let (hand_str, bid) = line.split_whitespace().collect_tuple().unwrap();
            Hand::new(hand_str, bid.parse().unwrap(), true)
        })
        .sorted()
        .collect();

    (1_u64..)
        .zip(hands)
        .fold(0, |acc, (rank, hand)| acc + rank * hand.bid)
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
        assert_eq!(part1(), 248_217_452);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 245_576_185);
    }
}
