use std::collections::{HashSet, VecDeque};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

type Deck = VecDeque<usize>;

fn load_decks() -> (Deck, Deck) {
    let mut lines = INPUT.lines();

    lines.next(); // skip Player 1

    let deck1: Deck = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    lines.next(); // skip Player 2

    let deck2: Deck = lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    (deck1, deck2)
}

#[derive(Debug)]
enum Player {
    Player1,
    Player2,
}

fn calculate_score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * card)
        .sum()
}

fn part1() -> usize {
    let (mut deck1, mut deck2) = load_decks();

    while !deck1.is_empty() && !deck2.is_empty() {
        let p1_card = deck1.pop_front().unwrap();
        let p2_card = deck2.pop_front().unwrap();

        if p1_card > p2_card {
            deck1.push_back(p1_card);
            deck1.push_back(p2_card);
        } else {
            deck2.push_back(p2_card);
            deck2.push_back(p1_card);
        }
    }

    if deck1.is_empty() {
        calculate_score(&deck2)
    } else {
        calculate_score(&deck1)
    }
}

struct Game {
    player1_deck: Deck,
    player2_deck: Deck,
}

impl Game {
    fn new(player1_deck: Deck, player2_deck: Deck) -> Self {
        Self {
            player1_deck,
            player2_deck,
        }
    }

    fn play_game(&mut self) -> Player {
        let mut seen_rounds = HashSet::new();

        loop {
            if !seen_rounds.insert((self.player1_deck.clone(), self.player2_deck.clone())) {
                return Player::Player1;
            }

            let p1_card = self.player1_deck.pop_front().unwrap();
            let p2_card = self.player2_deck.pop_front().unwrap();

            let p1_cards_remaining = self.player1_deck.len();
            let p2_cards_remaining = self.player2_deck.len();

            let round_winner = if p1_cards_remaining >= p1_card && p2_cards_remaining >= p2_card {
                Game::new(
                    self.player1_deck.iter().copied().take(p1_card).collect(),
                    self.player2_deck.iter().copied().take(p2_card).collect(),
                )
                .play_game()
            } else if p1_card < p2_card {
                Player::Player2
            } else {
                Player::Player1
            };

            match round_winner {
                Player::Player1 => {
                    self.player1_deck.push_back(p1_card);
                    self.player1_deck.push_back(p2_card);

                    if self.player2_deck.is_empty() {
                        return Player::Player1;
                    }
                }
                Player::Player2 => {
                    self.player2_deck.push_back(p2_card);
                    self.player2_deck.push_back(p1_card);

                    if self.player1_deck.is_empty() {
                        return Player::Player2;
                    }
                }
            };
        }
    }
}

fn part2() -> usize {
    let (deck1, deck2) = load_decks();

    let mut game = Game::new(deck1, deck2);

    match game.play_game() {
        Player::Player1 => calculate_score(&game.player1_deck),
        Player::Player2 => calculate_score(&game.player2_deck),
    }
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
        assert_eq!(part1(), 32401);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 31436);
    }
}
