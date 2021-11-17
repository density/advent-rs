const INPUT: &str = include_str!("../input.txt");

fn get_seat(char_iter: &mut impl Iterator<Item = char>) -> u64 {
    char_iter.fold(0, |acc, c| {
        let half = match c {
            'R' | 'B' => 1,
            _ => 0,
        };
        (acc << 1) | half
    })
}

fn part1() -> u64 {
    INPUT
        .lines()
        .map(|line| get_seat(&mut line.chars()))
        .max()
        .unwrap()
}

fn part2() -> u64 {
    let mut seat_ids: Vec<_> = INPUT
        .lines()
        .map(|line| get_seat(&mut line.chars()))
        .collect();
    seat_ids.sort_unstable();

    seat_ids
        .windows(2)
        .find(|tup| tup[0] + 1 != tup[1])
        .unwrap()[0]
        + 1
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
        assert_eq!(part1(), 989);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 548);
    }
}
