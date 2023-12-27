use rustc_hash::{FxHashMap, FxHashSet};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone)]
enum Rule {
    Terminal(char),
    NonTerminal(Vec<usize>),
}

fn parse_rule(s: &str) -> Vec<Rule> {
    if s.starts_with('"') {
        return vec![Rule::Terminal(s.chars().nth(1).unwrap())];
    }

    let mut all_rules = vec![];

    for group in s.split(" | ") {
        all_rules.push(Rule::NonTerminal(
            group
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        ));
    }

    all_rules
}

fn make_rules<'a, 'b>(
    line_iter: &'a mut impl Iterator<Item = &'b str>,
) -> FxHashMap<usize, Vec<Rule>> {
    let mut rules = FxHashMap::default();

    for line in line_iter.take_while(|line| !line.is_empty()) {
        let mut split = line.split(": ");

        let rule_id: usize = split.next().unwrap().parse().unwrap();

        let rule_list = parse_rule(split.next().unwrap());

        rules.insert(rule_id, rule_list);
    }

    remove_more_than_two_nonterminals(&mut rules);
    remove_lone_non_terminals(&mut rules);

    rules
}

fn remove_lone_non_terminals(rules: &mut FxHashMap<usize, Vec<Rule>>) {
    let mut remapping = FxHashMap::default();

    for (left, right_sides) in rules.iter() {
        if right_sides
            .iter()
            .any(|rhs| matches!(rhs, Rule::NonTerminal(x) if x.len() == 1))
        {
            let mut new_rhs = Vec::with_capacity(right_sides.len());

            for rhs in right_sides {
                match rhs {
                    Rule::NonTerminal(idx_list) if idx_list.len() == 1 => {
                        let rhs_for_this_rule = rules.get(&idx_list[0]).unwrap();

                        new_rhs.extend(rhs_for_this_rule.iter().cloned());
                    }
                    _ => new_rhs.push(rhs.clone()),
                }
            }

            remapping.insert(*left, new_rhs);
        }
    }

    rules.extend(remapping.drain());
}

fn remove_more_than_two_nonterminals(rules: &mut FxHashMap<usize, Vec<Rule>>) {
    let mut next_rule_id = rules.keys().max().unwrap() + 1;

    let mut to_add = FxHashMap::default();

    for (left, right_sides) in rules.iter() {
        let is_correct_length = right_sides.iter().all(|rule| {
            matches!(rule, Rule::Terminal(_))
                || matches!(rule, Rule::NonTerminal(x) if x.len() <= 2)
        });

        if is_correct_length {
            continue;
        }

        let mut new_right_sides = Vec::with_capacity(right_sides.len());

        for rhs in right_sides {
            match rhs {
                Rule::NonTerminal(rule_list) if rule_list.len() > 2 => {
                    new_right_sides.push(Rule::NonTerminal(vec![rule_list[0], next_rule_id]));
                    to_add.insert(
                        next_rule_id,
                        vec![Rule::NonTerminal(rule_list[1..].to_vec())],
                    );
                    next_rule_id += 1;
                }
                _ => new_right_sides.push(rhs.clone()),
            }
        }

        to_add.insert(*left, new_right_sides);
    }

    rules.extend(to_add.drain());
}

fn matches_rules3(test_str: &str, rules: &FxHashMap<usize, Vec<Rule>>) -> bool {
    let n = test_str.len();

    let mut memo: Vec<Vec<Option<FxHashSet<usize>>>> = vec![vec![None; n + 1]; n + 1];

    // See https://en.wikipedia.org/wiki/CYK_algorithm
    for (s, c) in test_str.chars().enumerate() {
        for (left, right_sides) in rules {
            for rhs in right_sides {
                match rhs {
                    Rule::Terminal(some_terminal) if *some_terminal == c => {
                        memo[1][s + 1]
                            .get_or_insert_with(FxHashSet::default)
                            .insert(*left);
                    }
                    _ => (),
                }
            }
        }
    }

    for l in 2..=n {
        for s in 0..=n - l + 1 {
            for p in 1..l {
                for (left, right_sides) in rules {
                    for rhs in right_sides {
                        match rhs {
                            Rule::Terminal(_) => (),
                            Rule::NonTerminal(rule_indexes) => {
                                let matches_first = memo[p][s]
                                    .as_ref()
                                    .map_or(false, |hash| hash.contains(&rule_indexes[0]));

                                if matches_first {
                                    let matches_second = memo[l - p][s + p]
                                        .as_ref()
                                        .map_or(false, |hash| hash.contains(&rule_indexes[1]));

                                    if matches_second {
                                        memo[l][s]
                                            .get_or_insert_with(FxHashSet::default)
                                            .insert(*left);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    memo[n][1].as_ref().map_or(false, |hash| hash.contains(&0))
}

fn part1() -> usize {
    let mut line_iter = INPUT.lines();

    let rules = make_rules(&mut line_iter);

    line_iter
        .filter(|line| matches_rules3(line, &rules))
        .count()
}

fn part2() -> usize {
    let replaced = INPUT
        .replace("8: 42\n", "8: 42 | 42 8\n")
        .replace("11: 42 31\n", "11: 42 31 | 42 11 31\n");
    let mut line_iter = replaced.lines();

    let rules = make_rules(&mut line_iter);

    let result = line_iter
        .filter(|line| matches_rules3(line, &rules))
        .count();

    drop(replaced);

    result
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
        assert_eq!(part1(), 279);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 384);
    }
}
