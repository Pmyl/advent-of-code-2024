// https://adventofcode.com/2024/day/12

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

use crate::{Movable, Position};

pub fn solution_part1(input: &str) -> usize {
    let plots = GardenPlots::from_input(input);
    plots.calculate_price_of_fencing()
}

pub fn solution_part2(input: &str) -> usize {
    let plots = GardenPlots::from_input(input);
    plots.calculate_new_price_of_fencing()
}

struct GardenPlots {
    regions: Vec<Region>,
}

struct Region {
    _id: char,
    plants: BTreeSet<Position>,
}

impl GardenPlots {
    fn from_input(input: &str) -> Self {
        let mut all_position = BTreeSet::new();
        let lines = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<char>>>();

        for y in 0..lines.len() {
            for x in 0..lines[0].len() {
                all_position.insert(Position(x, y));
            }
        }

        let mut regions = vec![];

        while let Some(position) = all_position.pop_first() {
            regions.push(find_region(position, &mut all_position, &lines));
        }

        Self { regions }
    }

    fn calculate_price_of_fencing(&self) -> usize {
        self.regions
            .iter()
            .map(|region| region.calculate_price_of_fencing())
            .sum()
    }

    fn calculate_new_price_of_fencing(&self) -> usize {
        self.regions
            .iter()
            .map(|region| region.calculate_new_price_of_fencing())
            .sum()
    }
}

impl Region {
    fn calculate_price_of_fencing(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn calculate_new_price_of_fencing(&self) -> usize {
        self.area() * self.sides()
    }

    fn area(&self) -> usize {
        self.plants.len()
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0;

        for plant in self.plants.iter() {
            match plant.left() {
                Some(pos) if self.plants.contains(&pos) => {}
                _ => perimeter += 1,
            }
            if !self.plants.contains(&plant.right()) {
                perimeter += 1;
            }
            match plant.up() {
                Some(pos) if self.plants.contains(&pos) => {}
                _ => perimeter += 1,
            }
            if !self.plants.contains(&plant.down()) {
                perimeter += 1;
            }
        }

        perimeter
    }

    fn sides(&self) -> usize {
        let mut verticals = BTreeMap::<isize, Vec<usize>>::new();
        let mut horizontals = BTreeMap::<isize, Vec<usize>>::new();

        // looking up/down (or left/right) to the same empty spot count as two different sides
        // so we add 1000000 for one and nothing for the other, hoping that the actual map
        // is less than 1000000 wide/tall
        for plant in self.plants.iter() {
            match plant.left() {
                Some(pos) if self.plants.contains(&pos) => {}
                _ => match verticals.entry(plant.0 as isize - 1) {
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(vec![plant.1 + 1000000]);
                    }
                    Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.get_mut().push(plant.1 + 1000000);
                    }
                },
            }
            if !self.plants.contains(&plant.right()) {
                match verticals.entry(plant.0 as isize + 1) {
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(vec![plant.1]);
                    }
                    Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.get_mut().push(plant.1);
                    }
                }
            }
            match plant.up() {
                Some(pos) if self.plants.contains(&pos) => {}
                _ => match horizontals.entry(plant.1 as isize - 1) {
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(vec![plant.0 + 1000000]);
                    }
                    Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.get_mut().push(plant.0 + 1000000);
                    }
                },
            }
            if !self.plants.contains(&plant.down()) {
                match horizontals.entry(plant.1 as isize + 1) {
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(vec![plant.0]);
                    }
                    Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.get_mut().push(plant.0);
                    }
                }
            }
        }

        fn count_contiguous_sides(map: BTreeMap<isize, Vec<usize>>) -> usize {
            map.into_iter()
                .map(|mut v| {
                    v.1.sort();
                    let mut sides = 1;
                    let mut prev_side = v.1.remove(0);
                    for side in v.1.into_iter() {
                        if side - prev_side != 1 {
                            sides += 1;
                        }
                        prev_side = side;
                    }
                    sides
                })
                .sum::<usize>()
        }

        count_contiguous_sides(verticals) + count_contiguous_sides(horizontals)
    }
}

fn find_region(
    position: Position,
    available_positions: &mut BTreeSet<Position>,
    lines: &Vec<Vec<char>>,
) -> Region {
    let id = lines[position.1][position.0];
    let width = lines[0].len();
    let height = lines.len();
    let mut positions = BTreeSet::new();
    positions.insert(position.clone());
    let mut positions_to_check = vec![position];

    while let Some(position) = positions_to_check.pop() {
        // left
        if position.0 > 0 {
            let new_position = Position(position.0 - 1, position.1);
            if lines[new_position.1][new_position.0] == id
                && available_positions.contains(&new_position)
            {
                available_positions.remove(&new_position);
                positions.insert(new_position.clone());
                positions_to_check.push(new_position);
            }
        }
        // right
        if position.0 < width - 1 {
            let new_position = Position(position.0 + 1, position.1);
            if lines[new_position.1][new_position.0] == id
                && available_positions.contains(&new_position)
            {
                available_positions.remove(&new_position);
                positions.insert(new_position.clone());
                positions_to_check.push(new_position);
            }
        }
        // up
        if position.1 > 0 {
            let new_position = Position(position.0, position.1 - 1);
            if lines[new_position.1][new_position.0] == id
                && available_positions.contains(&new_position)
            {
                available_positions.remove(&new_position);
                positions.insert(new_position.clone());
                positions_to_check.push(new_position);
            }
        }
        // down
        if position.1 < height - 1 {
            let new_position = Position(position.0, position.1 + 1);
            if lines[new_position.1][new_position.0] == id
                && available_positions.contains(&new_position)
            {
                available_positions.remove(&new_position);
                positions.insert(new_position.clone());
                positions_to_check.push(new_position);
            }
        }
    }

    Region {
        _id: id,
        plants: positions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const EXAMPLE4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const EXAMPLE5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example1() {
        assert_eq!(solution_part1(EXAMPLE1), 140);
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(solution_part1(EXAMPLE2), 772);
    }

    #[test]
    fn test_part1_example3() {
        assert_eq!(solution_part1(EXAMPLE3), 1930);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 1424472);
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(solution_part2(EXAMPLE1), 80);
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(solution_part2(EXAMPLE2), 436);
    }

    #[test]
    fn test_part2_example3() {
        assert_eq!(solution_part2(EXAMPLE3), 1206);
    }

    #[test]
    fn test_part2_example4() {
        assert_eq!(solution_part2(EXAMPLE4), 236);
    }

    #[test]
    fn test_part2_example5() {
        assert_eq!(solution_part2(EXAMPLE5), 368);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 870202);
    }
}
