// https://adventofcode.com/2024/day/20

use std::collections::{BTreeMap, HashMap};

use crate::{Distance, Position};

pub fn solution_part1(input: &str, picoseconds_to_save: usize) -> usize {
    let race = Race::from_input(input);
    race.how_many_cheats_to_save(2, picoseconds_to_save)
}

pub fn solution_part2(input: &str, picoseconds_to_save: usize) -> usize {
    let race = Race::from_input(input);
    race.how_many_cheats_to_save(20, picoseconds_to_save)
}

struct Race {
    width: usize,
    height: usize,
    walls: Vec<Vec<bool>>,
    start: Position,
    end: Position,
}

impl Race {
    fn from_input(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let height = lines.len();
        let width = lines[0].len();
        let mut start_position = None;
        let mut end_position = None;
        let mut walls = vec![];

        for (y, line) in lines.into_iter().enumerate() {
            let mut walls_line = vec![false; width];

            for (x, c) in line.into_iter().enumerate() {
                match c {
                    '.' => walls_line[x] = false,
                    '#' => walls_line[x] = true,
                    'S' => start_position = Some(Position(x, y)),
                    'E' => end_position = Some(Position(x, y)),
                    _ => unreachable!(),
                }
            }

            walls.push(walls_line);
        }

        Self {
            width,
            height,
            walls,
            end: end_position.unwrap(),
            start: start_position.unwrap(),
        }
    }

    fn how_many_cheats_to_save(
        &self,
        cheat_picoseconds: usize,
        picoseconds_to_save: usize,
    ) -> usize {
        let path = self.race_to_end_path();
        let path_map = HashMap::<Position, usize>::from_iter(
            path.iter().enumerate().map(|(i, p)| (p.clone(), i)),
        );

        let mut total = 0;

        for (ps, position) in path.into_iter().enumerate() {
            for x in -(cheat_picoseconds as isize)..=(cheat_picoseconds as isize) {
                let y_breadth = cheat_picoseconds as isize - x.abs();
                for y in -y_breadth..=y_breadth {
                    if x == 0 && y == 0 || x.abs() + y.abs() == 1 {
                        continue;
                    }

                    let distance_cheating = Distance(x, y);
                    let distance_cheating_steps = x.abs() as usize + y.abs() as usize;

                    let Some(reached_position) =
                        position.move_by(&distance_cheating, self.width, self.height)
                    else {
                        continue;
                    };

                    if let Some(picoseconds_reached) = path_map.get(&reached_position) {
                        if *picoseconds_reached > ps + distance_cheating_steps
                            && picoseconds_reached - (ps + distance_cheating_steps)
                                >= picoseconds_to_save
                        {
                            total += 1;
                        }
                    }
                }
            }
        }

        total
    }

    fn race_to_end_path(&self) -> Vec<Position> {
        let mut paths = BTreeMap::<usize, Vec<(Position, Vec<Position>)>>::new();
        paths.insert(0, vec![(self.start.clone(), vec![self.start.clone()])]);

        let mut visited = (0..self.height)
            .map(|_| (0..self.width).map(|_| usize::MAX).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        loop {
            let (picoseconds, positions) = paths.pop_first().expect("there is always a path");

            for (position, path) in positions {
                if visited[position.1][position.0] <= picoseconds {
                    continue;
                }

                visited[position.1][position.0] = picoseconds;

                let next_picoseconds = picoseconds + 1;

                let up = position.up();
                let down = position.down_bounded(self.height);
                let right = position.right_bounded(self.width);
                let left = position.left();

                for next_position in [up, down, right, left].into_iter().filter_map(|p| p) {
                    if self.walls[next_position.1][next_position.0] {
                        continue;
                    }

                    let mut next_path = path.clone();
                    next_path.push(next_position.clone());

                    if next_position == self.end {
                        return next_path;
                    }

                    if let Some(paths_mut) = paths.get_mut(&(next_picoseconds)) {
                        paths_mut.push((next_position, next_path));
                    } else {
                        paths.insert(next_picoseconds, vec![(next_position, next_path)]);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE, 70), 0);
        assert_eq!(solution_part1(EXAMPLE, 50), 1);
        assert_eq!(solution_part1(EXAMPLE, 40), 2);
        assert_eq!(solution_part1(EXAMPLE, 30), 4);
        assert_eq!(solution_part1(EXAMPLE, 20), 5);
        assert_eq!(solution_part1(EXAMPLE, 10), 10);
        assert_eq!(solution_part1(EXAMPLE, 5), 16);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT, 100), 1524);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE, 80), 0);
        assert_eq!(solution_part2(EXAMPLE, 75), 3);
        assert_eq!(solution_part2(EXAMPLE, 70), 41);
        assert_eq!(solution_part2(EXAMPLE, 60), 129);
        assert_eq!(solution_part2(EXAMPLE, 50), 285);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT, 100), 1033746);
    }
}
