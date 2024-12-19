// https://adventofcode.com/2024/day/19

use std::collections::HashMap;

pub fn solution_part1(input: &str) -> usize {
    let patterns = OnsenBranding::from_input(input);
    patterns.how_many_possible()
}

pub fn solution_part2(input: &str) -> usize {
    let patterns = OnsenBranding::from_input(input);
    patterns.how_many_different_ways_possible()
}

struct OnsenBranding {
    available: Vec<Vec<char>>,
    patterns: Vec<Vec<char>>,
}

impl OnsenBranding {
    fn from_input(input: &str) -> Self {
        let mut lines = input.lines();
        let available = lines
            .next()
            .unwrap()
            .split(',')
            .map(|a| a.trim().chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        lines.next().unwrap();

        let patterns = lines
            .map(|a| a.trim().chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        Self {
            available,
            patterns,
        }
    }

    fn how_many_possible(&self) -> usize {
        self.patterns
            .iter()
            .filter(|p| how_many_possible_recursive(0, &p, &self.available))
            .count()
    }

    fn how_many_different_ways_possible(&self) -> usize {
        let mut total = 0;
        let mut cache = HashMap::<Vec<char>, usize>::new();

        for pattern in self.patterns.iter() {
            total += how_many_different_ways_possible_recursive(
                0,
                &pattern,
                &self.available,
                &mut cache,
            );
        }

        total
    }
}

fn how_many_possible_recursive(start: usize, pattern: &[char], available: &[Vec<char>]) -> bool {
    if start == pattern.len() {
        return true;
    }

    for a in available {
        if does_match(a, &pattern, start)
            && how_many_possible_recursive(start + a.len(), &pattern, &available)
        {
            return true;
        }
    }

    false
}

fn how_many_different_ways_possible_recursive(
    start: usize,
    pattern: &[char],
    available: &[Vec<char>],
    cache: &mut HashMap<Vec<char>, usize>,
) -> usize {
    if start == pattern.len() {
        return 1;
    }

    let current_pattern = pattern.iter().skip(start).map(|c| *c).collect::<Vec<_>>();
    if let Some(count) = cache.get(&current_pattern) {
        return *count;
    }

    let mut total = 0;

    for a in available {
        if does_match(a, &pattern, start) {
            total += how_many_different_ways_possible_recursive(
                start + a.len(),
                &pattern,
                &available,
                cache,
            );
        }
    }

    cache.insert(current_pattern, total);
    total
}

fn does_match(available: &[char], pattern: &[char], pattern_start: usize) -> bool {
    if available.len() > pattern.len() - pattern_start {
        return false;
    }

    for (i, c) in available.iter().enumerate() {
        if *c != pattern[i + pattern_start] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 228);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 16);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 584553405070389);
    }
}
