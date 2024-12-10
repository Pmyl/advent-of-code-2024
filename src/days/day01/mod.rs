// https://adventofcode.com/2024/day/1

use std::collections::{hash_map::Entry, HashMap};

pub fn solution_part1(input: &str) -> usize {
    let mut lists = Lists::from_input(input);

    lists.left.sort();
    lists.right.sort();

    lists
        .left
        .iter()
        .zip(lists.right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

pub fn solution_part2(input: &str) -> usize {
    let lists = Lists::from_input(input);

    let mut right_match_map = HashMap::<usize, usize>::new();

    for right in lists.right {
        match right_match_map.entry(right) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
            Entry::Occupied(mut occupied_entry) => {
                *occupied_entry.get_mut() += 1;
            }
        }
    }

    let mut sum = 0;

    for left in lists.left {
        let how_many_times = right_match_map.get(&left);

        if let Some(how_many_times) = how_many_times {
            sum += left * how_many_times;
        }
    }

    sum
}

struct Lists {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl Lists {
    fn from_input(input: &str) -> Self {
        let mut left_list = vec![];
        let mut right_list = vec![];

        for (left, right) in input.lines().map(|l| l.split_once("   ").unwrap()) {
            left_list.push(left.parse::<usize>().unwrap());
            right_list.push(right.parse::<usize>().unwrap());
        }

        Self {
            left: left_list,
            right: right_list,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 11);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 1258579);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 23981443);
    }
}
