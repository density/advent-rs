use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn get_passports() -> impl Iterator<Item = HashMap<&'static str, &'static str>> {
    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    INPUT
        .split("\n\n")
        .map(|spec| {
            spec.split(|c| c == '\n' || c == ' ')
                .map(|component| {
                    let mut split = component.split(':');
                    (split.next().unwrap(), split.next().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .filter(move |passport| required_keys.iter().all(|k| passport.contains_key(k)))
}

fn is_valid(key: &str, value: &str) -> bool {
    match key {
        "byr" => matches!(value.parse::<u64>(), Ok(1920..=2002)),
        "iyr" => matches!(value.parse::<u64>(), Ok(2010..=2020)),
        "eyr" => matches!(value.parse::<u64>(), Ok(2020..=2030)),
        "hgt" => {
            if let Some(height) = value.strip_suffix("cm") {
                matches!(height.parse::<u64>(), Ok(150..=193))
            } else if let Some(height) = value.strip_suffix("in") {
                matches!(height.parse::<u64>(), Ok(59..=76))
            } else {
                false
            }
        }
        "hcl" => match value.split_at(1) {
            ("#", rest) => {
                rest.len() == 6
                    && rest
                        .chars()
                        .all(|c| c.to_ascii_lowercase().is_ascii_hexdigit())
            }
            _ => false,
        },
        "ecl" => matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "pid" => value.len() == 9 && value.parse::<u64>().is_ok(),
        _ => true,
    }
}

fn part1() -> usize {
    get_passports().count()
}

fn part2() -> usize {
    get_passports()
        .filter(|pass| pass.iter().all(|(k, v)| is_valid(k, v)))
        .count()
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 233);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 111);
    }
}
