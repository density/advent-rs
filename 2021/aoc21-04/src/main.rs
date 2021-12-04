use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

#[derive(Eq, PartialEq)]
enum BoardNumber {
    Marked(u32),
    UnMarked(u32),
}

struct Board {
    numbers: Vec<Vec<BoardNumber>>,
}

impl Board {
    fn new(lines: &str) -> Self {
        Self {
            numbers: lines
                .lines()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|n| BoardNumber::UnMarked(n.parse().unwrap()))
                        .collect()
                })
                .collect(),
        }
    }

    fn mark(&mut self, n: u32) -> bool {
        let mut marked_any = false;

        for num in self.numbers.iter_mut().flatten() {
            if *num == BoardNumber::UnMarked(n) {
                *num = BoardNumber::Marked(n);
                marked_any = true;
            }
        }

        marked_any
    }

    fn has_bingo(&self) -> bool {
        if self
            .numbers
            .iter()
            .any(|row| row.iter().all(|n| matches!(n, BoardNumber::Marked(_))))
        {
            return true;
        }

        for col in 0..self.numbers[0].len() {
            if self
                .numbers
                .iter()
                .all(|row| matches!(row[col], BoardNumber::Marked(_)))
            {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> u32 {
        self.numbers
            .iter()
            .flatten()
            .filter_map(|board_number| match board_number {
                BoardNumber::UnMarked(n) => Some(n),
                _ => None,
            })
            .sum()
    }
}

fn read_input() -> (Vec<Board>, Vec<u32>) {
    let mut segments = INPUT.split("\n\n");

    let draws: Vec<u32> = segments
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let boards: Vec<_> = segments.map(Board::new).collect();

    (boards, draws)
}

fn part1() -> u32 {
    let (mut boards, draws) = read_input();

    for draw in draws {
        for board in boards.iter_mut() {
            board.mark(draw);
            if board.has_bingo() {
                return board.sum_unmarked() * draw;
            }
        }
    }

    unreachable!()
}

fn part2() -> u32 {
    let (mut boards, draws) = read_input();

    let mut last_winner_score = 0;

    for draw in draws {
        for board in boards.iter_mut().filter(|b| !b.has_bingo()) {
            let did_mark = board.mark(draw);
            if did_mark && board.has_bingo() {
                last_winner_score = board.sum_unmarked() * draw;
            }
        }
    }

    last_winner_score
}

fn main() {
    let start = Instant::now();
    println!("part 1: {}", part1());
    println!("part 1 took {}ms", (Instant::now() - start).as_millis());

    let start = Instant::now();
    println!("part 2: {}", part2());
    println!("part 2 took {}ms", (Instant::now() - start).as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 89001);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 7296);
    }
}
