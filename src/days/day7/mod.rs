// https://adventofcode.com/2024/day/7

pub fn solution_part1(input: &str) -> usize {
    let calibrations = Calibrations::from_input(input);
    calibrations.sum_possible_calibration_results()
}

pub fn solution_part2(input: &str) -> usize {
    let calibrations = Calibrations::from_input(input);
    calibrations.sum_possible_calibration_results_with_concat()
}

struct Calibrations {
    incomplete_equations: Vec<IncompleteEquation>,
}

struct IncompleteEquation {
    result: usize,
    factors: Vec<usize>,
}

impl Calibrations {
    fn from_input(input: &str) -> Self {
        Self {
            incomplete_equations: input
                .lines()
                .map(|line| {
                    let (left, right) = line.split_once(':').expect("to have a colon");

                    IncompleteEquation {
                        result: left.parse::<usize>().expect("left to be usize"),
                        factors: right
                            .trim()
                            .split(' ')
                            .map(|factor| {
                                factor
                                    .parse::<usize>()
                                    .expect("right to be usizes separated by space")
                            })
                            .collect::<Vec<_>>(),
                    }
                })
                .collect::<Vec<_>>(),
        }
    }

    fn sum_possible_calibration_results(&self) -> usize {
        self.incomplete_equations
            .iter()
            .filter(|incomplete_equation| incomplete_equation.can_be_true())
            .map(|incomplete_equation| incomplete_equation.result)
            .sum()
    }

    fn sum_possible_calibration_results_with_concat(&self) -> usize {
        self.incomplete_equations
            .iter()
            .filter(|incomplete_equation| incomplete_equation.can_be_true_with_concat())
            .map(|incomplete_equation| incomplete_equation.result)
            .sum()
    }
}

impl IncompleteEquation {
    fn can_be_true(&self) -> bool {
        can_be_true(self.result, self.factors[0], &self.factors, 1)
    }

    fn can_be_true_with_concat(&self) -> bool {
        can_be_true_with_concat(self.result, self.factors[0], &self.factors, 1)
    }
}

fn can_be_true(result: usize, incomplete_result: usize, factors: &[usize], index: usize) -> bool {
    if factors.len() == index {
        return result == incomplete_result;
    }

    let next = factors[index];

    let sum = incomplete_result + next;
    let multiplication = incomplete_result * next;

    can_be_true(result, sum, factors, index + 1)
        || can_be_true(result, multiplication, factors, index + 1)
}

fn can_be_true_with_concat(
    result: usize,
    incomplete_result: usize,
    factors: &[usize],
    index: usize,
) -> bool {
    if factors.len() == index {
        return result == incomplete_result;
    }

    let next = factors[index];

    let sum = incomplete_result + next;
    let multiplication = incomplete_result * next;
    let concatenation = fast_concat(incomplete_result, next);

    can_be_true_with_concat(result, sum, factors, index + 1)
        || can_be_true_with_concat(result, multiplication, factors, index + 1)
        || can_be_true_with_concat(result, concatenation, factors, index + 1)
}

fn fast_concat(left: usize, right: usize) -> usize {
    let right_digits: u32 = match right {
        0..10 => 1,
        0..100 => 2,
        0..1000 => 3,
        0..10000 => 4,
        0..100000 => 5,
        0..1000000 => 6,
        0..10000000 => 7,
        0..100000000 => 8,
        0..1000000000 => 9,
        0..10000000000 => 10,
        0..100000000000 => 11,
        0..1000000000000 => 12,
        0..10000000000000 => 13,
        0..100000000000000 => 14,
        0..1000000000000000 => 15,
        0..10000000000000000 => 16,
        0..100000000000000000 => 17,
        0..1000000000000000000 => 18,
        0..10000000000000000000 => 19,
        _ => unreachable!(),
    };

    left * 10usize.pow(right_digits) + right
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 3749);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 12553187650171);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 11387);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 96779702119491);
    }
}
