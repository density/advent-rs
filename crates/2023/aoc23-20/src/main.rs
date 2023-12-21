use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

use itertools::Itertools;

use hymns::math::lcm;
use hymns::more_itertools::Counter;
use hymns::runner::timed_run;

use crate::Level::{High, Low};

const INPUT: &str = include_str!("../input.txt");

type ModuleID<'a> = &'a str;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Level {
    High,
    Low,
}

#[derive(Copy, Clone)]
struct Pulse<'a> {
    src: ModuleID<'a>,
    dst: ModuleID<'a>,
    level: Level,
}

impl<'a> Pulse<'a> {
    const fn button_press() -> Self {
        Self {
            src: "broadcaster",
            dst: "broadcaster",
            level: Low,
        }
    }
}

enum ModuleKind<'a> {
    Broadcast,
    Flipflop {
        on: bool,
    },
    Conjunction {
        inputs: HashMap<ModuleID<'a>, Level>,
    },
}

struct Module<'a> {
    id: &'a str,
    module: ModuleKind<'a>,
}

impl<'a> Module<'a> {
    fn broadcast(id: &'a str) -> Self {
        Self {
            id,
            module: ModuleKind::Broadcast,
        }
    }

    fn flipflop(id: &'a str) -> Self {
        Self {
            id,
            module: ModuleKind::Flipflop { on: false },
        }
    }

    fn conjunction(id: &'a str, inputs: impl Iterator<Item = &'a str>) -> Self {
        let inputs = inputs.map(|input| (input, Low)).collect();
        Self {
            id,
            module: ModuleKind::Conjunction { inputs },
        }
    }

    fn receive(&mut self, pulse: Pulse<'a>) -> Option<Level> {
        match self.module {
            ModuleKind::Broadcast => Some(pulse.level),
            ModuleKind::Flipflop { ref mut on } => {
                if pulse.level == High {
                    return None;
                }

                let result = if *on { Low } else { High };
                *on = !*on;
                Some(result)
            }
            ModuleKind::Conjunction { ref mut inputs } => {
                inputs.insert(pulse.src, pulse.level);

                if inputs.values().all(|level| *level == High) {
                    Some(Low)
                } else {
                    Some(High)
                }
            }
        }
    }
}

fn read_input() -> (
    HashMap<&'static str, Module<'static>>,
    HashMap<&'static str, Vec<&'static str>>,
) {
    let mut connections = HashMap::new();
    let mut modules = HashMap::new();

    let mut conj_names = HashSet::new();

    for line in INPUT.lines() {
        let (src, dsts) = line.split(" -> ").collect_tuple().unwrap();
        let dsts = dsts.split(", ").collect_vec();

        if src == "broadcaster" {
            modules.insert(src, Module::broadcast(src));
            connections.insert(src, dsts);
        } else {
            let (prefix, src) = src.split_at(1);
            connections.insert(src, dsts);

            if prefix == "%" {
                modules.insert(src, Module::flipflop(src));
            } else {
                conj_names.insert(src);
            }
        }
    }

    // conjunctions need to know thier inputs
    for name in conj_names {
        let inputs = connections.iter().filter_map(|(src, dsts)| {
            if dsts.contains(&name) {
                Some(*src)
            } else {
                None
            }
        });
        modules.insert(name, Module::conjunction(name, inputs));
    }

    (modules, connections)
}

fn part1() -> usize {
    let (mut modules, connections) = read_input();

    let mut pulses = VecDeque::from(vec![Pulse::button_press(); 1000]);

    let mut counter = Counter::new();

    while let Some(pulse @ Pulse { dst, level, .. }) = pulses.pop_front() {
        *counter.get_mut(level) += 1;

        let Some(module) = modules.get_mut(dst) else {
            continue;
        };

        let Some(next) = module.receive(pulse) else {
            continue;
        };

        pulses.extend(connections[module.id].iter().map(|dst| Pulse {
            src: module.id,
            dst,
            level: next,
        }));
    }

    counter.into_values().product()
}

fn part2() -> usize {
    let (mut modules, connections) = read_input();

    let targets = ["vt", "kk", "xc", "sk"];
    let mut press_counts = HashMap::with_capacity(4);

    let mut pulses = VecDeque::new();

    for count in 1_usize.. {
        if press_counts.len() == 4 {
            break;
        }
        pulses.push_back(Pulse::button_press());

        while let Some(pulse @ Pulse { src, dst, level }) = pulses.pop_front() {
            let Some(module) = modules.get_mut(dst) else {
                continue;
            };

            if level == High && dst == "tj" && targets.contains(&src) {
                press_counts.insert(src, count);
            }

            let Some(next) = module.receive(pulse) else {
                continue;
            };

            pulses.extend(connections[module.id].iter().map(|dst| Pulse {
                src: module.id,
                dst,
                level: next,
            }));
        }
    }

    press_counts.into_values().reduce(lcm).unwrap()
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
        assert_eq!(part1(), 818_649_769);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 246_313_604_784_977);
    }
}
