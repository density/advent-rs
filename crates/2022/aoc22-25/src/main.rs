use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Write};
use std::ops::Add;
use std::str::FromStr;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Digit {
    Zero,
    One,
    Two,
    MinusOne,
    MinusTwo,
}

impl Add for Digit {
    // carry, sum
    type Output = (Digit, Digit);

    fn add(self, rhs: Self) -> Self::Output {
        use Digit::{MinusOne, MinusTwo, One, Two, Zero};

        match (self, rhs) {
            (Zero, x) | (x, Zero) => (Zero, x),
            (One, One) => (Zero, Two),
            (One, Two) | (Two, One) => (One, MinusTwo),
            (One, MinusOne) | (MinusOne, One) | (Two, MinusTwo) | (MinusTwo, Two) => (Zero, Zero),
            (One, MinusTwo) | (MinusTwo, One) => (Zero, MinusOne),
            (Two, Two) => (One, MinusOne),
            (Two, MinusOne) | (MinusOne, Two) => (Zero, One),
            (MinusOne, MinusTwo) | (MinusTwo, MinusOne) => (MinusOne, Two),
            (MinusTwo, MinusTwo) => (MinusOne, One),
            (MinusOne, MinusOne) => (Zero, MinusTwo),
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Digit::Zero => '0',
            Digit::One => '1',
            Digit::Two => '2',
            Digit::MinusOne => '-',
            Digit::MinusTwo => '=',
        })
    }
}

impl From<char> for Digit {
    fn from(value: char) -> Self {
        match value {
            '0' => Digit::Zero,
            '1' => Digit::One,
            '2' => Digit::Two,
            '-' => Digit::MinusOne,
            '=' => Digit::MinusTwo,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Snafu(Vec<Digit>);

impl Add for Snafu {
    type Output = Snafu;

    fn add(self, rhs: Self) -> Self::Output {
        let mut digits = VecDeque::new();

        let mut carry = Digit::Zero;

        let max_len = self.0.len().max(rhs.0.len());

        let mut n1_iter = self.0.into_iter().rev();
        let mut n2_iter = rhs.0.into_iter().rev();

        for _ in 0..max_len {
            let d1 = n1_iter.next().unwrap_or(Digit::Zero);
            let d2 = n2_iter.next().unwrap_or(Digit::Zero);

            let (carry1, sum1) = d1 + carry;
            let (carry2, sum2) = sum1 + d2;

            digits.push_front(sum2);

            let (_, new_carry) = carry1 + carry2;
            carry = new_carry;
        }

        if carry != Digit::Zero {
            digits.push_front(carry);
        }

        Snafu(digits.into())
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: String = self.0.iter().map(ToString::to_string).collect();
        write!(f, "{s}")
    }
}

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snafu(s.chars().map(Into::into).collect()))
    }
}

fn part1() -> String {
    INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .reduce(|acc: Snafu, n| acc + n)
        .unwrap()
        .to_string()
}

fn part2() -> u64 {
    todo!()
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
        assert_eq!(part1(), "2-0-020-1==1021=--01");
    }

    #[test]
    fn test_part2() {}
}
