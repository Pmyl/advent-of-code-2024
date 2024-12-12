// https://adventofcode.com/2024/day/11

use std::collections::HashMap;

pub fn solution_part1(input: &str) -> usize {
    let stones = Stones::from_input(input);
    stones.count_stones_after_blinking(25)
}

pub fn solution_part2(input: &str) -> usize {
    let stones = Stones::from_input(input);
    stones.count_stones_after_blinking(75)
}

struct Stones {
    line: Vec<usize>,
}

impl Stones {
    fn from_input(input: &str) -> Self {
        Self {
            line: input
                .trim()
                .split(' ')
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>(),
        }
    }

    fn count_stones_after_blinking(&self, times: usize) -> usize {
        let mut stones = 0;

        let mut map = HashMap::new();
        for stone in self.line.iter() {
            stones += blink_recursive(*stone, times, &mut map);
        }

        stones
    }
}

fn blink_recursive(stone: usize, times: usize, map: &mut HashMap<(usize, usize), usize>) -> usize {
    if times == 0 {
        1
    } else if let Some(stones) = map.get(&(stone, times)) {
        *stones
    } else if stone == 0 {
        let stones = blink_recursive(1, times - 1, map);
        map.insert((stone, times), stones);
        stones
    } else if let Some((left, right)) = cut_even_number_of_digits(stone) {
        let stones_left = blink_recursive(left, times - 1, map);
        let stones_right = blink_recursive(right, times - 1, map);
        map.insert((stone, times), stones_left + stones_right);
        stones_left + stones_right
    } else {
        let stones = blink_recursive(stone * 2024, times - 1, map);
        map.insert((stone, times), stones);
        stones
    }
}

fn cut_even_number_of_digits(stone: usize) -> Option<(usize, usize)> {
    let digits = match stone {
        10..100 => 2,
        1000..10000 => 4,
        100000..1000000 => 6,
        10000000..100000000 => 8,
        1000000000..10000000000 => 10,
        100000000000..1000000000000 => 12,
        10000000000000..100000000000000 => 14,
        1000000000000000..10000000000000000 => 16,
        100000000000000000..1000000000000000000 => 18,
        _ => 1,
    };
    if digits % 2 == 0 {
        Some((
            stone / 10usize.pow(digits / 2),
            stone % 10usize.pow(digits / 2),
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "125 17";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 55312);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 186996);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 221683913164898);
    }
}
