use crate::workflow::Verdict::{Accept, Jump, Reject};
use crate::workflow::{Attribute, Condition, Operator, Part, Verdict, Workflow};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::char as nom_char;
use nom::character::complete::{digit1, one_of};
use nom::combinator::{map, map_res};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::IResult;

fn workflow_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_lowercase())(input)
}

fn verdict(input: &str) -> IResult<&str, Verdict> {
    let (input, d) = alt((workflow_name, tag("R"), tag("A")))(input)?;

    Ok((
        input,
        match d {
            "R" => Reject,
            "A" => Accept,
            name => Jump(name),
        },
    ))
}

fn operator(input: &str) -> IResult<&str, Operator> {
    map(one_of("<>"), Operator::from)(input)
}

fn condition(input: &str) -> IResult<&str, Condition> {
    let (input, (attr, op, rating, _, verdict)) =
        tuple((part_attr, operator, part_rating, nom_char(':'), verdict))(input)?;

    Ok((
        input,
        Condition {
            attribute: attr,
            operator: op,
            rating,
            result: verdict,
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

pub(crate) fn part(input: &str) -> Part {
    let (_, attrs) = delimited(
        nom_char('{'),
        separated_list1(
            nom_char(','),
            separated_pair(part_attr, nom_char('='), part_rating),
        ),
        nom_char('}'),
    )(input)
    .unwrap();

    Part::new(&attrs)
}

pub(crate) fn workflow(input: &str) -> Workflow {
    let (input, name) = workflow_name(input).unwrap();

    let (_, (conditions, catchall)) = delimited(
        nom_char('{'),
        tuple((many1(terminated(condition, nom_char(','))), verdict)),
        nom_char('}'),
    )(input)
    .unwrap();

    Workflow {
        name,
        conditions,
        catchall,
    }
}
