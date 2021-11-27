use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> usize {
    let mut encoded_count = 0;
    let mut decoded_count = 0;

    for line in INPUT.lines() {
        encoded_count += 2; // quotes at beginning and end

        let mut chars = line.chars().skip(1).take(line.len() - 2);
        while let Some(c) = chars.next() {
            encoded_count += 1;
            decoded_count += 1;

            if c == '\\' {
                match chars.next().unwrap() {
                    '\\' | '"' => {
                        encoded_count += 1; // \ or "
                    }
                    'x' => {
                        encoded_count += 3; // x and 2 digits
                        chars.next().unwrap();
                        chars.next().unwrap();
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    encoded_count - decoded_count
}

fn part2() -> usize {
    let mut original_count = 0;
    let mut newly_encoded_count = 0;

    for line in INPUT.lines() {
        original_count += line.len();
        newly_encoded_count += 6; // quotes and escaped quotes at beginning and end

        for c in line.chars().skip(1).take(line.len() - 2) {
            match c {
                '\\' | '"' => newly_encoded_count += 2,
                _ => newly_encoded_count += 1,
            }
        }
    }

    newly_encoded_count - original_count
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
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1350);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2085);
    }
}
