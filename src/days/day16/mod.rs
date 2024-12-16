// https://adventofcode.com/2024/day/16

use std::{
    collections::{btree_map::Entry, BTreeMap, HashMap, HashSet},
    thread::sleep,
    time::Duration,
};

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
        let width = self.maze[0].len();
        let height = self.maze.len();

        let mut step = 0;

        let mut min_score: usize = usize::MAX;

        let mut maze = self.maze.clone();
        let mut cases = BTreeMap::<usize, Vec<(Position, Direction)>>::new();
        cases.insert(0, vec![(self.start.clone(), Direction::Right)]);

        while let Some((score, sub_cases)) = cases.pop_first() {
            for case in sub_cases {
                // print_maze(&maze, &self.end, &case, min_score, &cases, step);
                step += 1;

                if let Cell::Visited(mut scores) = maze[case.0 .1][case.0 .0] {
                    scores[direction_to_index(&case.1)] = score;
                } else {
                    let mut scores = [usize::MAX; 4];
                    scores[direction_to_index(&case.1)] = score;
                    maze[case.0 .1][case.0 .0] = Cell::Visited(scores);
                }

                let case_forward = (
                    score + 1,
                    (
                        case.0
                            .move_by(&Distance::from_direction(&case.1), width, height)
                            .expect("the map has walls all around so it won't ever go out"),
                        case.1.clone(),
                    ),
                );
                let turn_left = case.1.turn_left();
                let case_left = (
                    score + 1001,
                    (
                        case.0
                            .move_by(&Distance::from_direction(&turn_left), width, height)
                            .expect("the map has walls all around so it won't ever go out"),
                        turn_left,
                    ),
                );
                let turn_right = case.1.turn_right();
                let case_right = (
                    score + 1001,
                    (
                        case.0
                            .move_by(&Distance::from_direction(&turn_right), width, height)
                            .expect("the map has walls all around so it won't ever go out"),
                        turn_right,
                    ),
                );

                for next_case in [case_forward, case_left, case_right] {
                    let next_score = next_case.0;
                    let next_position = &next_case.1 .0;
                    let next_direction = &next_case.1 .1;

                    if next_score > min_score {
                        continue;
                    }

                    if next_position == &self.end {
                        min_score = next_score;
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

    fn count_tiles_of_lowest_score_possible(&self) -> usize {
        HashSet::<Position>::from_iter(self.all_lowest_score_paths().1.into_iter().flatten()).len()
    }

    fn all_lowest_score_paths(&self) -> (usize, Vec<Vec<Position>>) {
        let width = self.maze[0].len();
        let height = self.maze.len();

        let mut step = 0;

        let mut min_score: usize = usize::MAX;

        let mut maze = self.maze.clone();
        let mut cases = BTreeMap::<usize, Vec<(Position, Direction)>>::new();
        cases.insert(0, vec![(self.start.clone(), Direction::Right)]);

        while let Some((score, sub_cases)) = cases.pop_first() {
            for case in sub_cases {
                // print_maze(&maze, &self.end, &case, min_score, &cases, step);
                step += 1;

                if let Cell::Visited(mut scores) = maze[case.0 .1][case.0 .0] {
                    scores[direction_to_index(&case.1)] = score;
                } else {
                    let mut scores = [usize::MAX; 4];
                    scores[direction_to_index(&case.1)] = score;
                    maze[case.0 .1][case.0 .0] = Cell::Visited(scores);
                }

                let case_forward = (
                    score + 1,
                    (
                        case.0
                            .move_by(&Distance::from_direction(&case.1), width, height)
                            .expect("the map has walls all around so it won't ever go out"),
                        case.1.clone(),
                    ),
                );
                let turn_left = case.1.turn_left();
                let case_left = (
                    score + 1001,
                    (
                        case.0
                            .move_by(&Distance::from_direction(&turn_left), width, height)
                            .expect("the map has walls all around so it won't ever go out"),
                        turn_left,
                    ),
                );
                let turn_right = case.1.turn_right();
                let case_right = (
                    score + 1001,
                    (
                        case.0
                            .move_by(&Distance::from_direction(&turn_right), width, height)
                            .expect("the map has walls all around so it won't ever go out"),
                        turn_right,
                    ),
                );

                for next_case in [case_forward, case_left, case_right] {
                    let next_score = next_case.0;
                    let next_position = &next_case.1 .0;
                    let next_direction = &next_case.1 .1;

                    if next_score > min_score {
                        continue;
                    }

                    if next_position == &self.end {
                        min_score = next_score;
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

        (min_score, vec![])
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

fn print_maze(
    maze: &[Vec<Cell>],
    end: &Position,
    case: &(Position, Direction),
    min_score: usize,
    cases: &BTreeMap<usize, Vec<(Position, Direction)>>,
    step: usize,
) {
    let cases = HashMap::<Position, Direction>::from_iter(
        cases.iter().map(|c| (c.1[0].0.clone(), c.1[0].1.clone())),
    );

    for (y, line) in maze.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if end == &Position(x, y) {
                print!("E");
            } else if cases.contains_key(&Position(x, y)) {
                print!(
                    "{}",
                    match cases.get(&Position(x, y)).unwrap() {
                        Direction::Up => "^",
                        Direction::Down => "V",
                        Direction::Left => "<",
                        Direction::Right => ">",
                    }
                );
            } else if case.0 == Position(x, y) {
                print!(
                    "{}",
                    match case.1 {
                        Direction::Up => "U",
                        Direction::Down => "D",
                        Direction::Left => "L",
                        Direction::Right => "R",
                    }
                );
            } else {
                print!(
                    "{}",
                    match cell {
                        Cell::Free => '.',
                        Cell::Wall => '#',
                        Cell::Visited(_) => 'O',
                    }
                );
            }
        }

        println!();
    }
    println!();
    println!("Score({}) Step({})", min_score, step);
    sleep(Duration::from_millis(500));
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
        assert_eq!(solution_part2(INPUT), 0);
    }
}
