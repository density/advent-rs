use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::iter::IntoIterator;

use regex::Regex;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

const ALL_RESOURCES: [Resource; 4] = [
    Resource::Ore,
    Resource::Clay,
    Resource::Obsidian,
    Resource::Geode,
];

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
struct Robot(Resource);

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Blueprint {
    id: u64,
    ore_robot_ore: u64,

    clay_robot_ore: u64,

    obsidian_robot_ore: u64,
    obsidian_robot_clay: u64,

    geode_robot_ore: u64,
    geode_robot_obsidian: u64,
}

impl Blueprint {
    fn usable_per_turn(&self, resource: Resource) -> u64 {
        match resource {
            Resource::Ore => [
                self.ore_robot_ore,
                self.clay_robot_ore,
                self.obsidian_robot_ore,
                self.geode_robot_ore,
            ]
            .into_iter()
            .max()
            .unwrap(),
            Resource::Clay => self.obsidian_robot_clay,
            Resource::Obsidian => self.geode_robot_obsidian,
            Resource::Geode => u64::MAX,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    robots: HashMap<Robot, u64>,
    bank: HashMap<Resource, u64>,
}

#[allow(clippy::derived_hash_with_manual_eq)]
impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for resource in ALL_RESOURCES {
            (
                resource,
                self.robots.get(&Robot(resource)).unwrap_or(&0),
                self.bank[&resource],
            )
                .hash(state);
        }
    }
}

impl State {
    fn new() -> Self {
        let robots = [
            (Robot(Resource::Ore), 1),
            (Robot(Resource::Clay), 0),
            (Robot(Resource::Obsidian), 0),
        ]
        .into_iter()
        .collect();

        let bank = [
            ((Resource::Ore), 0),
            ((Resource::Clay), 0),
            ((Resource::Obsidian), 0),
            ((Resource::Geode), 0),
        ]
        .into_iter()
        .collect();

        Self { robots, bank }
    }

    fn can_build(&self, blueprint: &Blueprint, robot: Robot) -> bool {
        if self.robots.get(&robot).unwrap_or(&0) >= &blueprint.usable_per_turn(robot.0) {
            return false;
        }

        match robot {
            Robot(Resource::Ore) => self.bank[&Resource::Ore] >= blueprint.ore_robot_ore,
            Robot(Resource::Clay) => self.bank[&Resource::Ore] >= blueprint.clay_robot_ore,
            Robot(Resource::Obsidian) => {
                self.bank[&Resource::Ore] >= blueprint.obsidian_robot_ore
                    && self.bank[&Resource::Clay] >= blueprint.obsidian_robot_clay
            }
            Robot(Resource::Geode) => {
                self.bank[&Resource::Ore] >= blueprint.geode_robot_ore
                    && self.bank[&Resource::Obsidian] >= blueprint.geode_robot_obsidian
            }
        }
    }

    fn build(&mut self, blueprint: &Blueprint, robot: Robot) {
        match robot {
            Robot(Resource::Ore) => {
                self.bank
                    .entry(Resource::Ore)
                    .and_modify(|n| *n -= blueprint.ore_robot_ore);
            }
            Robot(Resource::Clay) => {
                self.bank
                    .entry(Resource::Ore)
                    .and_modify(|n| *n -= blueprint.clay_robot_ore);
            }
            Robot(Resource::Obsidian) => {
                self.bank
                    .entry(Resource::Ore)
                    .and_modify(|n| *n -= blueprint.obsidian_robot_ore);
                self.bank
                    .entry(Resource::Clay)
                    .and_modify(|n| *n -= blueprint.obsidian_robot_clay);
            }
            Robot(Resource::Geode) => {
                self.bank
                    .entry(Resource::Ore)
                    .and_modify(|n| *n -= blueprint.geode_robot_ore);
                self.bank
                    .entry(Resource::Obsidian)
                    .and_modify(|n| *n -= blueprint.geode_robot_obsidian);
            }
        }

        self.robots.entry(robot).and_modify(|n| *n += 1);
    }

    fn ticked(&self) -> Self {
        let mut new_state = self.clone();

        for (Robot(resource), count) in &self.robots {
            new_state.bank.entry(*resource).and_modify(|n| *n += count);
        }

        new_state
    }
}

fn load_blueprints() -> Vec<Blueprint> {
    let re = Regex::new(r"Blueprint (?P<id>\d+): Each ore robot costs (?P<ore_ore>\d+) ore. Each clay robot costs (?P<clay_ore>\d+) ore. Each obsidian robot costs (?P<obsidian_ore>\d+) ore and (?P<obsidian_clay>\d+) clay. Each geode robot costs (?P<geode_ore>\d+) ore and (?P<geode_obsidian>\d+) obsidian.").unwrap();

    INPUT
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            Blueprint {
                id: caps["id"].parse().unwrap(),
                ore_robot_ore: caps["ore_ore"].parse().unwrap(),

                clay_robot_ore: caps["clay_ore"].parse().unwrap(),

                obsidian_robot_ore: caps["obsidian_ore"].parse().unwrap(),
                obsidian_robot_clay: caps["obsidian_clay"].parse().unwrap(),

                geode_robot_ore: caps["geode_ore"].parse().unwrap(),
                geode_robot_obsidian: caps["geode_obsidian"].parse().unwrap(),
            }
        })
        .collect()
}

fn run_simulation(step_count: u64, blueprints: &[Blueprint]) -> Vec<(u64, u64)> {
    let mut result = vec![];

    for blueprint in blueprints {
        let mut max_geode_count = 0;
        let mut seen_states = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back(State::new());
        seen_states.insert(State::new());

        for ts in 0..step_count {
            let mut next_states = vec![];

            for _ in 0..queue.len() {
                let cur_state = queue.pop_front().unwrap();
                if cur_state.bank[&Resource::Geode] < max_geode_count {
                    continue;
                }

                max_geode_count = max_geode_count.max(cur_state.bank[&Resource::Geode]);

                // if we can build a geode robot, build it
                if cur_state.can_build(blueprint, Robot(Resource::Geode)) {
                    let mut new_state = cur_state.ticked();
                    new_state.build(blueprint, Robot(Resource::Geode));

                    // immediately add all geodes this robot will create
                    let t_rem: u64 = step_count - ts - 1;
                    new_state
                        .bank
                        .entry(Resource::Geode)
                        .and_modify(|n| *n += t_rem);

                    next_states.push(new_state);
                } else {
                    // build other robots
                    for resource in [Resource::Obsidian, Resource::Clay, Resource::Ore] {
                        if cur_state.can_build(blueprint, Robot(resource)) {
                            let mut new_state = cur_state.ticked();
                            new_state.build(blueprint, Robot(resource));

                            next_states.push(new_state);
                        }
                    }

                    // build nothing
                    next_states.push(cur_state.ticked());
                }
            }

            for state in next_states.drain(..) {
                if seen_states.insert(state.clone()) {
                    queue.push_back(state);
                }
            }
        }

        result.push((blueprint.id, max_geode_count));
    }

    result
}

fn part1() -> u64 {
    let result = run_simulation(24, &load_blueprints());

    result
        .into_iter()
        .map(|(blueprint_id, geode_count)| blueprint_id * geode_count)
        .sum()
}

fn part2() -> u64 {
    let blueprints = load_blueprints();
    let mut results = run_simulation(32, &blueprints[..3]);

    results.sort_unstable_by_key(|(_, geode_count)| *geode_count);
    results
        .iter()
        .rev()
        .map(|(_, geode_count)| geode_count)
        .take(3)
        .product()
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
        assert_eq!(part1(), 1356);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 27720);
    }
}
