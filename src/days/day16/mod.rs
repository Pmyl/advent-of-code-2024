// https://adventofcode.com/2024/day/16

use std::collections::{btree_map::Entry, BTreeMap, HashSet};

use crate::{Direction, Distance, Position};

pub fn solution_part1(input: &str) -> usize {
    let maze = ReindeerMaze::from_input(input);
    maze.lowest_score_possible()
}

pub fn solution_part2(input: &str) -> usize {
    let maze = ReindeerMaze::from_input(input);
    maze.count_tiles_of_lowest_score_possible()
}

struct ReindeerMaze {
    maze: Vec<Vec<Cell>>,
    start: Position,
    end: Position,
}

#[derive(Clone)]
enum Cell {
    Free,
    Wall,
    Visited([usize; 4]),
}

impl ReindeerMaze {
    fn from_input(input: &str) -> Self {
        let mut start = None;
        let mut end = None;

        let maze = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Cell::Wall,
                        '.' => Cell::Free,
                        'S' => {
                            start = Some(Position(x, y));
                            Cell::Free
                        }
                        'E' => {
                            end = Some(Position(x, y));
                            Cell::Free
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Self {
            maze,
            start: start.expect("should have a start"),
            end: end.expect("should have an end"),
        }
    }

    fn lowest_score_possible(&self) -> usize {
        self.all_lowest_score_paths().0
    }

    fn count_tiles_of_lowest_score_possible(&self) -> usize {
        HashSet::<Position>::from_iter(self.all_lowest_score_paths().1.into_iter().flatten()).len()
    }

    fn all_lowest_score_paths(&self) -> (usize, Vec<Vec<Position>>) {
        let width = self.maze[0].len();
        let height = self.maze.len();

        let mut min_score: (usize, Vec<Vec<Position>>) = (usize::MAX, vec![]);

        let mut maze = self.maze.clone();
        let mut cases = BTreeMap::<usize, Vec<(Position, Direction, Vec<Position>)>>::new();
        cases.insert(
            0,
            vec![(
                self.start.clone(),
                Direction::Right,
                vec![self.start.clone()],
            )],
        );

        while let Some((score, sub_cases)) = cases.pop_first() {
            for case in sub_cases {
                if let Cell::Visited(mut scores) = maze[case.0 .1][case.0 .0] {
                    scores[direction_to_index(&case.1)] = score;
                } else {
                    let mut scores = [usize::MAX; 4];
                    scores[direction_to_index(&case.1)] = score;
                    maze[case.0 .1][case.0 .0] = Cell::Visited(scores);
                }

                let forward_position = case
                    .0
                    .move_by(&Distance::from_direction(&case.1), width, height)
                    .expect("the map has walls all around so it won't ever go out");
                let mut forward_path = case.2.clone();
                forward_path.push(forward_position.clone());
                let case_forward = (score + 1, (forward_position, case.1.clone(), forward_path));

                let turn_left = case.1.turn_left();
                let left_position = case
                    .0
                    .move_by(&Distance::from_direction(&turn_left), width, height)
                    .expect("the map has walls all around so it won't ever go out");
                let mut left_path = case.2.clone();
                left_path.push(left_position.clone());
                let case_left = (score + 1001, (left_position, turn_left, left_path));

                let turn_right = case.1.turn_right();
                let right_position = case
                    .0
                    .move_by(&Distance::from_direction(&turn_right), width, height)
                    .expect("the map has walls all around so it won't ever go out");
                let mut right_path = case.2.clone();
                right_path.push(right_position.clone());
                let case_right = (score + 1001, (right_position, turn_right, right_path));

                for next_case in [case_forward, case_left, case_right] {
                    let next_score = next_case.0;
                    let next_position = &next_case.1 .0;
                    let next_direction = &next_case.1 .1;
                    let next_path = &next_case.1 .2;

                    if next_score > min_score.0 {
                        continue;
                    }

                    if next_position == &self.end {
                        if next_score == min_score.0 {
                            min_score.1.push(next_path.to_owned());
                        } else {
                            min_score = (next_score, vec![next_path.to_owned()]);
                        }
                        continue;
                    }

                    if matches!(maze[next_position.1][next_position.0], Cell::Visited(scores) if scores[direction_to_index(next_direction)] > next_score)
                        || matches!(maze[next_position.1][next_position.0], Cell::Free)
                    {
                        match cases.entry(next_score) {
                            Entry::Vacant(vacant_entry) => {
                                vacant_entry.insert(vec![next_case.1]);
                            }
                            Entry::Occupied(mut occupied_entry) => {
                                occupied_entry.get_mut().push(next_case.1);
                            }
                        }
                    }
                }
            }
        }

        min_score
    }
}

fn direction_to_index(dir: &Direction) -> usize {
    match dir {
        Direction::Up => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 7036);
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(solution_part1(EXAMPLE2), 11048);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 89460);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 45);
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(solution_part2(EXAMPLE2), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 504);
    }
}
