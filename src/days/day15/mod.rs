// https://adventofcode.com/2024/day/15

use std::collections::HashSet;

use crate::{Distance, Position};

pub fn solution_part1(input: &str) -> usize {
    let warehouse_map = WarehouseMap::from_input(input);
    warehouse_map.move_and_sum_gps_coordinates()
}

pub fn solution_part2(input: &str) -> usize {
    let warehouse_map = WarehouseMap::from_input(input);
    warehouse_map.twice_as_wide_move_and_sum_gps_coordinates()
}

struct WarehouseMap {
    width: usize,
    height: usize,
    start: Position,
    boxes: Vec<Position>,
    walls: Vec<Position>,
    moves: Vec<Direction>,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl WarehouseMap {
    fn from_input(input: &str) -> Self {
        let (map_str, moves_str) = input.split_once("\n\n").unwrap();
        let map_lines = map_str.lines().collect::<Vec<&str>>();

        let width = map_lines[0].len() - 2;
        let height = map_lines.len() - 2;

        let mut boxes = vec![];
        let mut walls = vec![];
        let mut start = None;

        for i in 1..map_lines.len() - 1 {
            let y = i - 1;
            let tiles = map_lines[i].chars().collect::<Vec<char>>();

            for j in 1..tiles.len() - 1 {
                let x = j - 1;

                match tiles[j] {
                    'O' => boxes.push(Position(x, y)),
                    '#' => walls.push(Position(x, y)),
                    '@' => start = Some(Position(x, y)),
                    _ => {}
                }
            }
        }

        let moves = moves_str
            .chars()
            .filter_map(|c| match c {
                '>' => Some(Direction::Right),
                '^' => Some(Direction::Up),
                '<' => Some(Direction::Left),
                'v' => Some(Direction::Down),
                _ => None,
            })
            .collect::<Vec<_>>();

        Self {
            width,
            height,
            boxes,
            walls,
            start: start.unwrap(),
            moves,
        }
    }

    fn move_and_sum_gps_coordinates(self) -> usize {
        let width = self.width;
        let height = self.height;
        let mut boxes: HashSet<Position> = HashSet::from_iter(self.boxes.into_iter());
        let walls: HashSet<Position> = HashSet::from_iter(self.walls.into_iter());
        let mut current_pos = self.start.clone();

        let mut boxes_to_move: Vec<Position> = vec![];
        for movement in self.moves.iter() {
            let distance = direction_to_distance(movement);

            if let Some(tentative_pos) = current_pos.move_by(&distance, width, height) {
                boxes_to_move.clear();

                loop {
                    let next_pos = match boxes_to_move
                        .last()
                        .map(|b| b.move_by(&distance, width, height))
                    {
                        Some(None) => {
                            break;
                        }
                        None => tentative_pos.clone(),
                        Some(Some(pos)) => pos,
                    };

                    if boxes.contains(&next_pos) {
                        boxes_to_move.push(next_pos);
                        continue;
                    }

                    if walls.contains(&next_pos) {
                        break;
                    }

                    if boxes_to_move.len() > 0 {
                        boxes.remove(&boxes_to_move[0]);
                        boxes.insert(next_pos.clone());
                    }
                    current_pos = tentative_pos;
                    break;
                }
            }
        }

        boxes
            .into_iter()
            .map(|pos| pos.0 + 1 + (pos.1 + 1) * 100)
            .sum()
    }

    fn twice_as_wide_move_and_sum_gps_coordinates(self) -> usize {
        let width = self.width * 2;
        let height = self.height;

        let mut boxes: HashSet<Box> = HashSet::from_iter(self.boxes.iter().map(|b| Box {
            left: Position(b.0 * 2, b.1),
            right: Position(b.0 * 2 + 1, b.1),
        }));
        let walls: HashSet<Position> =
            HashSet::from_iter(self.walls.into_iter().flat_map(|wall| {
                vec![
                    Position(wall.0 * 2, wall.1),
                    Position(wall.0 * 2 + 1, wall.1),
                ]
            }));
        let mut current_pos = Position(self.start.0 * 2, self.start.1);

        for movement in self.moves.iter() {
            let distance = direction_to_distance(movement);

            let mut boxes_to_move: Vec<Box> = vec![];
            let mut current_layer_of_boxes_to_move: Vec<Box> = vec![];

            if let Some(tentative_pos) = current_pos.move_by(&distance, width, height) {
                loop {
                    let next_positions = if current_layer_of_boxes_to_move.len() != 0 {
                        current_layer_of_boxes_to_move
                            .iter()
                            .filter_map(|box_to_move| box_to_move.move_by(&distance, width, height))
                            .flat_map(|new_box| vec![new_box.left, new_box.right])
                            .collect::<Vec<_>>()
                    } else {
                        vec![tentative_pos.clone()]
                    };

                    if next_positions.len() == 0 {
                        break;
                    }

                    current_layer_of_boxes_to_move.clear();

                    let mut encountered_wall = false;
                    let mut encountered_box = false;
                    for next_pos in next_positions {
                        if let Some(b) = boxes.iter().find(|b| {
                            (b.left == next_pos || b.right == next_pos)
                                && !boxes_to_move.contains(b)
                        }) {
                            current_layer_of_boxes_to_move.push(b.clone());
                            boxes_to_move.push(b.clone());
                            encountered_box = true;
                            continue;
                        }

                        if walls.contains(&next_pos) {
                            encountered_wall = true;
                            break;
                        }
                    }

                    if encountered_wall {
                        break;
                    }

                    if encountered_box {
                        continue;
                    }

                    for b in boxes_to_move.iter() {
                        boxes.remove(&b);
                    }

                    for b in boxes_to_move {
                        boxes.insert(b.move_by(&distance, width, height).unwrap());
                    }

                    current_pos = tentative_pos;
                    break;
                }
            }
        }

        boxes
            .into_iter()
            .map(|pos| pos.left.0 + 2 + (pos.left.1 + 1) * 100)
            .sum()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Box {
    left: Position,
    right: Position,
}

impl Box {
    fn move_by(&self, distance: &Distance, width: usize, height: usize) -> Option<Box> {
        match (
            self.left.move_by(distance, width, height),
            self.right.move_by(distance, width, height),
        ) {
            (Some(left), Some(right)) => Some(Box { left, right }),
            _ => None,
        }
    }
}

fn direction_to_distance(dir: &Direction) -> Distance {
    match dir {
        Direction::Up => Distance(0, -1),
        Direction::Down => Distance(0, 1),
        Direction::Left => Distance(-1, 0),
        Direction::Right => Distance(1, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OTHER_EXAMPLE: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    const PRE_EXAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_pre_example() {
        assert_eq!(solution_part1(PRE_EXAMPLE), 2028);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 10092);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 1475249);
    }

    #[test]
    fn test_part2_other_example() {
        assert_eq!(solution_part2(OTHER_EXAMPLE), 618);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 9021);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 1509724);
    }
}
