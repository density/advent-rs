use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn reflect_points(axis: char, x_or_y: usize, points: &mut HashSet<Point2<usize>>) {
    let mut points_to_move = vec![];

    for point in points.iter() {
        if axis == 'y' {
            if point.y > x_or_y {
                points_to_move.push(*point);
            }
        } else if point.x > x_or_y {
            points_to_move.push(*point);
        }
    }

    for point in points_to_move {
        if axis == 'y' {
            let distance_from_axis = point.y - x_or_y;
            points.insert(p2!(point.x, x_or_y - distance_from_axis));
        } else {
            let distance_from_axis = point.x - x_or_y;
            points.insert(p2!(x_or_y - distance_from_axis, point.y));
        }
        points.remove(&point);
    }
}

fn build_points(line_iter: &mut impl Iterator<Item = &'static str>) -> HashSet<Point2<usize>> {
    line_iter
        .map(|line| {
            let mut nums = line.split(',');
            let x: usize = nums.next().unwrap().parse().unwrap();
            let y: usize = nums.next().unwrap().parse().unwrap();

            p2!(x, y)
        })
        .collect()
}

fn build_folds(
    line_iter: &mut impl Iterator<Item = &'static str>,
) -> impl Iterator<Item = (char, usize)> + '_ {
    line_iter.map(|line| {
        let bytes = line.as_bytes();

        let axis = char::from(bytes[11]);
        let x_or_y = std::str::from_utf8(&bytes[13..]).unwrap().parse().unwrap();

        (axis, x_or_y)
    })
}

fn part1() -> usize {
    let mut line_iter = INPUT.lines();

    let mut points = build_points(&mut line_iter.by_ref().take_while(|&line| !line.is_empty()));

    let (reflection_axis, x_or_y) = build_folds(&mut line_iter).next().unwrap();

    reflect_points(reflection_axis, x_or_y, &mut points);

    points.len()
}

fn part2() -> String {
    let mut line_iter = INPUT.lines();

    let mut points = build_points(&mut line_iter.by_ref().take_while(|&line| !line.is_empty()));

    for (reflection_axis, x_or_y) in build_folds(&mut line_iter) {
        reflect_points(reflection_axis, x_or_y, &mut points);
    }

    let (min_x, max_x, min_y, max_y) = points.iter().fold(
        (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
        |(min_x, max_x, min_y, max_y), p| {
            (
                min_x.min(p.x),
                max_x.max(p.x),
                min_y.min(p.y),
                max_y.max(p.y),
            )
        },
    );

    let mut result = String::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.contains(&p2!(x, y)) {
                result.push('#');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    result
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
        assert_eq!(part1(), 842);
    }

    #[test]
    fn test_part2() {
        let mut expected = r"###  #### #  # ###   ##    ## #### #  #
#  # #    # #  #  # #  #    #    # #  #
###  ###  ##   #  # #       #   #  #  #
#  # #    # #  ###  #       #  #   #  #
#  # #    # #  # #  #  # #  # #    #  #
###  #    #  # #  #  ##   ##  ####  ## "
            .to_string();
        // This is just to avoid ending a source code line with whitespace
        expected.push('\n');

        assert_eq!(part2(), expected);
    }
}
