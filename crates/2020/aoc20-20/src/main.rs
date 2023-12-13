use std::collections::{HashMap, HashSet};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

const MONSTER_COORDS: [(usize, usize); 15] = [
    (0, 18),
    (1, 0),
    (1, 5),
    (1, 6),
    (1, 11),
    (1, 12),
    (1, 17),
    (1, 18),
    (1, 19),
    (2, 1),
    (2, 4),
    (2, 7),
    (2, 10),
    (2, 13),
    (2, 16),
];

#[derive(Copy, Clone)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

const ALL_DIRS: [Direction; 4] = [
    Direction::Top,
    Direction::Bottom,
    Direction::Left,
    Direction::Right,
];

struct Tile {
    id: usize,
    data: Vec<Vec<char>>,
    transform_count: usize,
}

impl Tile {
    fn new(id: usize, data: Vec<Vec<char>>) -> Self {
        Self {
            id,
            data,
            transform_count: 0,
        }
    }

    fn flip(&mut self) {
        for row in &mut self.data {
            row.reverse();
        }
    }

    fn column(&self, n: usize) -> Vec<char> {
        self.data.iter().map(|row| row[n]).collect()
    }

    fn rotate_right(&mut self) {
        let num_cols = self.data[0].len();

        self.data = (0..num_cols)
            .map(|col| self.column(col).into_iter().rev().collect())
            .collect();
    }

    fn get_edge(&self, direction: Direction) -> Vec<char> {
        match direction {
            Direction::Top => self.data[0].clone(),
            Direction::Bottom => self.data.last().unwrap().clone(),
            Direction::Left => self.column(0),
            Direction::Right => self.column(self.data[0].len() - 1),
        }
    }

    fn all_edges(&self) -> Vec<Vec<char>> {
        let mut edges: Vec<Vec<char>> = ALL_DIRS.iter().map(|dir| self.get_edge(*dir)).collect();

        let mut reversed_edges = edges.clone();

        reversed_edges.iter_mut().for_each(|edge| edge.reverse());

        edges.append(&mut reversed_edges);
        edges
    }

    fn transform(&mut self) {
        self.rotate_right();

        self.transform_count = (self.transform_count + 1) % 4;

        if self.transform_count == 0 {
            self.flip();
        }
    }

    fn transform_to_match(&mut self, direction: Direction, border: &[char]) -> bool {
        for _ in 0..9 {
            if self.get_edge(direction) == border {
                return true;
            }
            self.transform();
        }

        false
    }

    fn remove_borders(&mut self) {
        let nr = self.data.len();
        let nc = self.data[0].len();

        self.data = self.data[1..nr - 1]
            .iter()
            .map(|row| row[1..nc - 1].to_vec())
            .collect();
    }

    fn mark_monster_at_offset(&mut self, r: usize, c: usize) -> bool {
        let hash_count = MONSTER_COORDS
            .iter()
            .take_while(|(x, y)| self.data.get(r + x).and_then(|row| row.get(c + y)) == Some(&'#'))
            .count();

        if hash_count == MONSTER_COORDS.len() {
            for (x, y) in &MONSTER_COORDS {
                self.data[r + x][c + y] = 'O';
            }
            true
        } else {
            false
        }
    }

    fn find_and_mark_monsters(&mut self) {
        let mut found = false;

        for _ in 0..9 {
            for row in 0..self.data.len() {
                for col in 0..self.data[row].len() {
                    if self.mark_monster_at_offset(row, col) {
                        found = true;
                    }
                }
            }

            if found {
                return;
            }
            self.transform();
        }
    }

    fn count_hashes(&self) -> usize {
        self.data
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&c| c == &'#')
            .count()
    }
}

