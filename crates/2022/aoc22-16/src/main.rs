use std::collections::VecDeque;

use hashbrown::{HashMap, HashSet};
use hymns::runner::timed_run;
use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

type Pressure = usize;
type Valve = &'static str;
type FlowMap = HashMap<Valve, Pressure>;
type AdjList = HashMap<Valve, Vec<Valve>>;

struct CaveSystem {
    flows: FlowMap,
    adj: AdjList,
}

impl CaveSystem {
    fn new() -> Self {
        let re = Regex::new(r"Valve (?P<src>\w+).+=(?P<flow>\d+).+ves? (?P<dest>.+)").unwrap();

        let mut flows = HashMap::default();

        let mut adj = HashMap::default();

        for line in INPUT.lines() {
            let caps = re.captures(line).unwrap();

            let src = caps.name("src").unwrap().as_str();
            let rate: usize = caps["flow"].parse().unwrap();
            let targets: Vec<&str> = caps.name("dest").unwrap().as_str().split(", ").collect();

            flows.insert(src, rate);

            adj.insert(src, targets);
        }

        Self { flows, adj }
    }

    fn should_open(&self, valve: Valve, state: &State) -> bool {
        !state.open.contains(valve) && self.flows[valve] > 0
    }

    fn neighbor_moves(&self, state: &State, is_elephant: bool) -> Vec<Action> {
        let loc = if is_elephant {
            state.elephant_loc.unwrap()
        } else {
            state.my_loc
        };
        self.adj[loc]
            .iter()
            .map(|neigh| Action::Move(neigh, is_elephant))
            .collect()
    }

    fn transition(&self, state: &State, actions: &[Action]) -> State {
        let mut new_state = state.clone();

        for action in actions {
            match action {
                Action::Open(valve) => {
                    new_state.open.insert(valve);
                    new_state.rate += self.flows[valve];
                }
                Action::Move(valve, is_elephant) => {
                    if *is_elephant {
                        new_state.elephant_loc = Some(valve);
                    } else {
                        new_state.my_loc = valve;
                    }
                }
            }
        }

        new_state.ts += 1;
        new_state.released += new_state.rate;

        new_state
    }
}

#[derive(Copy, Clone)]
enum Action {
    Open(Valve),
    Move(Valve, bool),
}

#[derive(Debug, Clone)]
struct State {
    ts: usize,
    my_loc: &'static str,
    elephant_loc: Option<&'static str>,
    open: HashSet<&'static str>,
    released: usize,
    rate: usize,
}

impl State {
    fn key(&self) -> (&'static str, Option<&'static str>, usize) {
        (self.my_loc, self.elephant_loc, self.ts)
    }
}

fn part1() -> usize {
    let cave_system = CaveSystem::new();
    let mut max_released = 0;
    let mut queue = VecDeque::new();

    queue.push_back(State {
        ts: 1,
        my_loc: "AA",
        elephant_loc: None,
        open: HashSet::default(),
        released: 0,
        rate: 0,
    });

    let mut memo: HashMap<_, _> = HashMap::default();

    while let Some(cur) = queue.pop_front() {
        if let Some(&past_score) = memo.get(&cur.key()) {
            if past_score >= cur.released {
                continue;
            }
        }
        memo.insert(cur.key(), cur.released);

        if cur.ts == 30 {
            max_released = max_released.max(cur.released);
            continue;
        }

        // open currrent location
        if cave_system.should_open(cur.my_loc, &cur) {
            queue.push_back(cave_system.transition(&cur, &[Action::Open(cur.my_loc)]));
        }

        // don't open
        queue.extend(
            cave_system
                .neighbor_moves(&cur, false)
                .into_iter()
                .map(|action| cave_system.transition(&cur, &[action])),
        );
    }

    max_released
}

fn part2() -> usize {
    let cave_system = CaveSystem::new();

    let mut max_released = 0;

    let mut queue = VecDeque::new();
    queue.push_back(State {
        ts: 5,
        my_loc: "AA",
        elephant_loc: Some("AA"),
        open: HashSet::default(),
        released: 0,
        rate: 0,
    });

    let mut memo: HashMap<_, _> = HashMap::default();

    while let Some(cur) = queue.pop_front() {
        if let Some(&past_score) = memo.get(&cur.key()) {
            if past_score >= cur.released {
                continue;
            }
        }
        memo.insert(cur.key(), cur.released);

        if cur.ts == 30 {
            max_released = max_released.max(cur.released);
            continue;
        }

        let elephant_loc = cur.elephant_loc.unwrap();

        // i open currrent location
        if cave_system.should_open(cur.my_loc, &cur) {
            // elephant opens
            if elephant_loc != cur.my_loc && cave_system.should_open(elephant_loc, &cur) {
                queue.push_back(cave_system.transition(
                    &cur,
                    &[Action::Open(cur.my_loc), Action::Open(elephant_loc)],
                ));
            }

            // elephant moves
            queue.extend(
                cave_system
                    .neighbor_moves(&cur, true)
                    .into_iter()
                    .map(|move_action| {
                        cave_system.transition(&cur, &[Action::Open(cur.my_loc), move_action])
                    }),
            );
        }

        // i move
        for i_move_action in cave_system.neighbor_moves(&cur, false) {
            // elephant opens
            if cave_system.should_open(elephant_loc, &cur) {
                queue.push_back(
                    cave_system.transition(&cur, &[i_move_action, Action::Open(elephant_loc)]),
                );
            }

            // elephant moves
            queue.extend(cave_system.neighbor_moves(&cur, true).into_iter().map(
                |elephant_move_action| {
                    cave_system.transition(&cur, &[i_move_action, elephant_move_action])
                },
            ));
        }
    }

    max_released
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
        assert_eq!(part1(), 2059);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2790);
    }
}
