// https://adventofcode.com/2024/day/8

use std::collections::{hash_map::Entry, HashMap};

use crate::Position;

pub fn solution_part1(input: &str) -> usize {
    let grid = CityGrid::from_input(input);
    grid.count_antinodes()
}

pub fn solution_part2(input: &str) -> usize {
    let grid = CityGrid::from_input(input);
    grid.count_antinodes_extended()
}

struct CityGrid {
    width: usize,
    height: usize,
    antennas: Vec<Antenna>,
}

struct Antenna {
    position: Position,
    frequency: char,
}

struct Distance(isize, isize);

impl Distance {
    fn between(position1: &Position, position2: &Position) -> Self {
        Self(
            position2.0 as isize - position1.0 as isize,
            position2.1 as isize - position1.1 as isize,
        )
    }

    fn multiply(&self, n: usize) -> Distance {
        Distance(self.0 * n as isize, self.1 * n as isize)
    }
}

impl Position {
    fn move_by(&self, distance: &Distance, width: usize, height: usize) -> Option<Position> {
        let target_x = self.0 as isize + distance.0;
        let target_y = self.1 as isize + distance.1;

        if target_x >= 0 && target_x < width as isize && target_y >= 0 && target_y < height as isize
        {
            Some(Position(target_x as usize, target_y as usize))
        } else {
            None
        }
    }
}

impl CityGrid {
    fn from_input(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();

        Self {
            width: lines[0].len(),
            height: lines.len(),
            antennas: lines
                .into_iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, cell)| {
                        if cell != '.' {
                            Some(Antenna {
                                position: Position(x, y),
                                frequency: cell,
                            })
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>(),
        }
    }

    fn count_antinodes(self) -> usize {
        let antennas_by_frequency = group_by_frequency(self.antennas);

        let mut antinode_cells: Vec<bool> = vec![false; self.width * self.height];

        for antennas in antennas_by_frequency.into_values() {
            for antenna1 in antennas.iter() {
                for antenna2 in antennas.iter() {
                    if antenna1.position.0 == antenna2.position.0
                        && antenna1.position.1 == antenna2.position.1
                    {
                        continue;
                    }

                    let distance = Distance::between(&antenna1.position, &antenna2.position);
                    let negative_distance =
                        Distance::between(&antenna2.position, &antenna1.position);

                    if let Some(antinode1_pos) =
                        antenna1
                            .position
                            .move_by(&negative_distance, self.width, self.height)
                    {
                        antinode_cells[antinode1_pos.0 * self.height + antinode1_pos.1] = true;
                    }

                    if let Some(antinode2_pos) =
                        antenna2
                            .position
                            .move_by(&distance, self.width, self.height)
                    {
                        antinode_cells[antinode2_pos.0 * self.height + antinode2_pos.1] = true;
                    }
                }
            }
        }

        antinode_cells.into_iter().filter(|cell| *cell).count()
    }

    fn count_antinodes_extended(self) -> usize {
        let antennas_by_frequency = group_by_frequency(self.antennas);

        let mut antinode_cells: Vec<bool> = vec![false; self.width * self.height];

        for antennas in antennas_by_frequency.into_values() {
            for antenna1 in antennas.iter() {
                for antenna2 in antennas.iter() {
                    if antenna1.position.0 == antenna2.position.0
                        && antenna1.position.1 == antenna2.position.1
                    {
                        continue;
                    }

                    antinode_cells[antenna1.position.0 * self.height + antenna1.position.1] = true;
                    antinode_cells[antenna2.position.0 * self.height + antenna2.position.1] = true;

                    let distance = Distance::between(&antenna1.position, &antenna2.position);
                    let negative_distance =
                        Distance::between(&antenna2.position, &antenna1.position);

                    let mut count = 1;
                    while let Some(antinode1_pos) = antenna1.position.move_by(
                        &negative_distance.multiply(count),
                        self.width,
                        self.height,
                    ) {
                        antinode_cells[antinode1_pos.0 * self.height + antinode1_pos.1] = true;
                        count += 1;
                    }

                    let mut count = 1;
                    while let Some(antinode2_pos) = antenna2.position.move_by(
                        &distance.multiply(count),
                        self.width,
                        self.height,
                    ) {
                        antinode_cells[antinode2_pos.0 * self.height + antinode2_pos.1] = true;
                        count += 1;
                    }
                }
            }
        }

        antinode_cells.into_iter().filter(|cell| *cell).count()
    }
}

fn group_by_frequency(antennas: Vec<Antenna>) -> HashMap<char, Vec<Antenna>> {
    let mut antennas_by_frequency: HashMap<char, Vec<Antenna>> = HashMap::new();

    for antenna in antennas {
        match antennas_by_frequency.entry(antenna.frequency) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.get_mut().push(antenna);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(vec![antenna]);
            }
        }
    }

    antennas_by_frequency
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRE_EXAMPLE: &str = "..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........";

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_pre_example() {
        assert_eq!(solution_part1(PRE_EXAMPLE), 2);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 14);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 295);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 34);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 1034);
    }
}
