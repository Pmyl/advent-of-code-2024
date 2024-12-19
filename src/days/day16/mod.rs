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
    Visited([usize; 4], Vec<usize>),
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
        self.all_lowest_score_paths().1
    }

    fn all_lowest_score_paths(&self) -> (usize, usize) {
        let width = self.maze[0].len();
        let height = self.maze.len();

        let mut min_score: (usize, Vec<usize>) = (usize::MAX, vec![]);

        let mut maze = self.maze.clone();

        struct Case {
            pos: Position,
            dir: Direction,
            ids: Vec<usize>,
        }

        let mut cases = BTreeMap::<usize, Vec<Case>>::new();

        let mut current_id = 0;

        cases.insert(
            0,
            vec![Case {
                pos: self.start.clone(),
                dir: Direction::Right,
                ids: vec![current_id],
            }],
        );

        current_id += 1;

        while let Some((score, sub_cases)) = cases.pop_first() {
            for case in sub_cases {
                if let Cell::Visited(mut scores, ref mut ids) = maze[case.pos.1][case.pos.0] {
                    scores[direction_to_index(&case.dir)] = score;
                    ids.push(case.ids[0]);
                } else {
                    let mut scores = [usize::MAX; 4];
                    scores[direction_to_index(&case.dir)] = score;
                    maze[case.pos.1][case.pos.0] = Cell::Visited(scores, vec![case.ids[0]]);
                }

                let turn_left = case.dir.turn_left();
                let left_position = case
                    .pos
                    .move_by(&Distance::from_direction(&turn_left), width, height)
                    .expect("the map has walls all around so it won't ever go out");
                let mut left_ids = vec![current_id];
                left_ids.extend(case.ids.iter());
                let case_left = (
                    score + 1001,
                    Case {
                        pos: left_position,
                        dir: turn_left,
                        ids: left_ids,
                    },
                );
                current_id += 1;

                let turn_right = case.dir.turn_right();
                let right_position = case
                    .pos
                    .move_by(&Distance::from_direction(&turn_right), width, height)
                    .expect("the map has walls all around so it won't ever go out");
                let mut right_ids = vec![current_id];
                right_ids.extend(case.ids.iter());
                let case_right = (
                    score + 1001,
                    Case {
                        pos: right_position,
                        dir: turn_right,
                        ids: right_ids,
                    },
                );
                current_id += 1;

                let forward_position = case
                    .pos
                    .move_by(&Distance::from_direction(&case.dir), width, height)
                    .expect("the map has walls all around so it won't ever go out");
                let mut forward_ids = vec![current_id];
                forward_ids.extend(case.ids.iter());
                let case_forward = (
                    score + 1,
                    Case {
                        pos: forward_position,
                        dir: case.dir.clone(),
                        ids: forward_ids,
                    },
                );
                current_id += 1;

                for (next_score, next_case) in [case_forward, case_left, case_right] {
                    if next_score > min_score.0 {
                        continue;
                    }

                    if next_case.pos == self.end {
                        if next_score == min_score.0 {
                            min_score.1.extend(next_case.ids);
                        } else {
                            min_score = (next_score, next_case.ids);
                        }
                        continue;
                    }

                    if matches!(maze[next_case.pos.1][next_case.pos.0], Cell::Visited(scores, _) if scores[direction_to_index(&next_case.dir)] > next_score)
                        || matches!(maze[next_case.pos.1][next_case.pos.0], Cell::Free)
                    {
                        match cases.entry(next_score) {
                            Entry::Vacant(vacant_entry) => {
                                vacant_entry.insert(vec![next_case]);
                            }
                            Entry::Occupied(mut occupied_entry) => {
                                occupied_entry.get_mut().push(next_case);
                            }
                        }
                    }
                }
            }
        }

        let min_score_ids = HashSet::<usize>::from_iter(min_score.1.into_iter());
        (
            min_score.0,
            maze
                .iter()
                .map(|line| {
                    line.iter()
                        .filter(|cell| matches!(cell, Cell::Visited(_, ids) if ids.iter().any(|id| min_score_ids.contains(id))))
                        .count()
                })
                .sum::<usize>() + 1,
        )
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
