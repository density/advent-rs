use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");
const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

type GridRow = Vec<char>;
type Grid = Vec<GridRow>;

fn read_grid() -> Grid {
    INPUT.lines().map(|line| line.chars().collect()).collect()
}

fn step(grid: Grid, extended: bool) -> (Grid, bool) {
    let mut new_grid = grid.clone();
    let mut changed = false;

    let neighbor_limit = if extended { 5 } else { 4 };

    for (r, row) in grid.iter().enumerate() {
        for (c, item) in row.iter().enumerate() {
            if matches!(item, 'L' | '#') {
                let neighbors = count_neighbors(&grid, r, c, extended);

                if *item == 'L' {
                    if neighbors == 0 {
                        new_grid[r][c] = '#';
                        changed = true;
                    }
                } else if neighbors >= neighbor_limit {
                    new_grid[r][c] = 'L';
                    changed = true;
                }
            }
        }
    }

    (new_grid, changed)
}

fn count_neighbors(grid: &[GridRow], r: usize, c: usize, extended: bool) -> u64 {
    let mut total = 0;

    for (row_off, col_off) in &OFFSETS {
        let mut dr = *row_off;
        let mut dc = *col_off;

        loop {
            let new_r = ((r as isize) + dr) as usize;
            let new_c = ((c as isize) + dc) as usize;

            match grid.get(new_r).and_then(|row| row.get(new_c)) {
                Some('#') => {
                    total += 1;
                    break;
                }
                Some('L') | None => {
                    break;
                }
                Some('.') => (),
                _ => unreachable!()
            }

            if extended {
                dr += row_off;
                dc += col_off;
            } else {
                break;
            }
        }
    }

    total
}

fn get_neighbors_at_steady_state(extended: bool) -> usize {
    let mut grid = read_grid();

    loop {
        let (new, changed) = step(grid, extended);

        if !changed {
            return new
                .iter()
                .map(|row| row.iter().filter(|c| **c == '#').count())
                .sum();
        }

        grid = new;
    }
}

fn part1() -> usize {
    get_neighbors_at_steady_state(false)
}

fn part2() -> usize {
    get_neighbors_at_steady_state(true)
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
        assert_eq!(part1(), 2113);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1865);
    }
}