fn make_tiles() -> Vec<Tile> {
    INPUT
        .split("\n\n")
        .map(|tile_spec| {
            let mut line_iter = tile_spec.lines();

            let first_line = line_iter.next().unwrap();

            let colon_idx = first_line.find(':').unwrap();

            let id: usize = first_line[5..colon_idx].parse().unwrap();

            let data: Vec<Vec<char>> = line_iter.map(|line| line.chars().collect()).collect();

            Tile::new(id, data)
        })
        .collect()
}

fn part1() -> usize {
    let tiles = make_tiles();

    let mut border_to_tile_map: HashMap<Vec<char>, Vec<usize>> = HashMap::new();

    for tile in &tiles {
        for edge in tile.all_edges() {
            border_to_tile_map
                .entry(edge.clone())
                .or_default()
                .push(tile.id);
        }
    }

    let edge_tile_ids: HashSet<_> = border_to_tile_map
        .values()
        .filter(|tile_ids| tile_ids.len() == 1)
        .flatten()
        .collect();

    edge_tile_ids
        .into_iter()
        .filter(|&&tile_id| {
            border_to_tile_map
                .values()
                .filter(|&tile_list| tile_list == &vec![tile_id])
                .count()
                == 4
        })
        .product()
}

fn find_top_left(tiles: &mut [Tile]) -> usize {
    let mut border_to_tile_map: HashMap<Vec<char>, Vec<usize>> = HashMap::new();

    for tile in tiles.iter() {
        for edge in tile.all_edges() {
            border_to_tile_map
                .entry(edge.clone())
                .or_default()
                .push(tile.id);
        }
    }

    tiles
        .iter_mut()
        .position(|tile| {
            for _ in 0..9 {
                let top = tile.get_edge(Direction::Top);
                let left = tile.get_edge(Direction::Left);

                let is_top_unique = border_to_tile_map
                    .get(&top)
                    .map_or(false, |ids| ids.len() == 1);
                let is_left_unique = border_to_tile_map
                    .get(&left)
                    .map_or(false, |ids| ids.len() == 1);
                if is_top_unique && is_left_unique {
                    return true;
                }

                tile.transform();
            }

            false
        })
        .unwrap()
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn part2() -> usize {
    let mut tiles = make_tiles();
    let side_length = (tiles.len() as f64).sqrt() as usize;

    let top_left = find_top_left(&mut tiles);

    let mut image: Vec<Vec<Tile>> = vec![];

    let mut cur_row = vec![tiles.swap_remove(top_left)];

    while !tiles.is_empty() {
        let border_to_match;
        let direction_to_match;
        let tile_to_match;

        if cur_row.is_empty() {
            tile_to_match = image.last().unwrap().first().unwrap();
            border_to_match = tile_to_match.get_edge(Direction::Bottom);
            direction_to_match = Direction::Top;
        } else {
            tile_to_match = cur_row.last().unwrap();
            border_to_match = tile_to_match.get_edge(Direction::Right);
            direction_to_match = Direction::Left;
        }

        let matching_tile = tiles
            .iter_mut()
            .position(|tile| tile.transform_to_match(direction_to_match, &border_to_match))
            .unwrap();

        cur_row.push(tiles.swap_remove(matching_tile));

        if cur_row.len() == side_length {
            image.push(cur_row);
            cur_row = vec![];
        }
    }

    let mut big_tile = combine_tiles(image);

    big_tile.find_and_mark_monsters();

    big_tile.count_hashes()
}

fn combine_tiles(mut tiles: Vec<Vec<Tile>>) -> Tile {
    for row in &mut tiles {
        for tile in row {
            tile.remove_borders();
        }
    }

    let mut big_tile = vec![];

    for tile_row in &tiles {
        for row in 0..tile_row[0].data.len() {
            let mut cur_row: Vec<char> = vec![];
            for tile in tile_row {
                cur_row.extend(tile.data[row].iter());
            }
            big_tile.push(cur_row);
        }
    }

    Tile::new(0, big_tile)
}

fn main() {
    let start = Instant::now();
    println!("part 1: {}", part1());
    println!("part 1 took {}ms", start.elapsed().as_millis());
    let start = Instant::now();
    println!("part 2: {}", part2());
    println!("part 2 took {}ms", start.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 29125888761511);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2219);
    }

    #[test]
    fn test_tile_rotation() {
        let tile_data = vec![vec!['#', '.'], vec!['.', '#'], vec!['#', '#']];
        let mut tile = Tile::new(10, tile_data);

        assert_eq!(tile.get_edge(Direction::Top), vec!['#', '.']);
        assert_eq!(tile.get_edge(Direction::Bottom), vec!['#', '#']);
        assert_eq!(tile.get_edge(Direction::Left), vec!['#', '.', '#']);
        assert_eq!(tile.get_edge(Direction::Right), vec!['.', '#', '#']);

        tile.rotate_right();

        assert_eq!(tile.get_edge(Direction::Top), vec!['#', '.', '#']);
        assert_eq!(tile.get_edge(Direction::Bottom), vec!['#', '#', '.']);
        assert_eq!(tile.get_edge(Direction::Left), vec!['#', '#']);
        assert_eq!(tile.get_edge(Direction::Right), vec!['#', '.']);

        tile.flip();

        assert_eq!(tile.get_edge(Direction::Top), vec!['#', '.', '#']);
        assert_eq!(tile.get_edge(Direction::Bottom), vec!['.', '#', '#']);
        assert_eq!(tile.get_edge(Direction::Left), vec!['#', '.']);
        assert_eq!(tile.get_edge(Direction::Right), vec!['#', '#']);
    }

    #[test]
    fn test_remove_borders() {
        let tile_data = vec![
            vec!['#', '#', '#'],
            vec!['#', '.', '#'],
            vec!['#', '#', '#'],
        ];
        let mut tile = Tile::new(10, tile_data);

        tile.remove_borders();

        assert_eq!(tile.data, vec![vec!['.']]);
    }

    #[test]
    fn test_combine1() {
        let t1 = Tile::new(
            0,
            vec![
                vec!['#', '#', '#'],
                vec!['#', '.', '#'],
                vec!['#', '#', '#'],
            ],
        );

        let t2 = Tile::new(
            0,
            vec![
                vec!['#', '#', '#'],
                vec!['#', '.', '#'],
                vec!['#', '#', '#'],
            ],
        );

        let t3 = Tile::new(
            0,
            vec![
                vec!['#', '#', '#'],
                vec!['#', '.', '#'],
                vec!['#', '#', '#'],
            ],
        );

        let t4 = Tile::new(
            0,
            vec![
                vec!['#', '#', '#'],
                vec!['#', '.', '#'],
                vec!['#', '#', '#'],
            ],
        );

        assert_eq!(
            combine_tiles(vec![vec![t1, t2], vec![t3, t4],]).data,
            vec![vec!['.', '.'], vec!['.', '.']]
        );
    }

    #[test]
    fn test_combine2() {
        let t1 = Tile::new(
            0,
            vec![
                vec!['#', '#', '#'],
                vec!['#', '#', '#'],
                vec!['#', '#', '#'],
            ],
        );

        let t2 = Tile::new(
            0,
            vec![
                vec!['#', '#', '#'],
                vec!['#', '.', '#'],
                vec!['#', '#', '#'],
            ],
        );

        let t3 = Tile::new(
            0,
            vec![
                vec!['#', '#', '#'],
                vec!['#', '.', '#'],
                vec!['#', '#', '#'],
            ],
        );

        let t4 = Tile::new(
            0,
            vec![
                vec!['#', '#', '#'],
                vec!['#', '#', '#'],
                vec!['#', '#', '#'],
            ],
        );

        assert_eq!(
            combine_tiles(vec![vec![t1, t2], vec![t3, t4]]).data,
            vec![vec!['#', '.'], vec!['.', '#']]
        );
    }
}
