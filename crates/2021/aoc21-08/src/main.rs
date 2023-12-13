use std::collections::{HashMap, HashSet};

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

type CharMap = HashSet<char>;

fn part1() -> u64 {
    let mut counts = 0;

    for line in INPUT.lines() {
        let mut split = line.split(" | ");

        counts += split
            .nth(1)
            .unwrap()
            .split_ascii_whitespace()
            .filter_map(|item| match item.len() {
                2 | 3 | 7 | 4 => Some(1),
                _ => None,
            })
            .sum::<u64>();
    }

    counts
}

fn sorted_string(s: impl Iterator<Item = char>) -> String {
    let mut v: Vec<_> = s.collect();
    v.sort_unstable();
    v.into_iter().collect()
}

fn intersect_hashsets<'a>(mut sets: impl Iterator<Item = &'a CharMap>) -> CharMap {
    let mut result = sets.next().unwrap().clone();

    for set in sets {
        result.retain(|c| set.contains(c));
    }

    result
}

fn find_remove<P>(sets: &mut Vec<CharMap>, pred: P) -> CharMap
where
    P: FnMut(&CharMap) -> bool,
{
    let position = sets.iter().position(pred).unwrap();
    sets.swap_remove(position)
}

fn determine_mapping(observations: Vec<CharMap>) -> HashMap<String, u32> {
    let mut encoding_map: HashMap<u32, CharMap> = HashMap::new();

    let mut fives = vec![];
    let mut sixes = vec![];

    for observation in observations {
        match observation.len() {
            2 => {
                encoding_map.insert(1, observation);
            }
            3 => {
                encoding_map.insert(7, observation);
            }
            4 => {
                encoding_map.insert(4, observation);
            }
            5 => {
                fives.push(observation);
            }
            6 => {
                sixes.push(observation);
            }
            7 => {
                encoding_map.insert(8, observation);
            }
            _ => unreachable!(),
        }
    }

    // determine segments common to the four unique digits
    let common_to_4_uniques = intersect_hashsets(encoding_map.values());

    let three = find_remove(&mut fives, |map| {
        let to_intersect = [map, &common_to_4_uniques];
        intersect_hashsets(to_intersect.into_iter()).len() == 2
    });
    encoding_map.insert(3, three);

    // figure out the five segment digits
    let five = find_remove(&mut fives, |map| {
        let four_minus_seven = &encoding_map[&4] - &encoding_map[&7];
        (&four_minus_seven & map).len() == 2
    });
    encoding_map.insert(5, five);
    encoding_map.insert(2, fives.into_iter().next().unwrap());

    // figure out the six segment digits
    let nine = find_remove(&mut sixes, |map| (map & &encoding_map[&3]).len() == 5);
    encoding_map.insert(9, nine);

    let six = find_remove(&mut sixes, |map| (map & &encoding_map[&5]).len() == 5);
    encoding_map.insert(6, six);
    encoding_map.insert(0, sixes.into_iter().next().unwrap());

    encoding_map
        .iter()
        .map(|(digit, map)| (sorted_string(map.iter().copied()), *digit))
        .collect()
}

fn part2() -> u64 {
    let mut sum = 0;

    for line in INPUT.lines() {
        let mut split = line.split(" | ");

        let observations: Vec<CharMap> = split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|observation| observation.chars().collect())
            .collect();

        let digit_str_to_digit = determine_mapping(observations);

        let decoded_number: String = split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|digit_str| {
                let digit_str = sorted_string(digit_str.chars());
                digit_str_to_digit[&digit_str].to_string()
            })
            .collect();

        sum += decoded_number.parse::<u64>().unwrap();
    }

    sum
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
        assert_eq!(part1(), 247);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 933_305);
    }
}
