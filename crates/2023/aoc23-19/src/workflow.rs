use crate::workflow::Attribute::{Aero, Cool, Music, Shine};
use crate::workflow::Operator::{GT, LT};
use crate::workflow::Verdict::{Accept, Reject};
use std::cmp::Ordering;
use std::ops::RangeInclusive;

pub(crate) type AttributeRanges = [RangeInclusive<u64>; 4];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Attribute {
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

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Part {
    pub(crate) attributes: [u64; 4],
}

impl Part {
    pub(crate) fn new(attributes: &[(Attribute, u64)]) -> Self {
        let mut attrs = [0; 4];

        for (attr, rating) in attributes {
            attrs[*attr as usize] = *rating;
        }

        Self { attributes: attrs }
    }

    pub(crate) fn get_attribute(&self, attribute: Attribute) -> u64 {
        self.attributes[attribute as usize]
    }
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum Verdict<'a> {
    Accept,
    Reject,
    Jump(&'a str),
}

impl From<char> for Verdict<'_> {
    fn from(value: char) -> Self {
        match value {
            'A' => Accept,
            'R' => Reject,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Operator {
    GT,
    LT,
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '>' => GT,
            '<' => LT,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Condition<'a> {
    pub(crate) attribute: Attribute,
    pub(crate) operator: Operator,
    pub(crate) rating: u64,
    pub(crate) result: Verdict<'a>,
}

impl<'a> Condition<'a> {
    pub(crate) fn evaluate(&self, part: &Part) -> Option<&Verdict<'a>> {
        let comparison_result = part.get_attribute(self.attribute).cmp(&self.rating);

        if (comparison_result == Ordering::Less && self.operator == LT)
            || (comparison_result == Ordering::Greater && self.operator == GT)
        {
            Some(&self.result)
        } else {
            None
        }
    }

    pub(crate) fn split_ranges(
        &self,
        ranges: &AttributeRanges,
    ) -> ((AttributeRanges, &Verdict<'a>), AttributeRanges) {
        let mut below = ranges.clone();
        let mut above = ranges.clone();

        let cutoff = if self.operator == LT {
            self.rating.saturating_sub(1)
        } else {
            self.rating
        };

        let cur_range = &ranges[self.attribute as usize];
        below[self.attribute as usize] = *cur_range.start()..=cutoff;
        above[self.attribute as usize] = (cutoff + 1)..=*cur_range.end();

        match self.operator {
            GT => ((above, &self.result), below),
            LT => ((below, &self.result), above),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Workflow<'a> {
    pub(crate) name: &'a str,
    pub(crate) conditions: Vec<Condition<'a>>,
    pub(crate) catchall: Verdict<'a>,
}

impl<'a> Workflow<'a> {
    pub(crate) fn evaluate(&self, part: &Part) -> &Verdict<'a> {
        for condition in &self.conditions {
            if let Some(result) = condition.evaluate(part) {
                return result;
            }
        }

        &self.catchall
    }
}
