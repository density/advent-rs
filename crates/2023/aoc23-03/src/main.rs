use hymns::grid::Grid;

use hymns::runner::timed_run;

use std::collections::HashSet;
use std::iter::Peekable;
use std::ops::Range;
use std::str::Chars;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Part {
    row: usize,
    cols: Range<usize>,
    number: u64,
}

struct PartIterator<'a> {
    chars: Peekable<Chars<'a>>,
    row: usize,
    col: usize,
}

impl<'a> PartIterator<'a> {
    fn new(grid: &'a str) -> Self {
        Self {
            chars: grid.chars().peekable(),
            row: 0,
            col: 0,
        }
    }
}

impl Iterator for PartIterator<'_> {
    type Item = Part;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.chars.next() {
            if c == '\n' {
                self.row += 1;
                self.col = 0;
            } else if !c.is_ascii_digit() {
                self.col += 1;
            } else {
                let mut result = u64::from(c.to_digit(10).unwrap());

                let start = self.col;
                let mut end = start + 1;

                while let Some(c) = self.chars.next_if(char::is_ascii_digit) {
                    result *= 10;
                    result += u64::from(c.to_digit(10).unwrap());
                    end += 1;
                }

                self.col = end;

                return Some(Part {
                    row: self.row,
                    cols: start..end,
                    number: result,
                });
            }
        }

        None
    }
}

fn part1() -> u64 {
    let grid: Grid<char> = INPUT.parse().unwrap();

    let parts: Vec<Part> = PartIterator::new(INPUT).collect();

    let mut result = 0;

    let mut seen_parts = HashSet::new();

    for (point, _) in grid
        .iter_points_values()
        .filter(|(_, c)| !c.is_ascii_digit() && **c != '.')
    {
        for neigh in grid.all_neighbors(&point, true) {
            if let Some(part) = parts
                .iter()
                .find(|part| part.row == neigh.y && part.cols.contains(&neigh.x))
            {
                if !seen_parts.contains(part) {
                    seen_parts.insert(part.clone());
                    result += part.number;
                }
            }
        }
    }

    result
}

fn part2() -> u64 {
    let grid: Grid<char> = INPUT.parse().unwrap();

    let part_numbers: Vec<Part> = PartIterator::new(INPUT).collect();

    let mut result = 0;

    for (point, _) in grid.iter_points_values().filter(|(_, c)| c == &&'*') {
        let neighbor_parts: HashSet<Part> = grid
            .all_neighbors(&point, true)
            .iter()
            .filter_map(|neigh| {
                part_numbers
                    .iter()
                    .find(|part| part.row == neigh.y && part.cols.contains(&neigh.x))
                    .cloned()
            })
            .collect();

        if neighbor_parts.len() == 2 {
            result += neighbor_parts.iter().map(|p| p.number).product::<u64>();
        }
    }

    result
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
        assert_eq!(part1(), 546_563);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 91_031_374);
    }
}
