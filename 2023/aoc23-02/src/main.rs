use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{eof, map_opt, map_res};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

struct Drawing {
    red: usize,
    green: usize,
    blue: usize,
}

impl Drawing {
    fn add_cubes(&mut self, cubes: &Cubes) {
        match cubes.color {
            "red" => self.red += cubes.count,
            "green" => self.green += cubes.count,
            "blue" => self.blue += cubes.count,
            _ => unreachable!(),
        }
    }
}

impl<'a> FromIterator<Cubes<'a>> for Drawing {
    fn from_iter<T: IntoIterator<Item = Cubes<'a>>>(iter: T) -> Self {
        let mut drawing = Drawing {
            red: 0,
            green: 0,
            blue: 0,
        };

        for cubes in iter {
            drawing.add_cubes(&cubes);
        }

        drawing
    }
}

struct Game {
    id: u64,
    drawings: Vec<Drawing>,
}

struct Cubes<'a> {
    count: usize,
    color: &'a str,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(game(s).unwrap().1)
    }
}

fn game_id(input: &str) -> IResult<&str, u64> {
    terminated(
        preceded(tag("Game "), map_res(digit1, str::parse::<u64>)),
        tag(": "),
    )(input)
}

fn cube_with_count(input: &str) -> IResult<&str, Cubes> {
    map_opt(
        separated_pair(
            map_res(digit1, str::parse::<usize>),
            tag(" "),
            alt((tag("red"), tag("green"), tag("blue"))),
        ),
        |(count, color)| Some(Cubes { count, color }),
    )(input)
}

fn drawing(input: &str) -> IResult<&str, Vec<Cubes>> {
    terminated(
        separated_list1(tag(", "), cube_with_count),
        alt((eof, tag("; "))),
    )(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, game_id) = game_id(input)?;
    let (input, drawings): (&str, Vec<Vec<Cubes>>) = many1(drawing)(input)?;

    Ok((
        input,
        Game {
            id: game_id,
            drawings: drawings
                .into_iter()
                .map(|drawing| drawing.into_iter().collect())
                .collect(),
        },
    ))
}

fn part1() -> u64 {
    INPUT
        .lines()
        .filter_map(|line| {
            let game: Game = line.parse().unwrap();
            if game
                .drawings
                .into_iter()
                .all(|d| d.red <= 12 && d.green <= 13 && d.blue <= 14)
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn part2() -> usize {
    INPUT
        .lines()
        .map(|line| {
            let game: Game = line.parse().unwrap();
            let (r, g, b) = game.drawings.into_iter().fold((0, 0, 0), |(r, g, b), d| {
                (r.max(d.red), g.max(d.green), b.max(d.blue))
            });
            r * g * b
        })
        .sum()
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
        assert_eq!(part1(), 2913);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 55593);
    }
}
