use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;

use hymns::math::lcm;
use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;

const INPUT: &str = include_str!("../input.txt");

type Point = Point2<usize>;

type BlizzardPoints = HashSet<Point>;
type Timestamp = usize;

impl From<&BlizzardGrid> for BlizzardPoints {
    fn from(value: &BlizzardGrid) -> Self {
        value.grid.keys().copied().collect()
    }
}

#[derive(Debug, Clone)]
struct BlizzardGrid {
    grid: HashMap<Point, HashSet<char>>,
    // max is the wall
    max_x: usize,
    max_y: usize,
}

impl BlizzardGrid {
    fn new(lines: &[&'static str]) -> Self {
        let mut grid = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if matches!(c, '^' | '<' | 'v' | '>') {
                    grid.insert(p2!(x, y), HashSet::from([c]));
                }
            }
        }

        Self {
            grid,
            max_x: lines[0].len() - 1,
            max_y: lines.len() - 1,
        }
    }

    fn empty_clone(&self) -> Self {
        Self {
            grid: HashMap::with_capacity(self.grid.len()),
            max_x: self.max_x,
            max_y: self.max_y,
        }
    }

    fn step(&self) -> Self {
        let mut new_grid = self.empty_clone();

        for x in 1..self.max_x {
            for y in 1..self.max_y {
                let p = p2!(x, y);
                if let Some(blizzards) = self.grid.get(&p) {
                    for blizzard in blizzards {
                        let mut new_point = match blizzard {
                            '<' => p2!(x - 1, y),
                            '>' => p2!(x + 1, y),
                            'v' => p2!(x, y + 1),
                            '^' => p2!(x, y - 1),
                            _ => unreachable!(),
                        };

                        if new_point.x == 0 {
                            new_point.x = self.max_x - 1;
                        } else if new_point.x == self.max_x {
                            new_point.x = 1;
                        }

                        if new_point.y == 0 {
                            new_point.y = self.max_y - 1;
                        } else if new_point.y == self.max_y {
                            new_point.y = 1;
                        }

                        new_grid
                            .grid
                            .entry(new_point)
                            .or_default()
                            .insert(*blizzard);
                    }
                }
            }
        }

        new_grid
    }

    fn build_states(lines: &[&'static str]) -> HashMap<Timestamp, BlizzardPoints> {
        let mut grid = BlizzardGrid::new(lines);

        let state_count = lcm(grid.max_x - 1, grid.max_y - 1);
        let mut states = HashMap::with_capacity(state_count);

        states.insert(0, (&grid).into());

        for ts in 1..state_count {
            grid = grid.step();
            states.insert(ts, (&grid).into());
        }

        states
    }
}

struct Valley {
    states: HashMap<Timestamp, BlizzardPoints>,
    max_x: usize,
    max_y: usize,
    start: Point,
    end: Point,
}

impl Valley {
    fn new(input: &'static str) -> Self {
        let lines: Vec<_> = input.lines().collect();

        let max_y = lines.len() - 1;

        Self {
            states: BlizzardGrid::build_states(&lines),
            max_x: lines[0].len() - 1,
            max_y,
            start: p2!(lines[0].chars().position(|c| c == '.').unwrap(), 0),
            end: p2!(lines[max_y].chars().position(|c| c == '.').unwrap(), max_y),
        }
    }

    fn moves_at_timestamp(&self, ts: Timestamp, from_location: Point) -> Vec<Point> {
        let ts = ts % self.states.len();
        let mut result = from_location.all_neighbors(false, true);

        result.retain(|p| {
            *p == self.start
                || *p == self.end
                || ((1..self.max_x).contains(&p.x)
                    && (1..self.max_y).contains(&p.y)
                    && !self.states[&ts].contains(p))
        });

        result
    }
}

fn navigate(valley: &Valley, start_ts: Timestamp) -> usize {
    let mut queue = VecDeque::new();

    let mut seen_locations = HashSet::new();
    queue.push_back((valley.start, start_ts));

    for ts in start_ts.. {
        for _ in 0..queue.len() {
            let (cur_loc, cur_ts) = queue.pop_front().unwrap();

            if cur_loc == valley.end {
                return ts - start_ts;
            }

            if !seen_locations.insert((cur_loc, cur_ts % valley.states.len())) {
                continue;
            }

            let next_ts = ts + 1;

            queue.extend(
                valley
                    .moves_at_timestamp(next_ts, cur_loc)
                    .into_iter()
                    .map(|p| (p, next_ts)),
            );
        }
    }

    unreachable!()
}

fn part1() -> usize {
    navigate(&Valley::new(INPUT), 0)
}

fn part2() -> usize {
    let mut valley = Valley::new(INPUT);

    let mut duration = navigate(&valley, 0);

    swap(&mut valley.start, &mut valley.end);
    duration += navigate(&valley, duration);
    swap(&mut valley.start, &mut valley.end);
    duration + navigate(&valley, duration)
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
        assert_eq!(part1(), 238);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 751);
    }
}
