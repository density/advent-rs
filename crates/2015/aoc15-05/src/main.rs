use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn is_vowel(c: u8) -> bool {
    matches!(c, b'a' | b'e' | b'i' | b'o' | b'u')
}

fn is_nice_part_1(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut has_double_letter = false;

    let bytes = s.as_bytes();

    if is_vowel(bytes[0]) {
        vowel_count += 1;
    }

    for i in 1..bytes.len() {
        let previous = bytes[i - 1];
        let current = bytes[i];

        if matches!(
            (previous, current),
            (b'a', b'b') | (b'c', b'd') | (b'p', b'q') | (b'x', b'y')
        ) {
            return false;
        }

        if previous == current {
            has_double_letter = true;
        }

        if is_vowel(current) {
            vowel_count += 1;
        }
    }

    vowel_count >= 3 && has_double_letter
}

fn is_nice_part_2(s: &str) -> bool {
    let bytes = s.as_bytes();

    let mut pairs: HashMap<(u8, u8), usize> = HashMap::new();

    let mut has_valid_pair = false;
    let mut has_valid_repeat = false;

    let mut i = 1;

    while i < bytes.len() && !(has_valid_pair && has_valid_repeat) {
        let current_pair = (bytes[i - 1], bytes[i]);

        let entry = pairs.entry(current_pair);

        match entry {
            Entry::Occupied(occupied_entry) => {
                if *occupied_entry.get() < i - 1 {
                    has_valid_pair = true;
                }
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(i);
            }
        }

        if let Some(next_byte) = bytes.get(i + 1) {
            if *next_byte == bytes[i - 1] {
                has_valid_repeat = true;
            }
        }

        i += 1;
    }

    has_valid_pair && has_valid_repeat
}

fn part1() -> usize {
    INPUT.lines().filter(|s| is_nice_part_1(s)).count()
}

fn part2() -> usize {
    INPUT.lines().filter(|s| is_nice_part_2(s)).count()
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
        assert_eq!(part1(), 236);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 51);
    }
}
