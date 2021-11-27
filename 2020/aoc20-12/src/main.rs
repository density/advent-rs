use hymns::vector2::{Rotation, Vector2};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

#[derive(Copy, Clone)]
enum Command {
    Move(Vector2<i64>, i64),
    Forward(i64),
    Turn(Rotation),
}

fn read_commands() -> impl Iterator<Item = Command> {
    INPUT.lines().map(|line| {
        let (cmd_str, amt) = line.split_at(1);
        let amt: i64 = amt.parse().unwrap();

        match cmd_str {
            "N" => Command::Move(Vector2::new(0, 1), amt),
            "S" => Command::Move(Vector2::new(0, -1), amt),
            "E" => Command::Move(Vector2::new(1, 0), amt),
            "W" => Command::Move(Vector2::new(-1, 0), amt),
            "R" => {
                let rotation = match amt {
                    90 => Rotation::Right90,
                    180 => Rotation::OneEighty,
                    270 => Rotation::Left90,
                    _ => unreachable!(),
                };
                Command::Turn(rotation)
            }
            "L" => {
                let rotation = match amt {
                    90 => Rotation::Left90,
                    180 => Rotation::OneEighty,
                    270 => Rotation::Right90,
                    _ => unreachable!(),
                };
                Command::Turn(rotation)
            }
            "F" => Command::Forward(amt),
            _ => unreachable!(),
        }
    })
}

fn part1() -> i64 {
    let mut ship_pos = Vector2::default();
    let mut ship_heading = Vector2::new(1, 0);

    for command in read_commands() {
        match command {
            Command::Move(mov_vec, distance) => ship_pos.move_by(mov_vec * distance),
            Command::Forward(distance) => ship_pos.move_by(ship_heading * distance),
            Command::Turn(degrees) => ship_heading.rotate(degrees),
        }
    }

    ship_pos.x.abs() + ship_pos.y.abs()
}

fn part2() -> i64 {
    let mut ship_pos = Vector2::default();
    let mut waypoint_pos = Vector2::new(10, 1);

    for command in read_commands() {
        match command {
            Command::Move(mov_vec, distance) => waypoint_pos.move_by(mov_vec * distance),
            Command::Forward(distance) => ship_pos.move_by(waypoint_pos * distance),
            Command::Turn(degrees) => waypoint_pos.rotate(degrees),
        }
    }

    ship_pos.x.abs() + ship_pos.y.abs()
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
        assert_eq!(part1(), 962);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 56135);
    }
}
