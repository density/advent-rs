use std::cmp::Ordering;
use std::str::FromStr;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::recognize;
use nom::multi::{many1, separated_list0};
use nom::sequence::delimited;
use nom::{Finish, IResult};

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn decimal(input: &str) -> IResult<&str, Element> {
    let (input, numbers) = recognize(many1(one_of("0123456789")))(input)?;

    Ok((input, Element::Integer(numbers.parse().unwrap())))
}

fn list(input: &str) -> IResult<&str, Element> {
    let (input, elems) = delimited(
        tag("["),
        separated_list0(tag(","), alt((decimal, list))),
        tag("]"),
    )(input)?;

    Ok((input, Element::List(elems)))
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Element {
    Integer(u64),
    List(Vec<Element>),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Element::Integer(left), Element::Integer(right)) => left.cmp(right),
            (Element::List(left), Element::List(right)) => {
                for (l, r) in left.iter().zip(right.iter()) {
                    if l.cmp(r) != Ordering::Equal {
                        return l.cmp(r);
                    }
                }

                left.len().cmp(&right.len())
            }
            (Element::Integer(_), Element::List(_)) => Element::List(vec![self.clone()]).cmp(other),
            (Element::List(_), Element::Integer(_)) => {
                self.cmp(&Element::List(vec![other.clone()]))
            }
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Element {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match list(s).finish() {
            Ok((_, elem)) => Ok(elem),
            Err(_) => Err(()),
        }
    }
}

fn element_iter(s: &str) -> impl Iterator<Item = Element> + '_ {
    s.lines().filter_map(|l| l.parse().ok())
}

fn part1() -> usize {
    (1..)
        .zip(element_iter(INPUT).tuples())
        .map(|(i, (left, right))| if left < right { i } else { 0 })
        .sum()
}

fn part2() -> usize {
    let divider2: Element = "[[2]]".parse().unwrap();
    let divider6: Element = "[[6]]".parse().unwrap();

    let mut elements: Vec<_> = element_iter(INPUT).collect();
    elements.push(divider2.clone());
    elements.push(divider6.clone());
    elements.sort();

    (1..)
        .zip(elements.into_iter())
        .filter_map(|(i, element)| {
            if element == divider2 || element == divider6 {
                Some(i)
            } else {
                None
            }
        })
        .take(2)
        .product()
}

fn main() {
    timed_run(1, part1);
    timed_run(2, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp() {
        let left: Element = "[1,1,3,1,1]".parse().unwrap();
        let right: Element = "[1,1,5,1,1]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Less);
        assert_eq!(left.cmp(&left), Ordering::Equal);
        assert_eq!(right.cmp(&right), Ordering::Equal);
        assert_eq!(right.cmp(&left), Ordering::Greater);

        let left: Element = "[[1],[2,3,4]]".parse().unwrap();
        let right: Element = "[[1],4]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Less);
        assert_eq!(left.cmp(&left), Ordering::Equal);
        assert_eq!(right.cmp(&right), Ordering::Equal);
        assert_eq!(right.cmp(&left), Ordering::Greater);

        let left: Element = "[9]".parse().unwrap();
        let right: Element = "[[8,7,6]]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Greater);
        assert_eq!(left.cmp(&left), Ordering::Equal);
        assert_eq!(right.cmp(&right), Ordering::Equal);
        assert_eq!(right.cmp(&left), Ordering::Less);

        let left: Element = "[[4,4],4,4]".parse().unwrap();
        let right: Element = "[[4,4],4,4,4]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Less);
        assert_eq!(left.cmp(&left), Ordering::Equal);
        assert_eq!(right.cmp(&right), Ordering::Equal);
        assert_eq!(right.cmp(&left), Ordering::Greater);

        let left: Element = "[]".parse().unwrap();
        let right: Element = "[3]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Less);
        assert_eq!(left.cmp(&left), Ordering::Equal);
        assert_eq!(right.cmp(&right), Ordering::Equal);
        assert_eq!(right.cmp(&left), Ordering::Greater);

        let left: Element = "[[[]]]".parse().unwrap();
        let right: Element = "[[]]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Greater);
        assert_eq!(left.cmp(&left), Ordering::Equal);
        assert_eq!(right.cmp(&right), Ordering::Equal);
        assert_eq!(right.cmp(&left), Ordering::Less);

        let left: Element = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse().unwrap();
        let right: Element = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Greater);
        assert_eq!(left.cmp(&left), Ordering::Equal);
        assert_eq!(right.cmp(&right), Ordering::Equal);
        assert_eq!(right.cmp(&left), Ordering::Less);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 6420);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 22000);
    }
}
