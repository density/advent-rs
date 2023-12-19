use nom::branch::alt;
use std::str::FromStr;

use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete;
use nom::character::complete::{digit1, one_of};
use nom::combinator::{into, map, map_res};
use nom::multi::{many1, many_till, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::IResult;

use hymns::runner::timed_run;

use crate::Attribute::{Aero, Cool, Music, Shine};

const INPUT: &str = include_str!("../input.txt");

// qqz{s>2770:qs,m<1801:hdj,R}
fn workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = workflow_name(input)?;

    dbg!(name);

    let (input, (conditions, catchall)) = delimited(
        complete::char('{'),
        many_till(terminated(condition, complete::char(',')), decision),
        complete::char('}'),
    )(input)?;

    Ok((
        input,
        Workflow {
            name,
            conditions,
            catchall,
        },
    ))
}

fn workflow_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_lowercase())(input)
}

fn decision<'a>(input: &'a str) -> IResult<&'a str, Decision<'a>> {
    let (input, d) = alt((workflow_name, tag("R"), tag("A")))(input)?;

    let decision = match d {
        "R" => Decision::Reject,
        "A" => Decision::Accept,
        name => Decision::Jump(name),
    };

    Ok((input, decision))
}

fn operator(input: &str) -> IResult<&str, Operator> {
    println!("condition");
    map(one_of("<>"), Operator::from)(input)
}

fn condition(input: &str) -> IResult<&str, Condition> {
    println!("condition");
    let (input, (attr, op, rating, _, decision)) = tuple((
        part_attr,
        operator,
        part_rating,
        complete::char(':'),
        decision,
    ))(input)?;


    Ok((
        input,
        Condition {
            attribute: attr,
            operator: op,
            rating,
            result: decision,
        },
    ))
}

fn part_attr(input: &str) -> IResult<&str, Attribute> {
    let (input, c) = one_of("xmas")(input)?;

    Ok((input, c.into()))
}

fn part_rating(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(input)
}

fn part(input: &str) -> Part {
    let (input, inner) = delimited(
        complete::char('{'),
        separated_list1(
            complete::char(','),
            separated_pair(part_attr, complete::char('='), part_rating),
        ),
        complete::char('}'),
    )(input)
    .unwrap();

    let mut part = Part::new();
    for (attr, rating) in inner {
        part.set_attribute(attr, rating);
    }

    part
}

// =================== END PARSING ==================

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Attribute {
    Cool,
    Music,
    Aero,
    Shine,
}

impl From<char> for Attribute {
    fn from(value: char) -> Self {
        match value {
            'x' => Cool,
            'm' => Music,
            'a' => Aero,
            's' => Shine,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug)]
struct Part {
    attributes: [u64; 4],
}

impl Part {
    fn new() -> Self {
        Self { attributes: [0; 4] }
    }

    fn set_attribute(&mut self, attribute: Attribute, rating: u64) {
        self.attributes[attribute as usize] = rating;
    }
}

#[derive(Debug)]
enum Decision<'a> {
    Accept,
    Reject,
    Jump(&'a str),
}

impl<'a> From<char> for Decision<'a> {
    fn from(value: char) -> Self {
        match value {
            'A' => Decision::Accept,
            'R' => Decision::Reject,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Operator {
    GT,
    LT,
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '>' => Operator::GT,
            '<' => Operator::LT,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Condition<'a> {
    attribute: Attribute,
    operator: Operator,
    rating: u64,
    result: Decision<'a>,
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    conditions: Vec<Condition<'a>>,
    catchall: Decision<'a>,
}

fn part1() -> u64 {
    let part = part("{x=787,m=2655,a=1222,s=2876}");

    let wf = workflow("qqz{s>2770:qs,m<1801:hdj,R}");

    dbg!(wf);
    0
}

fn part2() -> u64 {
    0
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
        assert_eq!(part1(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
