// https://adventofcode.com/2024/day/22

use std::collections::{hash_map::Entry, HashMap, HashSet};

use crate::debug_pause;

pub fn solution_part1(input: &str) -> usize {
    let market = MonkeyMarket::from_input(input);
    market.sum_nth_secret_numbers(2000)
}

pub fn solution_part2(input: &str) -> usize {
    let market = MonkeyMarket::from_input(input);
    market.most_bananas_after_same_sequence_for_nth_secret_numbers(2000)
}

struct MonkeyMarket {
    initial_secret_numbers: Vec<usize>,
}

impl MonkeyMarket {
    fn from_input(input: &str) -> Self {
        Self {
            initial_secret_numbers: input
                .lines()
                .map(|l| l.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        }
    }

    fn sum_nth_secret_numbers(self, nth: usize) -> usize {
        self.initial_secret_numbers
            .into_iter()
            .map(|sn| {
                (0..nth).fold(sn, |mut acc, _| {
                    acc = ((acc * 64) ^ acc) % 16777216;
                    acc = ((acc / 32) ^ acc) % 16777216;
                    acc = ((acc * 2048) ^ acc) % 16777216;
                    acc
                })
            })
            .sum()
    }

    fn most_bananas_after_same_sequence_for_nth_secret_numbers(self, nth: usize) -> usize {
        // Sequence encoded
        // sequence          X  Y  Z  K
        // negatives go      1  2  4  8 (max 14)
        // numbers go 10^abs 2  3  4  5 (max 999900)
        // can be                        max 999914
        //
        // e.g.             -2  1 -1  3
        // e.g. negatives    1  0  4  0 = 5
        // e.g. numbers     200 1000 10000 300000 = 311200
        // e.g. encoded      5 + 311200 = 311205

        fn next_secret_number(mut sn: usize) -> usize {
            sn = ((sn * 64) ^ sn) % 16777216;
            sn = ((sn / 32) ^ sn) % 16777216;
            ((sn * 2048) ^ sn) % 16777216
        }

        let mut price_changes_encoded = vec![0; 999914];

        for secret_number in self.initial_secret_numbers {
            let sn1 = secret_number;
            let sn2 = next_secret_number(sn1);
            let sn3 = next_secret_number(sn2);
            let sn4 = next_secret_number(sn3);
            let mut history = [sn1, sn2, sn3, sn4];
            let mut snh_index: usize = 0;

            let mut encodings = HashSet::<usize>::new();

            for _ in 3..nth {
                let current_secret_number = history[(snh_index + 3) % 4];
                let next_secret_number = next_secret_number(current_secret_number);

                let difference1 = (history[(snh_index + 1) % 4] % 10) as isize
                    - (history[snh_index % 4] % 10) as isize;
                let difference2 = (history[(snh_index + 2) % 4] % 10) as isize
                    - (history[(snh_index + 1) % 4] % 10) as isize;
                let difference3 = (history[(snh_index + 3) % 4] % 10) as isize
                    - (history[(snh_index + 2) % 4] % 10) as isize;
                let difference4 = (next_secret_number % 10) as isize
                    - (history[(snh_index + 3) % 4] % 10) as isize;

                let encoding1 = difference1.abs() as usize * 10usize.pow(2)
                    + if difference1.is_negative() { 1 } else { 0 };
                let encoding2 = difference2.abs() as usize * 10usize.pow(3)
                    + if difference2.is_negative() { 2 } else { 0 };
                let encoding3 = difference3.abs() as usize * 10usize.pow(4)
                    + if difference3.is_negative() { 4 } else { 0 };
                let encoding4 = difference4.abs() as usize * 10usize.pow(5)
                    + if difference4.is_negative() { 8 } else { 0 };

                let encoding: usize = encoding1 + encoding2 + encoding3 + encoding4;
                let price: usize = next_secret_number % 10;

                if !encodings.contains(&encoding) {
                    price_changes_encoded[encoding] += price;
                    encodings.insert(encoding);
                }

                history[snh_index] = next_secret_number;
                snh_index = (snh_index + 1) % 4;
            }
        }

        price_changes_encoded.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1
10
100
2024";

    const EXAMPLE2: &str = "1
2
3
2024";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_pre_example() {
        let result = MonkeyMarket {
            initial_secret_numbers: vec![123],
        }
        .sum_nth_secret_numbers(1);
        assert_eq!(result, 15887950);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 37327623);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 13185239446);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE2), 23);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 1501);
    }
}
