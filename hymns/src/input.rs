use std::fmt::Debug;
use std::str::FromStr;

use num_traits::PrimInt;

pub fn parse_iterable<'a, T>(
    iter: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = T> + 'a
where
    T: PrimInt + FromStr,
    <T as FromStr>::Err: Debug,
{
    iter.map(|elem| elem.parse::<T>().unwrap())
}

// TODO: Once `Pattern` is stablized, consolidate these
pub fn parse_str_delimited_numbers<'a, T: 'a>(
    s: &'a str,
    delim: &'a str,
) -> impl Iterator<Item = T> + 'a
where
    T: PrimInt + FromStr,
    <T as FromStr>::Err: Debug,
{
    parse_iterable(s.split(delim))
}

pub fn parse_char_delimited_numbers<'a, T: 'a>(
    s: &'a str,
    delim: char,
) -> impl Iterator<Item = T> + 'a
where
    T: PrimInt + FromStr,
    <T as FromStr>::Err: Debug,
{
    parse_iterable(s.split(delim))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_iterable() {
        assert_eq!(parse_iterable([].into_iter()).collect::<Vec<u32>>(), vec![]);
        assert_eq!(
            parse_iterable(["1", "2", "3", "4"].into_iter()).collect::<Vec<u32>>(),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            parse_iterable(["100", "-200", "300", "-400"].into_iter()).collect::<Vec<i32>>(),
            vec![100, -200, 300, -400]
        );
    }

    #[test]
    fn test_parse_str_delimited_numbers() {
        assert_eq!(
            parse_str_delimited_numbers("1!#2!#3!#4", "!#").collect::<Vec<u32>>(),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            parse_str_delimited_numbers("1!#-2!#3!#-400", "!#").collect::<Vec<i32>>(),
            vec![1, -2, 3, -400]
        );
    }

    #[test]
    fn test_parse_char_delimited_numbers() {
        assert_eq!(
            parse_char_delimited_numbers("1,2,3,4", ',').collect::<Vec<u32>>(),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            parse_char_delimited_numbers("1,-2,3,-400", ',').collect::<Vec<i32>>(),
            vec![1, -2, 3, -400]
        );
    }
}
