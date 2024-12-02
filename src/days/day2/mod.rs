// https://adventofcode.com/2024/day/2

pub fn solution_part1(input: &str) -> usize {
    let reports = Reports::from_input(input);
    reports.count_safe(false)
}

pub fn solution_part2(input: &str) -> usize {
    let reports = Reports::from_input(input);
    reports.count_safe(true)
}

struct Report {
    levels: Vec<usize>,
}

struct Reports {
    list: Vec<Report>,
}

impl Reports {
    fn from_input(input: &str) -> Self {
        Self {
            list: input.lines().map(Report::from_input_line).collect(),
        }
    }

    fn count_safe(self, with_problem_dampener: bool) -> usize {
        self.list
            .into_iter()
            .filter(|r| r.is_safe(with_problem_dampener))
            .count()
    }
}

impl Report {
    fn from_input_line(line: &str) -> Self {
        Self {
            levels: line
                .split_whitespace()
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect(),
        }
    }

    fn is_safe(&self, with_problem_dampener: bool) -> bool {
        return are_levels_safe(&self.levels, with_problem_dampener);
    }
}

fn are_levels_safe(levels: &[usize], with_problem_dampener: bool) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut prev_level = levels[0];
    let direction_modifier = if prev_level < levels[1] { -1 } else { 1 };

    for (i, level) in levels.iter().skip(1).enumerate() {
        let difference = (prev_level as isize - *level as isize) * direction_modifier;

        if difference > 3 || difference <= 0 {
            if with_problem_dampener {
                return are_levels_safe(&levels.skip_index(i.max(1) - 1), false)
                    || are_levels_safe(&levels.skip_index(i), false)
                    || are_levels_safe(&levels.skip_index(i + 1), false);
            }

            return false;
        }

        prev_level = *level;
    }

    return true;
}

trait SkipIndex<T> {
    fn skip_index(&self, index: usize) -> Vec<T>;
}

impl<T: Copy> SkipIndex<T> for &[T] {
    fn skip_index(&self, index: usize) -> Vec<T> {
        self.iter()
            .enumerate()
            .filter(|(i, _)| *i != index)
            .map(|(_, item)| *item)
            .collect::<Vec<T>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 2);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 356);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 413);
    }
}
