use std::time::Instant;

use serde_json::Value;

const INPUT: &str = include_str!("../input.txt");

fn is_digit_char(c: char) -> bool {
    c.is_ascii_digit() || c == '-'
}

fn part1() -> i64 {
    let mut chars = INPUT.chars();

    let mut sum = 0;

    loop {
        let s = chars
            .by_ref()
            .skip_while(|&c| !is_digit_char(c))
            .take_while(|c| is_digit_char(*c))
            .collect::<String>();
        if s.is_empty() {
            break;
        }
        sum += s.parse::<i64>().unwrap();
    }

    sum
}

fn sum_numbers(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(sum_numbers).sum(),
        Value::Object(o) => o
            .values()
            .try_fold(0, |acc, v| match v {
                Value::String(s) if s == &"red".to_string() => None,
                v => Some(acc + sum_numbers(v)),
            })
            .unwrap_or(0),
        _ => 0,
    }
}

fn part2() -> i64 {
    let v: Value = serde_json::from_str(INPUT).unwrap();
    sum_numbers(&v)
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
        assert_eq!(part1(), 156366);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 96852);
    }
}
