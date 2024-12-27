// https://adventofcode.com/2024/day/21

use std::collections::{hash_map::Entry, BTreeMap, HashMap};

use crate::Position;

pub fn solution_part1(input: &str) -> usize {
    let codes = Codes::from_input(input);
    codes.complexity2(3)
}

pub fn solution_part2(input: &str) -> usize {
    let codes = Codes::from_input(input);
    codes.complexity2(26)
}

struct Codes(Vec<(Vec<NumericKey>, usize)>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumericKey {
    A,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirectionalKey {
    A,
    Up,
    Left,
    Down,
    Right,
}

fn numeric_keypad_shortest_paths() -> HashMap<(NumericKey, NumericKey), Vec<Vec<DirectionalKey>>> {
    let all_keys = [
        (NumericKey::A, Position(2, 3)),
        (NumericKey::N0, Position(1, 3)),
        (NumericKey::N1, Position(0, 2)),
        (NumericKey::N2, Position(1, 2)),
        (NumericKey::N3, Position(2, 2)),
        (NumericKey::N4, Position(0, 1)),
        (NumericKey::N5, Position(1, 1)),
        (NumericKey::N6, Position(2, 1)),
        (NumericKey::N7, Position(0, 0)),
        (NumericKey::N8, Position(1, 0)),
        (NumericKey::N9, Position(2, 0)),
    ];
    let keypad = vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![false, true, true],
    ];

    let mut paths = HashMap::new();

    for (from_key, from_pos) in all_keys.iter() {
        for (to_key, to_pos) in all_keys.iter() {
            let directional_keys_to_press = keypad_shortest_paths(&keypad, from_pos, to_pos);

            paths.insert(
                (from_key.clone(), to_key.clone()),
                directional_keys_to_press,
            );
        }
    }

    paths
}

fn directional_keypad_shortest_paths(
) -> HashMap<(DirectionalKey, DirectionalKey), Vec<Vec<DirectionalKey>>> {
    let all_keys = [
        (DirectionalKey::A, Position(2, 0)),
        (DirectionalKey::Up, Position(1, 0)),
        (DirectionalKey::Down, Position(1, 1)),
        (DirectionalKey::Left, Position(0, 1)),
        (DirectionalKey::Right, Position(2, 1)),
    ];
    let keypad = vec![vec![false, true, true], vec![true, true, true]];

    let mut paths = HashMap::new();

    for (from_key, from_pos) in all_keys.iter() {
        for (to_key, to_pos) in all_keys.iter() {
            let directional_keys_to_press = keypad_shortest_paths(&keypad, from_pos, to_pos);

            paths.insert(
                (from_key.clone(), to_key.clone()),
                directional_keys_to_press,
            );
        }
    }

    paths
}

fn keypad_shortest_paths(
    keypad: &Vec<Vec<bool>>,
    from_pos: &Position,
    to_pos: &Position,
) -> Vec<Vec<DirectionalKey>> {
    if from_pos == to_pos {
        return vec![vec![DirectionalKey::A]];
    }

    let height = keypad.len();
    let width = keypad[0].len();

    let mut path_finding_paths = BTreeMap::<usize, Vec<(Position, Vec<DirectionalKey>)>>::new();
    path_finding_paths.insert(0, vec![(from_pos.clone(), vec![])]);
    let mut visited = HashMap::<Position, usize>::new();

    let mut shortest_paths = vec![];

    while let Some((steps, positions)) = path_finding_paths.pop_first() {
        for (position, directional_keys_to_press) in positions {
            let up = position.up().map(|p| (p, DirectionalKey::Up));
            let down = position
                .down_bounded(height)
                .map(|p| (p, DirectionalKey::Down));
            let right = position
                .right_bounded(width)
                .map(|p| (p, DirectionalKey::Right));
            let left = position.left().map(|p| (p, DirectionalKey::Left));

            let next_steps = steps + 1;

            for (next_pos, directional_key) in [up, down, right, left].into_iter().filter_map(|p| p)
            {
                match visited.entry(next_pos.clone()) {
                    Entry::Occupied(mut occupied_entry) => {
                        if *occupied_entry.get() < next_steps {
                            continue;
                        } else {
                            *occupied_entry.get_mut() = next_steps;
                        }
                    }
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(next_steps);
                    }
                }
                let mut directional_keys_to_press = directional_keys_to_press.clone();
                directional_keys_to_press.push(directional_key);

                if keypad[next_pos.1][next_pos.0] {
                    if &next_pos == to_pos {
                        directional_keys_to_press.push(DirectionalKey::A);
                        shortest_paths.push(directional_keys_to_press);
                    } else if let Some(paths_mut) = path_finding_paths.get_mut(&next_steps) {
                        paths_mut.push((next_pos, directional_keys_to_press));
                    } else {
                        path_finding_paths
                            .insert(next_steps, vec![(next_pos, directional_keys_to_press)]);
                    }
                }
            }
        }
    }

    shortest_paths
}

