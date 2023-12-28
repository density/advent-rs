use hashbrown::HashSet;
use std::collections::VecDeque;
use std::iter::IntoIterator;

use regex::Regex;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
#[repr(usize)]
enum Resource {
    Ore = 0,
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

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct State {
    robots: [u64; 4],
    bank: [u64; 4],
}

impl State {
    fn new() -> Self {
        let mut robots = [0; 4];
        robots[Resource::Ore as usize] = 1;

        Self {
            robots,
            bank: [0_u64; 4],
        }
    }

    fn get_robots(&self, robot: Robot) -> u64 {
        self.robots[robot.0 as usize]
    }

    fn get_resource(&self, resource: Resource) -> u64 {
        self.bank[resource as usize]
    }

    fn add_resource(&mut self, resource: Resource, count: u64) {
        self.bank[resource as usize] += count;
    }

    fn use_resource(&mut self, resource: Resource, count: u64) {
        self.bank[resource as usize] = self.bank[resource as usize].checked_sub(count).unwrap();
    }

    fn add_robot(&mut self, robot: Robot) {
        self.robots[robot.0 as usize] += 1;
    }

    fn increment_resources(&mut self) {
        for resource in ALL_RESOURCES {
            // don't increment geodes because we increment them as soon as we build the robot
            if resource == Resource::Geode {
                continue;
            }
            self.bank[resource as usize] += self.get_robots(Robot(resource));
        }
    }

    fn can_build(&self, blueprint: &Blueprint, robot: Robot) -> bool {
        if self.get_robots(robot) >= blueprint.usable_per_turn(robot.0) {
            return false;
        }

        match robot {
            Robot(Resource::Ore) => self.get_resource(Resource::Ore) >= blueprint.ore_robot_ore,
            Robot(Resource::Clay) => self.get_resource(Resource::Ore) >= blueprint.clay_robot_ore,
            Robot(Resource::Obsidian) => {
                self.get_resource(Resource::Ore) >= blueprint.obsidian_robot_ore
                    && self.get_resource(Resource::Clay) >= blueprint.obsidian_robot_clay
            }
            Robot(Resource::Geode) => {
                self.get_resource(Resource::Ore) >= blueprint.geode_robot_ore
                    && self.get_resource(Resource::Obsidian) >= blueprint.geode_robot_obsidian
            }
        }
    }

    fn build(&mut self, blueprint: &Blueprint, robot: Robot) {
        match robot {
            Robot(Resource::Ore) => {
                self.use_resource(Resource::Ore, blueprint.ore_robot_ore);
            }
            Robot(Resource::Clay) => {
                self.use_resource(Resource::Ore, blueprint.clay_robot_ore);
            }
            Robot(Resource::Obsidian) => {
                self.use_resource(Resource::Ore, blueprint.obsidian_robot_ore);
                self.use_resource(Resource::Clay, blueprint.obsidian_robot_clay);
            }
            Robot(Resource::Geode) => {
                self.use_resource(Resource::Ore, blueprint.geode_robot_ore);
                self.use_resource(Resource::Obsidian, blueprint.geode_robot_obsidian);
            }
        }

        self.add_robot(robot);
    }

    fn ticked(&self) -> Self {
        let mut new_state = self.clone();
        new_state.increment_resources();
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
        let mut seen_states: HashSet<State> = HashSet::default();

        let mut queue = VecDeque::new();
        queue.push_back(State::new());
        seen_states.insert(State::new());

        for ts in 0..step_count {
            let mut next_states = vec![];

            for _ in 0..queue.len() {
                let cur_state = queue.pop_front().unwrap();
                if cur_state.get_resource(Resource::Geode) < max_geode_count {
                    continue;
                }

                max_geode_count = max_geode_count.max(cur_state.get_resource(Resource::Geode));

                // if we can build a geode robot, build it
                if cur_state.can_build(blueprint, Robot(Resource::Geode)) {
                    let mut new_state = cur_state.ticked();
                    new_state.build(blueprint, Robot(Resource::Geode));

                    // immediately add all geodes this robot will create
                    let t_rem: u64 = step_count - ts - 1;
                    new_state.add_resource(Resource::Geode, t_rem);

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

            queue.extend(
                next_states
                    .drain(..)
                    .filter(|state| seen_states.insert(state.clone())),
            );
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
