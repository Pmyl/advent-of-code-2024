// https://adventofcode.com/2024/day/10

use std::collections::HashSet;

pub fn solution_part1(input: &str) -> usize {
    let map = TopographicMap::from_input(input);
    map.count_hiking_trails()
}

pub fn solution_part2(input: &str) -> usize {
    let map = TopographicMap::from_input(input);
    map.rate_trailheads()
}

struct TopographicMap {
    heights: Vec<Vec<u32>>,
}

impl TopographicMap {
    fn from_input(input: &str) -> Self {
        Self {
            heights: input
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| c.to_digit(10).expect("all to be a digit"))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        }
    }

    fn count_hiking_trails(&self) -> usize {
        self.count_trails(false)
    }

    fn rate_trailheads(&self) -> usize {
        self.count_trails(true)
    }

    fn count_trails(&self, repeats: bool) -> usize {
        let trailheads: Vec<(usize, usize)> = self
            .heights
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .filter_map(move |(x, h)| if *h == 0 { Some((x, y)) } else { None })
            })
            .collect::<Vec<_>>();

        let mut found = 0;
        for trailhead in trailheads {
            let mut trails = vec![(0, trailhead)];
            let mut tops = HashSet::new();

            while let Some(trailpoint) = trails.pop() {
                if trailpoint.0 == 9 {
                    if repeats || tops.insert(trailpoint.1) {
                        found += 1;
                    }
                    continue;
                }

                // left
                match (trailpoint.1 .0.checked_sub(1), trailpoint.1 .1) {
                    (None, _) => {}
                    (Some(x), y) => {
                        if self.heights[y][x] == trailpoint.0 + 1 {
                            trails.push((trailpoint.0 + 1, (x, y)));
                        }
                    }
                }

                // up
                match (trailpoint.1 .0, trailpoint.1 .1.checked_sub(1)) {
                    (_, None) => {}
                    (x, Some(y)) => {
                        if self.heights[y][x] == trailpoint.0 + 1 {
                            trails.push((trailpoint.0 + 1, (x, y)));
                        }
                    }
                }

                // right
                let (x, y) = (
                    (trailpoint.1 .0 + 1).min(self.heights[0].len() - 1),
                    trailpoint.1 .1,
                );
                if self.heights[y][x] == trailpoint.0 + 1 {
                    trails.push((trailpoint.0 + 1, (x, y)));
                }

                // down
                let (x, y) = (
                    trailpoint.1 .0,
                    (trailpoint.1 .1 + 1).min(self.heights.len() - 1),
                );
                if self.heights[y][x] == trailpoint.0 + 1 {
                    trails.push((trailpoint.0 + 1, (x, y)));
                }
            }
        }

        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRE_EXAMPLE: &str = "1110111
1111111
1112111
6543456
7111117
8111118
9111119";

    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_pre_example() {
        assert_eq!(solution_part1(PRE_EXAMPLE), 2);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 36);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 652);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 81);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 1432);
    }
}
