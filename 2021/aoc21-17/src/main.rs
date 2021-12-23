use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;
use std::collections::HashSet;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input.txt");

type Point = Point2<isize>;
type Velocity = Point2<isize>;

fn step(point: &mut Point, velocity: &mut Velocity) {
    *point += *velocity;

    *velocity = p2!(velocity.x - velocity.x.signum(), velocity.y - 1);
}

fn check_velocity(
    start_velocity: Velocity,
    target_x: &RangeInclusive<isize>,
    target_y: &RangeInclusive<isize>,
) -> Option<isize> {
    let mut point = p2!(0, 0);

    let mut cur_velocity = start_velocity;
    let mut max_reached = 0;

    while point.x <= *target_x.end() {
        max_reached = max_reached.max(point.y);

        if target_x.contains(&point.x) && target_y.contains(&point.y) {
            return Some(max_reached);
        }

        if cur_velocity.x == 0 && point.y < *target_y.start() {
            break;
        }

        step(&mut point, &mut cur_velocity);
    }

    None
}

fn part1() -> isize {
    let target_x = 192..=251;
    let target_y = -89..=-59;

    // x velocity has to be less than target_x.end()

    let mut result = 0;

    for x_vel in 1..=(*target_x.end()) {
        for y_vel in *target_y.start()..=1000 {
            if let Some(max_height) = check_velocity(p2!(x_vel, y_vel), &target_x, &target_y) {
                result = result.max(max_height);
            }
        }
    }

    result
}

fn part2() -> usize {
    let target_x = 192..=251;
    let target_y = -89..=-59;

    let mut good_velocities = HashSet::new();

    for x_vel in 1..=(*target_x.end()) {
        for y_vel in *target_y.start()..=1000 {
            let velocity = p2!(x_vel, y_vel);
            if check_velocity(velocity, &target_x, &target_y).is_some() {
                good_velocities.insert(velocity);
            }
        }
    }

    good_velocities.len()
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
        assert_eq!(part1(), 3916);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2986);
    }
}
