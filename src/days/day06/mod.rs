// https://adventofcode.com/2024/day/6

use std::collections::HashSet;

pub fn solution_part1(input: &str) -> usize {
    let lab = Lab::from_input(input);
    lab.count_patrolled_positions()
}

pub fn solution_part2(input: &str) -> usize {
    let lab = Lab::from_input(input);
    lab.count_obstructions_to_loop()
}

#[derive(Clone)]
struct Lab {
    map: LabMap,
    guard: Guard,
}

#[derive(Clone)]
struct LabMap {
    obstructions: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

#[derive(Clone)]
struct Guard {
    position: GuardPosition,
    direction: Direction,
}

#[derive(Clone)]
enum GuardPosition {
    InMap((usize, usize)),
    OutsideMap,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

impl Lab {
    fn from_input(input: &str) -> Self {
        let mut obstructions: Vec<Vec<bool>> = vec![];
        let mut guard = None;
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            height += 1;
            width = line.len();
            let mut obstructions_line = vec![false; width];

            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        obstructions_line[x] = true;
                    }
                    '^' => {
                        guard = Some(Guard {
                            position: GuardPosition::InMap((x, y)),
                            direction: Direction::UP,
                        });
                    }
                    _ => {}
                }
            }

            obstructions.push(obstructions_line);
        }

        Self {
            guard: guard.expect("Guard to be in the input"),
            map: LabMap {
                obstructions,
                width,
                height,
            },
        }
    }

    fn count_patrolled_positions(mut self) -> usize {
        self.patrol_to_end().len()
    }

    fn count_obstructions_to_loop(mut self) -> usize {
        let GuardPosition::InMap(start_position) = self.guard.position else {
            unreachable!("Always starts in map");
        };
        let start_guard_position = self.guard.clone();
        let original_patrolling_positions = self.patrol_to_end();
        let mut obstructions_count = 0;

        for position in original_patrolling_positions {
            if position == start_position {
                continue;
            }

            self.guard.direction = start_guard_position.direction.clone();
            self.guard.position = start_guard_position.position.clone();

            self.map.obstructions[position.1][position.0] = true;
            if self.check_for_loop() {
                obstructions_count += 1;
            }
            self.map.obstructions[position.1][position.0] = false;
        }

        obstructions_count
    }

    fn patrol_to_end(&mut self) -> HashSet<(usize, usize)> {
        let mut patrolled_cases = HashSet::new();

        loop {
            let GuardPosition::InMap(pos) = self.guard.position else {
                break;
            };

            if patrolled_cases.contains(&(pos.0, pos.1, self.guard.direction)) {
                break;
            }

            patrolled_cases.insert((pos.0, pos.1, self.guard.direction));
            self.guard.move_step(&self.map);
        }

        let patrolled_positions = patrolled_cases
            .into_iter()
            .map(|c| (c.0, c.1))
            .collect::<HashSet<(usize, usize)>>();

        patrolled_positions
    }

    fn check_for_loop(&mut self) -> bool {
        let mut hit_obstructions = HashSet::new();

        loop {
            let GuardPosition::InMap(_) = self.guard.position else {
                break;
            };

            if let Some(hit_obstruction) = self.guard.move_step(&self.map) {
                let key = (hit_obstruction.0, hit_obstruction.1, self.guard.direction);

                if hit_obstructions.contains(&key) {
                    return true;
                }

                hit_obstructions.insert(key);
            }
        }

        false
    }
}

impl Guard {
    fn move_step(&mut self, map: &LabMap) -> Option<(usize, usize)> {
        let mut hit_obstruction = None;

        loop {
            let GuardPosition::InMap(pos) = self.position else {
                break;
            };

            let movement: (isize, isize) = match self.direction {
                Direction::UP => (0, -1),
                Direction::RIGHT => (1, 0),
                Direction::DOWN => (0, 1),
                Direction::LEFT => (-1, 0),
            };

            let new_x = pos.0 as isize + movement.0;
            let new_y = pos.1 as isize + movement.1;

            if (pos.0 as isize) < -movement.0
                || (pos.0 as isize) >= (map.width as isize - movement.0)
                || (pos.1 as isize) < -movement.1
                || (pos.1 as isize) >= (map.height as isize - movement.1)
            {
                self.position = GuardPosition::OutsideMap;
                break;
            } else if map.obstructions[new_y as usize][new_x as usize] {
                self.direction = self.direction.turn_right();
                hit_obstruction = Some((new_x as usize, new_y as usize));
            } else {
                self.position = GuardPosition::InMap((new_x as usize, new_y as usize));
                break;
            }
        }

        hit_obstruction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 41);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 4973);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 1482);
    }
}
