use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;

const INPUT: &str = include_str!("../input.txt");

type Point = Point2<usize>;
type Rock = Vec<Point>;

const MAX_X: usize = 6;

const ROCKS: [RockType; 5] = [
    RockType::Horiz,
    RockType::Plus,
    RockType::Ell,
    RockType::Vert,
    RockType::Box,
];

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum RockType {
    Horiz,
    Plus,
    Ell,
    Vert,
    Box,
}

#[derive(Debug)]
struct Chamber {
    height: usize,
    occupied: HashSet<Point>,
    rock: Rock,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (0..=self.height + 7).rev() {
            write!(f, "|")?;
            for x in 0..=MAX_X {
                let point = p2!(x, y);
                if self.occupied.contains(&point) {
                    write!(f, "#")?;
                } else if self.rock.iter().any(|&p| p == point) {
                    write!(f, "@")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")?;

        Ok(())
    }
}

impl Chamber {
    fn new() -> Self {
        Self {
            height: 0,
            occupied: HashSet::new(),
            rock: vec![],
        }
    }

    fn make_rock(rock_type: RockType) -> Rock {
        match rock_type {
            RockType::Horiz => vec![p2!(0, 0), p2!(1, 0), p2!(2, 0), p2!(3, 0)],
            RockType::Plus => vec![p2!(1, 2), p2!(0, 1), p2!(1, 1), p2!(2, 1), p2!(1, 0)],
            RockType::Ell => vec![p2!(2, 2), p2!(2, 1), p2!(0, 0), p2!(1, 0), p2!(2, 0)],
            RockType::Vert => vec![p2!(0, 3), p2!(0, 2), p2!(0, 1), p2!(0, 0)],
            RockType::Box => vec![p2!(0, 1), p2!(1, 1), p2!(0, 0), p2!(1, 0)],
        }
    }

    fn shape_key(&self) -> [usize; 7] {
        let mut result = [0; 7];
        let mut min_y = usize::MAX;

        for (col, y_val) in result.iter_mut().enumerate() {
            for y in (0..=self.height).rev() {
                if self.occupied.contains(&p2!(col, y)) {
                    *y_val = y;
                    min_y = min_y.min(y);
                    break;
                }
            }
        }

        for y_val in &mut result {
            *y_val -= min_y;
        }

        result
    }

    fn move_rock(&mut self, dir: Direction) -> bool {
        let delta = match dir {
            Direction::Left => p2!(-1, 0),
            Direction::Right => p2!(1, 0),
            Direction::Down => p2!(0, -1),
        };

        self.shift_rock(delta)
    }

    fn lock_rock(&mut self) {
        self.occupied.extend(self.rock.iter());
        self.height = self
            .height
            .max(self.rock.iter().map(|p| p.y + 1).max().unwrap());
    }

    fn create_next_rock(&mut self, rock_type: RockType) {
        self.rock = Self::make_rock(rock_type);
        self.shift_rock(p2!(2, isize::try_from(self.height).unwrap() + 3));
    }

    fn shift_rock(&mut self, delta: Point2<isize>) -> bool {
        let mut new_points = self.rock.clone();

        for p in &mut new_points {
            match p.x.checked_add_signed(delta.x) {
                Some(x) if x <= MAX_X => p.x = x,
                _ => return false,
            };

            match p.y.checked_add_signed(delta.y) {
                Some(y) => p.y = y,
                None => return false,
            }

            if self.occupied.contains(p) {
                return false;
            }
        }

        self.rock = new_points;
        true
    }
}

fn read_jets() -> Vec<Direction> {
    let line = INPUT.lines().next().unwrap();
    line.chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        })
        .collect()
}

fn part1() -> usize {
    let jets = read_jets();

    let mut jet_idx = 0;
    let mut chamber = Chamber::new();

    for step in 0..2022 {
        chamber.create_next_rock(ROCKS[step % ROCKS.len()]);

        loop {
            chamber.move_rock(jets[jet_idx % jets.len()]);
            jet_idx += 1;

            if !chamber.move_rock(Direction::Down) {
                chamber.lock_rock();
                break;
            }
        }
    }

    chamber.height
}

fn part2() -> usize {
    let jets = read_jets();

    let mut jet_count = 0;
    let mut chamber = Chamber::new();
    let mut seen_states = HashMap::new();
    let mut step = 0;
    let mut add_height = 0;

    let max_step = 1_000_000_000_000;

    while step < max_step {
        let rock_idx = step % ROCKS.len();
        chamber.create_next_rock(ROCKS[rock_idx]);

        loop {
            let jet_idx = jet_count % jets.len();
            chamber.move_rock(jets[jet_idx]);
            jet_count += 1;

            if !chamber.move_rock(Direction::Down) {
                chamber.lock_rock();

                let key = (chamber.shape_key(), jet_idx, rock_idx);
                match seen_states.entry(key) {
                    Entry::Occupied(e) => {
                        let (prev_height, prev_step, prev_jet_count) = e.get();

                        let cycle_step_count = step - prev_step;
                        let cycles_remaining = (max_step - step) / cycle_step_count;

                        if cycles_remaining > 0 {
                            step += cycle_step_count * cycles_remaining;
                            jet_count += jet_count - prev_jet_count;
                            add_height += (chamber.height - prev_height) * cycles_remaining;
                        }
                    }
                    Entry::Vacant(e) => {
                        e.insert((chamber.height, step, jet_count));
                    }
                }

                break;
            }
        }

        step += 1;
    }

    chamber.height + add_height
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
        assert_eq!(part1(), 3184);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1577077363915);
    }
}
