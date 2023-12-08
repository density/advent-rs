use hymns::counter::Counter;
use hymns::default_map::DefaultHashMap;
use hymns::runner::timed_run;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
enum HandType {
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
    hand_type: HandType,
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
            hand_type: Self::hand_type(&cards),
            bid,
            cards,
        }
    }

    fn hand_type(cards: &[Card; 5]) -> HandType {
        let mut counts: Counter<Card> = Counter::from_iter(cards.iter().cloned());

        let joker_count = counts.remove(&Card::Joker).unwrap_or_default();

        let counts: DefaultHashMap<usize, usize> =
            Counter::from_iter(counts.counts()).into_map().into();

        if counts[&5] == 1 {
            return HandType::FiveOfAKind;
        }

        if counts[&4] == 1 {
            match joker_count {
                0 => return HandType::FourOfAKind,
                1 => return HandType::FiveOfAKind,
                _ => unreachable!(),
            }
        }

        if counts[&3] == 1 {
            match joker_count {
                0 => {
                    if counts[&2] == 1 {
                        return HandType::FullHouse;
                    }
                    return HandType::ThreeOfAKind;
                }
                1 => return HandType::FourOfAKind,
                2 => return HandType::FiveOfAKind,
                _ => unreachable!(),
            }
        }

        if counts[&2] == 2 {
            match joker_count {
                0 => return HandType::TwoPair,
                1 => return HandType::FullHouse,
                _ => unreachable!(),
            }
        }

        if counts[&2] == 1 {
            match joker_count {
                0 => return HandType::OnePair,
                1 => return HandType::ThreeOfAKind,
                2 => return HandType::FourOfAKind,
                3 => return HandType::FiveOfAKind,
                _ => unreachable!(),
            }
        }

        match joker_count {
            0 => HandType::HighCard,
            1 => HandType::OnePair,
            2 => HandType::ThreeOfAKind,
            3 => HandType::FourOfAKind,
            4 | 5 => HandType::FiveOfAKind,
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
        assert_eq!(part1(), 248217452);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 245576185);
    }
}
