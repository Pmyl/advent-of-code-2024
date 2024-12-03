// https://adventofcode.com/2024/day/3

use std::iter::Peekable;

macro_rules! check_char {
    ($c:expr,$chars:ident) => {
        if let Some($c) = $chars.peek() {
            $chars.next();
        } else {
            continue;
        }
    };
}

pub fn solution_part1(input: &str) -> usize {
    let mut sum = 0;
    let mut chars = input.chars().peekable();

    while let Some(char) = chars.next() {
        if char == 'm' {
            check_char!('u', chars);
            check_char!('l', chars);
            check_char!('(', chars);
            let Some(left) = get_numbers(&mut chars) else {
                continue;
            };
            check_char!(',', chars);
            let Some(right) = get_numbers(&mut chars) else {
                continue;
            };
            check_char!(')', chars);
            sum += left * right;
        }
    }

    sum
}

pub fn solution_part2(input: &str) -> usize {
    let mut sum = 0;
    let mut chars = input.chars().peekable();
    let mut should_do = true;

    while let Some(char) = chars.next() {
        if should_do && char == 'm' {
            check_char!('u', chars);
            check_char!('l', chars);
            check_char!('(', chars);
            let Some(left) = get_numbers(&mut chars) else {
                continue;
            };
            check_char!(',', chars);
            let Some(right) = get_numbers(&mut chars) else {
                continue;
            };
            check_char!(')', chars);
            sum += left * right;
        }
        if char == 'd' {
            check_char!('o', chars);
            match chars.peek() {
                Some('(') => {
                    chars.next();
                    check_char!(')', chars);
                    should_do = true;
                    continue;
                }
                Some('n') => {
                    chars.next();
                    check_char!('\'', chars);
                    check_char!('t', chars);
                    check_char!('(', chars);
                    check_char!(')', chars);
                    should_do = false;
                }
                _ => continue,
            }
        }
    }

    sum
}

fn get_numbers<T: Iterator<Item = char>>(chars: &mut Peekable<T>) -> Option<usize> {
    if let Some(c) = chars.peek() {
        if !c.is_numeric() {
            return None;
        }
    } else {
        return None;
    }

    let mut num: usize = 0;

    while let Some(c) = chars.peek() {
        if let Some(digit) = c.to_digit(10) {
            num = num * 10 + digit as usize;
            chars.next();
        } else {
            break;
        }
    }

    Some(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE1), 161);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 173529487);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE2), 48);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 99532691);
    }
}
