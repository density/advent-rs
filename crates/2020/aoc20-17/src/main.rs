use std::collections::HashSet;

use std::iter::repeat;

use itertools::Itertools;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

type Coord = Vec<isize>;

struct Simulation {
    dimensions: usize,
    state: HashSet<Coord>,
}

impl Simulation {
    fn new(dimensions: usize) -> Self {
        let state: HashSet<Coord> = (0_isize..)
            .zip(INPUT.lines())
            .flat_map(|(y, line)| {
                (0_isize..).zip(line.chars()).filter_map(move |(x, c)| {
                    let mut coord = vec![0; dimensions];

                    if c == '#' {
                        coord[0] = x;
                        coord[1] = y;
                        Some(coord)
                    } else {
                        None
                    }
                })
            })
            .collect();

        Self { dimensions, state }
    }

    fn offsets(&self) -> impl Iterator<Item = Coord> {
        repeat(-1..=1)
            .take(self.dimensions)
            .multi_cartesian_product()
            .filter(|coord| !coord.iter().all(|&n| n == 0))
    }

    fn all_neighbors<'a>(&self, coord: &'a [isize]) -> impl Iterator<Item = Coord> + 'a {
        self.offsets().filter_map(move |offset| {
            if offset.iter().all(|n| *n == 0) {
                None
            } else {
                Some(coord.iter().zip(offset).map(|(c, o)| *c + o).collect_vec())
            }
        })
    }

    fn count_active_neighbors(&self, coord: &[isize]) -> usize {
        self.all_neighbors(coord)
            .filter(|coord| self.state.contains(coord))
            .count()
    }

    fn step(&mut self) {
        let mut to_insert = vec![];
        let mut to_remove = vec![];

        for coord in &self.state {
            let mut active_neighbor_count = 0;

            for neighbor in self.all_neighbors(coord) {
                if self.state.contains(&neighbor) {
                    active_neighbor_count += 1;
                } else if self.count_active_neighbors(&neighbor) == 3 {
                    to_insert.push(neighbor);
                }
            }

            if !matches!(active_neighbor_count, 2 | 3) {
                to_remove.push(coord.clone());
            }
        }

        for coord in to_remove {
            self.state.remove(&coord);
        }
        self.state.extend(to_insert);
    }
}

fn part1() -> usize {
    let mut sim = Simulation::new(3);

    for _ in 0..6 {
        sim.step();
    }

    sim.state.len()
}

fn part2() -> usize {
    let mut sim = Simulation::new(4);

    for _ in 0..6 {
        sim.step();
    }

    sim.state.len()
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
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 269);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1380);
    }
}
