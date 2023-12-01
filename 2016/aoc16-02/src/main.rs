use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;

const INPUT: &str = include_str!("../input.txt");

fn get_key_p1(point: Point2<usize>) -> Option<char> {
    match (point.x, point.y) {
        (0, 0) => Some('1'),
        (1, 0) => Some('2'),
        (2, 0) => Some('3'),
        (0, 1) => Some('4'),
        (1, 1) => Some('5'),
        (2, 1) => Some('6'),
        (0, 2) => Some('7'),
        (1, 2) => Some('8'),
        (2, 2) => Some('9'),
        _ => None,
    }
}

fn get_key_p2(point2: Point2<usize>) -> Option<char> {
    match (point2.x, point2.y) {
        (2, 0) => Some('1'),
        (1, 1) => Some('2'),
        (2, 1) => Some('3'),
        (3, 1) => Some('4'),
        (0, 2) => Some('5'),
        (1, 2) => Some('6'),
        (2, 2) => Some('7'),
        (3, 2) => Some('8'),
        (4, 2) => Some('9'),
        (1, 3) => Some('A'),
        (2, 3) => Some('B'),
        (3, 3) => Some('C'),
        (2, 4) => Some('D'),
        _ => None,
    }
}

fn part1() -> String {
    run(p2!(1, 1), get_key_p1)
}

fn part2() -> String {
    run(p2!(0, 2), get_key_p2)
}

fn run(start_location: Point2<usize>, get_key_fn: fn(Point2<usize>) -> Option<char>) -> String {
    let mut result = String::new();

    let mut loc = start_location;

    for line in INPUT.lines() {
        for dir in line.chars() {
            let new_point = match dir {
                'U' => p2!(loc.x, loc.y.saturating_sub(1)),
                'D' => loc + p2!(0, 1),
                'L' => p2!(loc.x.saturating_sub(1), loc.y),
                'R' => loc + p2!(1, 0),
                _ => unreachable!(),
            };

            if get_key_fn(new_point).is_some() {
                loc = new_point;
            }
        }

        result.push(get_key_fn(loc).unwrap());
    }

    result
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
        assert_eq!(part1(), "84452".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "D65C3".to_string());
    }
}
