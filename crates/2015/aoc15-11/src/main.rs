use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

struct Password {
    digits: Vec<u8>,
}

impl Password {
    fn new(s: &str) -> Self {
        let mut digits = Vec::with_capacity(s.len());

        for c in s.as_bytes().iter().rev() {
            digits.push(c - b'a');
        }

        Self { digits }
    }

    fn increment(&mut self) {
        let mut add = 1;

        for d in &mut self.digits {
            if *d == 25 {
                *d = 0;
                add = 1;
            } else {
                *d += add;
                add = 0;
                break;
            }
        }

        if add == 1 {
            self.digits.push(0);
        }
    }

    fn is_valid(&self) -> bool {
        let has_straight = self.digits.windows(3).any(|w| {
            // going from LSB to MSB, so check for descending rather than ascending sequence
            w[0] == w[1] + 1 && w[1] == w[2] + 1
        });

        let has_invalid_letter = self
            .digits
            .iter()
            .any(|&d| d == b'i' - b'a' || d == b'o' - b'a' || d == b'l' - b'a');

        let mut doubles = HashSet::new();

        let mut has_valid_double = false;

        for i in 0..self.digits.len() - 1 {
            if self.digits[i] == self.digits[i + 1] {
                let key = [self.digits[i], self.digits[i + 1]];

                if !doubles.is_empty() && !doubles.contains(&key) {
                    has_valid_double = true;
                    break;
                }
                doubles.insert(key);
            }
        }

        has_straight && !has_invalid_letter && has_valid_double
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for &d in self.digits.iter().rev() {
            let c = ('a'..='z').nth(d.into()).unwrap();
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

fn part1() -> String {
    let mut p = Password::new(INPUT);

    while !p.is_valid() {
        p.increment();
    }

    p.to_string()
}

fn part2() -> String {
    let mut p = Password::new(&part1());
    p.increment();

    while !p.is_valid() {
        p.increment();
    }

    p.to_string()
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
        assert_eq!(part1(), "hxbxxyzz");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "hxcaabcc");
    }
}
