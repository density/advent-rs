use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

struct ReindeerInfo {
    name: String,
    rate: u32,
    max_moving_time: u32,
    rest_time: u32,
}

impl ReindeerInfo {
    fn distance_travelled(&self, time: u32) -> u32 {
        let moving_time_plus_rest_time = self.rest_time + self.max_moving_time;

        let intervals_at_top_speed = time / moving_time_plus_rest_time;

        let remaining_moving_time = (time % (intervals_at_top_speed * moving_time_plus_rest_time))
            .min(self.max_moving_time);

        self.rate * self.max_moving_time * intervals_at_top_speed
            + remaining_moving_time * self.rate
    }
}

enum ReindeerState {
    Resting(u32),
    Flying(u32),
}

struct Reindeer {
    info: ReindeerInfo,
    state: ReindeerState,
}

impl Reindeer {
    fn new(info: ReindeerInfo) -> Self {
        Self {
            state: ReindeerState::Flying(info.max_moving_time),
            info,
        }
    }

    fn tick(&mut self) -> u32 {
        match self.state {
            ReindeerState::Resting(0) => {
                self.state = ReindeerState::Flying(self.info.max_moving_time - 1);
                self.info.rate
            }
            ReindeerState::Resting(time_remaining) => {
                self.state = ReindeerState::Resting(time_remaining - 1);
                0
            }
            ReindeerState::Flying(0) => {
                self.state = ReindeerState::Resting(self.info.rest_time - 1);
                0
            }
            ReindeerState::Flying(time_remaining) => {
                self.state = ReindeerState::Flying(time_remaining - 1);
                self.info.rate
            }
        }
    }
}

fn part1() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            let words: Vec<_> = line.split_ascii_whitespace().collect();

            ReindeerInfo {
                name: words[0].to_string(),
                rate: words[3].parse().unwrap(),
                max_moving_time: words[6].parse().unwrap(),
                rest_time: words[13].parse().unwrap(),
            }
        })
        .map(|reindeer| reindeer.distance_travelled(2503))
        .max()
        .unwrap()
}

fn part2() -> u64 {
    let mut reindeers: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let words: Vec<_> = line.split_ascii_whitespace().collect();

            let info = ReindeerInfo {
                name: words[0].to_string(),
                rate: words[3].parse().unwrap(),
                max_moving_time: words[6].parse().unwrap(),
                rest_time: words[13].parse().unwrap(),
            };

            Reindeer::new(info)
        })
        .collect();

    let mut locations = HashMap::new();
    let mut points = HashMap::new();

    for _ in 0..2503 {
        let mut leader_distance = 0;
        let mut current_leaders = vec![];

        for reindeer in reindeers.iter_mut() {
            let delta = reindeer.tick();

            locations
                .entry(reindeer.info.name.clone())
                .and_modify(|d| *d += delta)
                .or_insert(delta);

            let distance = *locations.get(&reindeer.info.name).unwrap();

            match distance.cmp(&leader_distance) {
                Ordering::Less => (),
                Ordering::Equal => {
                    current_leaders.push(reindeer.info.name.clone());
                }
                Ordering::Greater => {
                    leader_distance = distance;
                    current_leaders.clear();
                    current_leaders.push(reindeer.info.name.clone());
                }
            }
        }

        for leader in current_leaders.into_iter() {
            points.entry(leader).and_modify(|p| *p += 1).or_insert(1);
        }
    }

    points
        .into_iter()
        .max_by_key(|(_, points)| *points)
        .unwrap()
        .1
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
        assert_eq!(part1(), 2640);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1102);
    }
}
