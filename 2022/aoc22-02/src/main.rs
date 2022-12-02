use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

#[repr(u64)]
#[derive(Copy, Clone, Eq, PartialEq)]
enum Implement {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<u8> for Implement {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' | b'X' => Ok(Self::Rock),
            b'B' | b'Y' => Ok(Self::Paper),
            b'C' | b'Z' => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

#[repr(u64)]
#[derive(Copy, Clone)]
enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl TryFrom<u8> for Outcome {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'X' => Ok(Self::Loss),
            b'Y' => Ok(Self::Draw),
            b'Z' => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

fn part1() -> u64 {
    let mut score = 0;

    for line in INPUT.lines().map(|l| l.as_bytes()) {
        let theirs: Implement = line[0].try_into().unwrap();
        let mine: Implement = line[2].try_into().unwrap();

        let outcome = match (theirs, mine) {
            (Implement::Rock, Implement::Paper)
            | (Implement::Paper, Implement::Scissors)
            | (Implement::Scissors, Implement::Rock) => Outcome::Win,
            (x, y) if x == y => Outcome::Draw,
            _ => Outcome::Loss,
        };

        score += outcome as u64 + mine as u64;
    }

    score
}

fn part2() -> u64 {
    let mut score = 0;

    for line in INPUT.lines().map(|l| l.as_bytes()) {
        let theirs: Implement = line[0].try_into().unwrap();
        let outcome: Outcome = line[2].try_into().unwrap();

        let mine = match (theirs, outcome) {
            (_, Outcome::Draw) => theirs,
            (Implement::Rock, Outcome::Win) => Implement::Paper,
            (Implement::Rock, Outcome::Loss) => Implement::Scissors,

            (Implement::Paper, Outcome::Win) => Implement::Scissors,
            (Implement::Paper, Outcome::Loss) => Implement::Rock,

            (Implement::Scissors, Outcome::Win) => Implement::Rock,
            (Implement::Scissors, Outcome::Loss) => Implement::Paper,
        };

        score += mine as u64 + outcome as u64;
    }

    score
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
        assert_eq!(part1(), 12458);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 12683);
    }
}
