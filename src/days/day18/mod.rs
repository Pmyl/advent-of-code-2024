// https://adventofcode.com/2024/day/18

use std::{collections::BTreeMap, usize};

use crate::Position;

pub fn solution_part1(input: &str, width: usize, height: usize, bytes_fallen: usize) -> usize {
    let mut map = MemoryMap::from_input(input, width, height);
    map.nanoseconds_passed(bytes_fallen);
    map.shortest_path_length(Position(0, 0), Position(width - 1, height - 1))
        .expect("should have a path")
}

pub fn solution_part2(input: &str, width: usize, height: usize, bytes_fallen: usize) -> String {
    let mut map = MemoryMap::from_input(input, width, height);
    map.nanoseconds_passed(bytes_fallen);

    loop {
        if !map.is_there_a_path(Position(0, 0), Position(width - 1, height - 1)) {
            return format!(
                "{},{}",
                map.bytes_to_fall[map.time - 1].0,
                map.bytes_to_fall[map.time - 1].1
            );
        }
        map.nanoseconds_passed(1);
    }
}

struct MemoryMap {
    width: usize,
    height: usize,
    bytes_corrupted: Vec<Vec<bool>>,
    bytes_to_fall: Vec<Position>,
    time: usize,
}

impl MemoryMap {
    fn from_input(input: &str, width: usize, height: usize) -> Self {
        let bytes_to_fall = input
            .lines()
            .map(|byte_to_fall| {
                let (x, y) = byte_to_fall.split_once(',').unwrap();
                Position(x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .collect::<Vec<_>>();

        let bytes = (0..height)
            .map(|_| (0..width).map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            width,
            height,
            bytes_corrupted: bytes,
            bytes_to_fall,
            time: 0,
        }
    }

    fn nanoseconds_passed(&mut self, nanoseconds: usize) {
        for i in 0..nanoseconds {
            let pos = &self.bytes_to_fall[self.time + i];
            self.bytes_corrupted[pos.1][pos.0] = true;
        }

        self.time += nanoseconds;
    }

    fn shortest_path_length(&self, start: Position, end: Position) -> Option<usize> {
        let mut paths = BTreeMap::<usize, Vec<Position>>::new();
        paths.insert(0, vec![start]);

        let mut visited = (0..self.height)
            .map(|_| (0..self.width).map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        visited[0][0] = true;

        while let Some((steps, positions)) = paths.pop_first() {
            for position in positions {
                let up = position.up();
                let down = position.down_bounded(self.height);
                let right = position.right_bounded(self.width);
                let left = position.left();

                for next_position in [up, down, right, left].into_iter().filter_map(|p| p) {
                    if visited[next_position.1][next_position.0]
                        || self.bytes_corrupted[next_position.1][next_position.0]
                    {
                        continue;
                    }
                    visited[next_position.1][next_position.0] = true;

                    if next_position == end {
                        return Some(steps + 1);
                    }

                    if let Some(paths_mut) = paths.get_mut(&(steps + 1)) {
                        paths_mut.push(next_position);
                    } else {
                        paths.insert(steps + 1, vec![next_position]);
                    }
                }
            }
        }

        None
    }

    fn is_there_a_path(&self, start: Position, end: Position) -> bool {
        let mut paths = BTreeMap::<usize, Vec<Position>>::new();
        paths.insert(0, vec![start]);

        let mut visited = (0..self.height)
            .map(|_| (0..self.width).map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        visited[0][0] = true;

        while let Some((_, positions)) = paths.pop_first() {
            for position in positions {
                let up = position.up();
                let down = position.down_bounded(self.height);
                let right = position.right_bounded(self.width);
                let left = position.left();

                for next_position in [up, down, right, left].into_iter().filter_map(|p| p) {
                    if visited[next_position.1][next_position.0]
                        || self.bytes_corrupted[next_position.1][next_position.0]
                    {
                        continue;
                    }
                    visited[next_position.1][next_position.0] = true;

                    if next_position == end {
                        return true;
                    }

                    let key = (end.0 - next_position.0).pow(2) + (end.1 - next_position.1).pow(2);

                    if let Some(paths_mut) = paths.get_mut(&key) {
                        paths_mut.push(next_position);
                    } else {
                        paths.insert(key, vec![next_position]);
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE, 7, 7, 12), 22);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT, 71, 71, 1024), 454);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE, 7, 7, 12), "6,1");
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT, 71, 71, 1024), "8,51");
    }
}
