use std::mem;
use std::ops::{AddAssign, Mul};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

#[derive(Default, Copy, Clone)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn rotate(&mut self, degrees: i64) {
        match degrees {
            0 => (),
            90 | -270 => {
                mem::swap(&mut self.x, &mut self.y);
                self.y = -self.y
            }
            180 | -180 => {
                self.x = -self.x;
                self.y = -self.y;
            }
            270 | -90 => {
                mem::swap(&mut self.x, &mut self.y);
                self.x = -self.x;
            }
            _ => unreachable!(),
        }
    }

    fn move_by(&mut self, delta: Vector) {
        *self += delta;
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Copy, Clone)]
enum Command {
    Move(Vector, i64),
    Forward(i64),
    Turn(i64),
}

fn read_commands() -> impl Iterator<Item = Command> {
    INPUT.lines().map(|line| {
        let (cmd_str, amt) = line.split_at(1);
        let amt: i64 = amt.parse().unwrap();

        match cmd_str {
            "N" => Command::Move(Vector::new(0, 1), amt),
            "S" => Command::Move(Vector::new(0, -1), amt),
            "E" => Command::Move(Vector::new(1, 0), amt),
            "W" => Command::Move(Vector::new(-1, 0), amt),
            "L" => Command::Turn(-amt),
            "R" => Command::Turn(amt),
            "F" => Command::Forward(amt),
            _ => unreachable!(),
        }
    })
}

fn part1() -> i64 {
    let mut ship_pos = Vector::default();
    let mut ship_heading = Vector::new(1, 0);

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
    let mut ship_pos = Vector::default();
    let mut waypoint_pos = Vector::new(10, 1);

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
