use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn spec_iter() -> impl Iterator<Item = (usize, usize, char, String)> {
    let re = Regex::new(
        r"(?x)
                (?P<min>\d+)
                -
                (?P<max>\d+)
                \s
                (?P<letter>[[:alpha:]]+)
                :\s
                (?P<pw>[[:alpha:]]+)",
    )
    .unwrap();

    INPUT.lines().into_iter().map(move |spec| {
        let cap = re.captures(spec).unwrap();

        let min: usize = cap["min"].parse().unwrap();
        let max: usize = cap["max"].parse().unwrap();
        let letter = cap["letter"].chars().next().unwrap();
        let pw = cap["pw"].to_string();

        (min, max, letter, pw)
    })
}

fn part1() -> usize {
    spec_iter()
        .filter(|(min, max, letter, pw)| {
            let char_count = pw.chars().filter(|c| c == letter).count();

            char_count >= *min && char_count <= *max
        })
        .count()
}

fn part2() -> usize {
    spec_iter()
        .filter(|(min, max, letter, pw)| {
            let min = min - 1;
            let max = max - 1;
            let mut pw_iter = pw.chars();

            let first_matches = pw_iter.nth(min).unwrap() == *letter;
            let second_matches = pw_iter.nth(max - min - 1).unwrap() == *letter;

            (first_matches || second_matches) && !(first_matches && second_matches)
        })
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
        assert_eq!(part1(), 582);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 729);
    }
}
