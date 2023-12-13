use std::collections::HashSet;

use regex::Regex;

use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;

const INPUT: &str = include_str!("../input.txt");

type Point = Point2<isize>;

#[derive(Debug)]
struct Sensor {
    loc: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn scan_radius(&self) -> isize {
        self.loc.manhattan_dist(&self.closest_beacon)
    }

    fn points_beyond_edge(&self) -> Vec<Point> {
        let mut points = vec![];

        let r = self.scan_radius() + 1;
        for dx in -r..=r {
            let dy = r - dx.abs();

            points.push(p2!(self.loc.x + dx, self.loc.y + dy));
            // avoid dupes
            if dy != 0 {
                points.push(p2!(self.loc.x + dx, self.loc.y - dy));
            }
        }

        points
    }
}

fn load_sensors() -> Vec<Sensor> {
    let re = Regex::new(r"Sensor at x=(?P<s_x>.+?), y=(?P<s_y>.+?): closest beacon is at x=(?P<b_x>.+?), y=(?P<b_y>.+?)$").unwrap();

    INPUT
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            Sensor {
                loc: p2!(caps["s_x"].parse().unwrap(), caps["s_y"].parse().unwrap()),
                closest_beacon: p2!(caps["b_x"].parse().unwrap(), caps["b_y"].parse().unwrap()),
            }
        })
        .collect()
}

fn part1() -> usize {
    let sensors = load_sensors();

    let target_y = 2_000_000;

    let mut result = HashSet::new();

    for sensor in sensors {
        let r = sensor.loc.manhattan_dist(&sensor.closest_beacon);

        let x_min = sensor.loc.x - r;
        let x_max = sensor.loc.x + r;

        for x in x_min..=x_max {
            let d = p2!(x, target_y).manhattan_dist(&sensor.loc);

            if d <= r && p2!(x, target_y) != sensor.closest_beacon {
                result.insert(x);
            }
        }
    }

    result.len()
}

fn part2() -> isize {
    let sensors = load_sensors();

    let high_limit = 4_000_000;

    for sensor in &sensors {
        for edge_point in sensor.points_beyond_edge() {
            if (0..=high_limit).contains(&edge_point.x)
                && (0..=high_limit).contains(&edge_point.y)
                && sensors
                    .iter()
                    .all(|s| s.loc.manhattan_dist(&edge_point) > s.scan_radius())
            {
                return edge_point.x * 4_000_000 + edge_point.y;
            }
        }
    }

    unreachable!()
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
        assert_eq!(part1(), 5_127_797);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 12_518_502_636_475);
    }
}
