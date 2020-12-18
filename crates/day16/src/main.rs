use regex::Regex;
use std::collections::{HashMap, HashSet};

use std::iter::{FromIterator};

use std::ops::{RangeInclusive};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

type Ticket = Vec<u64>;

struct Field {
    lo_range: RangeInclusive<u64>,
    hi_range: RangeInclusive<u64>,
}

impl Field {
    fn new(lo_range: RangeInclusive<u64>, hi_range: RangeInclusive<u64>) -> Self {
        Self { lo_range, hi_range }
    }

    fn validate(&self, field_val: u64) -> bool {
        self.lo_range.contains(&field_val) || self.hi_range.contains(&field_val)
    }
}

fn read_fields(line_iter: &mut impl Iterator<Item = &'static str>) -> HashMap<String, Field> {
    let range_re =
        Regex::new(r"(?P<desc>.+?): (?P<lo_1>\d+)-(?P<hi_1>\d+) or (?P<lo_2>\d+)-(?P<hi_2>\d+)")
            .unwrap();

    line_iter
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let cap = range_re.captures(line).unwrap();

            let field_name = cap.name("desc").unwrap().as_str().to_string();

            let lo1 = cap.name("lo_1").unwrap().as_str().parse().unwrap();
            let hi1 = cap.name("hi_1").unwrap().as_str().parse().unwrap();

            let lo2 = cap.name("lo_2").unwrap().as_str().parse().unwrap();
            let hi2 = cap.name("hi_2").unwrap().as_str().parse().unwrap();

            (field_name, Field::new(lo1..=hi1, lo2..=hi2))
        })
        .collect()
}

fn read_tickets(
    line_iter: &mut impl Iterator<Item = &'static str>,
) -> impl Iterator<Item = Ticket> + '_ {
    line_iter.filter_map(|line| {
        if line.starts_with(|c: char| c.is_ascii_digit()) {
            Some(line.split(',').map(|num| num.parse().unwrap()).collect())
        } else {
            None
        }
    })
}

fn read_input() -> (HashMap<String, Field>, Ticket, Vec<Ticket>) {
    let mut line_iter = INPUT.lines();

    let all_fields = read_fields(&mut line_iter);

    let mut ticket_iter = read_tickets(&mut line_iter);

    let my_ticket = ticket_iter.next().unwrap();

    (all_fields, my_ticket, ticket_iter.collect())
}

fn part1() -> u64 {
    let (all_fields, _, nearby) = read_input();

    nearby
        .iter()
        .map(|ticket_fields| {
            ticket_fields
                .iter()
                .filter(|&&field_val| !all_fields.values().any(|field| field.validate(field_val)))
                .sum::<u64>()
        })
        .sum()
}

fn part2() -> u64 {
    let (all_fields, mine, mut nearby) = read_input();

    // remove invalid tickets
    nearby.retain(|ticket| {
        ticket
            .iter()
            .all(|field_val| all_fields.values().any(|field| field.validate(*field_val)))
    });

    let field_count = mine.len();

    // map from field to possible positions for that field
    let mut field_to_positions_map: HashMap<String, HashSet<usize>> = all_fields
        .keys()
        .map(|field_name| (field_name.clone(), HashSet::from_iter(0_usize..field_count)))
        .collect();

    for ticket in &nearby {
        for (field_pos, &field_val) in ticket.iter().enumerate() {
            for (field_name, positions) in &mut field_to_positions_map {
                if !all_fields[field_name].validate(field_val) {
                    positions.remove(&field_pos);
                }
            }
        }
    }

    while field_to_positions_map.values().any(|pos| pos.len() != 1) {
        let known: Vec<_> = field_to_positions_map
            .values()
            .filter_map(|positions| {
                if positions.len() == 1 {
                    positions.iter().next().cloned()
                } else {
                    None
                }
            })
            .collect();

        for positions in field_to_positions_map.values_mut() {
            if positions.len() > 1 {
                positions.retain(|n| !known.contains(n))
            }
        }
    }

    all_fields
        .keys()
        .filter_map(|key| {
            if key.starts_with("departure") {
                let pos = field_to_positions_map[key].iter().next().unwrap();
                Some(mine[*pos])
            } else {
                None
            }
        })
        .product()
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
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 18142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1069784384303);
    }
}