impl Codes {
    fn from_input(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    let value = line
                        .chars()
                        .filter_map(|c| c.to_digit(10))
                        .fold(0usize, |acc, n| acc * 10 + n as usize);
                    (
                        line.chars()
                            .map(|c| match c {
                                '0' => NumericKey::N0,
                                '1' => NumericKey::N1,
                                '2' => NumericKey::N2,
                                '3' => NumericKey::N3,
                                '4' => NumericKey::N4,
                                '5' => NumericKey::N5,
                                '6' => NumericKey::N6,
                                '7' => NumericKey::N7,
                                '8' => NumericKey::N8,
                                '9' => NumericKey::N9,
                                'A' => NumericKey::A,
                                _ => panic!("Invalid input"),
                            })
                            .collect::<Vec<NumericKey>>(),
                        value,
                    )
                })
                .collect::<Vec<(Vec<NumericKey>, usize)>>(),
        )
    }

    fn complexity(&self, directional_keypads: usize) -> usize {
        let numeric_keys_paths = numeric_keypad_shortest_paths();
        let directional_keys_paths = directional_keypad_shortest_paths();

        let mut complexity = 0;

        for (keys, keys_value) in self.0.iter() {
            let mut directions_possibilities: Vec<Vec<DirectionalKey>> = vec![vec![]];

            let mut prev = NumericKey::A;
            for i in 0..keys.len() {
                let current = keys[i];

                let paths = &numeric_keys_paths[&(prev, current)];
                let prev_directions_possibilities = directions_possibilities;
                directions_possibilities = vec![];

                for possibility in prev_directions_possibilities {
                    for path in paths.iter() {
                        let mut possibility = possibility.clone();
                        possibility.extend(path.clone());
                        directions_possibilities.push(possibility);
                    }
                }
                prev = current;
            }

            for _ in 1..directional_keypads {
                let mut new_directions_possibilities: Vec<Vec<DirectionalKey>> = vec![];

                for possibility in directions_possibilities {
                    let mut new_directions: Vec<Vec<DirectionalKey>> = vec![vec![]];

                    let mut prev = DirectionalKey::A;
                    for current in possibility {
                        let paths = &directional_keys_paths[&(prev, current)];
                        let prev_directions = new_directions;
                        new_directions = vec![];
                        for new_possibility in prev_directions {
                            for path in paths.iter() {
                                let mut new_possibility = new_possibility.clone();
                                new_possibility.extend(path.clone());
                                new_directions.push(new_possibility);
                            }
                        }
                        prev = current;
                    }

                    new_directions_possibilities.append(&mut new_directions);
                }

                let shortest = new_directions_possibilities
                    .iter()
                    .map(|nd| nd.len())
                    .min()
                    .unwrap();
                directions_possibilities = new_directions_possibilities
                    .into_iter()
                    .filter(|nd| nd.len() == shortest)
                    .collect::<Vec<_>>();
            }

            complexity += directions_possibilities[0].len() * keys_value;
        }

        complexity
    }

    fn complexity2(&self, directional_keypads: usize) -> usize {
        let numeric_keys_paths = numeric_keypad_shortest_paths();
        let directional_keys_paths = directional_keypad_shortest_paths();

        let mut complexity = 0;

        for (keys, keys_value) in self.0.iter() {
            let mut directions_possibilities: Vec<Vec<DirectionalKey>> = vec![vec![]];

            let mut prev = NumericKey::A;
            for i in 0..keys.len() {
                let current = keys[i];

                let paths = &numeric_keys_paths[&(prev, current)];
                let prev_directions_possibilities = directions_possibilities;
                directions_possibilities = vec![];

                for possibility in prev_directions_possibilities {
                    for path in paths.iter() {
                        let mut possibility = possibility.clone();
                        possibility.extend(path.clone());
                        directions_possibilities.push(possibility);
                    }
                }
                prev = current;
            }

            let mut memo: HashMap<(DirectionalKey, DirectionalKey, usize), usize> = HashMap::new();

            let keys_to_press = directions_possibilities
                .into_iter()
                .map(|directions| {
                    let mut prev_direction = DirectionalKey::A;

                    directions
                        .into_iter()
                        .map(|direction| {
                            let complexity = complexity_recursive(
                                &mut memo,
                                &directional_keys_paths,
                                directional_keypads - 1,
                                prev_direction,
                                direction,
                            );
                            prev_direction = direction;
                            complexity
                        })
                        .sum::<usize>()
                })
                .min()
                .unwrap();

            complexity += keys_to_press * keys_value;
        }

        complexity
    }
}

fn complexity_recursive(
    memo: &mut HashMap<(DirectionalKey, DirectionalKey, usize), usize>,
    directional_keys_paths: &HashMap<(DirectionalKey, DirectionalKey), Vec<Vec<DirectionalKey>>>,
    directional_keypads: usize,
    start_key: DirectionalKey,
    to_key: DirectionalKey,
) -> usize {
    if directional_keypads == 0 {
        return 1;
    }

    let paths = &directional_keys_paths[&(start_key, to_key)];
    paths
        .iter()
        .map(|path| {
            let mut prev_key = DirectionalKey::A;

            path.iter()
                .map(|key| {
                    let new_score = if let Some(memo_score) =
                        memo.get(&(prev_key, *key, directional_keypads))
                    {
                        *memo_score
                    } else {
                        let new_score = complexity_recursive(
                            memo,
                            directional_keys_paths,
                            directional_keypads - 1,
                            prev_key,
                            *key,
                        );
                        *memo
                            .entry((prev_key, *key, directional_keypads))
                            .or_default() = new_score;
                        new_score
                    };
                    prev_key = *key;

                    new_score
                })
                .sum::<usize>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "029A
980A
179A
456A
379A";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_sub_example_1() {
        assert_eq!(Codes::from_input("029A").complexity2(1), 12 * 29);
    }

    #[test]
    fn test_part1_sub_example_2() {
        assert_eq!(Codes::from_input("029A").complexity2(2), 28 * 29);
    }

    #[test]
    fn test_part1_sub_example_3() {
        assert_eq!(Codes::from_input("029A").complexity2(3), 68 * 29);
    }

    #[test]
    fn test_part1_sub_example_4() {
        assert_eq!(Codes::from_input("3").complexity2(3), 12 * 3);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 126384);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 278568);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 341460772681012);
    }
}
