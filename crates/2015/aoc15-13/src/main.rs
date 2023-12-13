use std::collections::HashMap;
use std::time::Instant;

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn build_happiness_map() -> HashMap<&'static str, HashMap<&'static str, i64>> {
    let mut happiness_map: HashMap<&'static str, HashMap<&'static str, i64>> = HashMap::new();

    for line in INPUT.lines() {
        let words: Vec<_> = line.split_ascii_whitespace().collect();

        let recipient = words[0];
        let giver = words[10].strip_suffix('.').unwrap();

        let mut amount: i64 = words[3].parse().unwrap();
        if words[2] == "lose" {
            amount = -amount;
        }

        happiness_map
            .entry(recipient)
            .or_default()
            .insert(giver, amount);
    }

    happiness_map
}

fn part1() -> i64 {
    let happiness_map = build_happiness_map();

    let person_count = happiness_map.len();

    let mut max_happiness = 0;

    for permutation in happiness_map.keys().permutations(person_count) {
        let mut cur_happiness = 0;

        for i in 0..person_count {
            let neighbors = if i == 0 {
                [1, person_count - 1]
            } else if i == person_count - 1 {
                [person_count - 2, 0]
            } else {
                [i - 1, i + 1]
            };

            let cur_person = permutation[i];
            let first = permutation[neighbors[0]];
            let second = permutation[neighbors[1]];

            cur_happiness += happiness_map[cur_person][first] + happiness_map[cur_person][second];
        }

        max_happiness = max_happiness.max(cur_happiness);
    }

    max_happiness
}

fn part2() -> i64 {
    let happiness_map = build_happiness_map();

    let mut max_happiness = 0;

    let mut all_people: Vec<_> = happiness_map.keys().copied().collect();
    all_people.push("self");

    let person_count = all_people.len();

    for permutation in all_people.iter().permutations(all_people.len()) {
        let mut cur_happiness = 0;

        for i in 0..person_count {
            let neighbors = if i == 0 {
                [1, person_count - 1]
            } else if i == person_count - 1 {
                [person_count - 2, 0]
            } else {
                [i - 1, i + 1]
            };

            let cur_person = permutation[i];
            let first = permutation[neighbors[0]];
            let second = permutation[neighbors[1]];

            if let Some(map_for_person) = happiness_map.get(cur_person) {
                cur_happiness += map_for_person.get(first).unwrap_or(&0)
                    + map_for_person.get(second).unwrap_or(&0);
            }
        }

        max_happiness = max_happiness.max(cur_happiness);
    }

    max_happiness
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
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 733);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 725);
    }
}
