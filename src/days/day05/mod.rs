// https://adventofcode.com/2024/day/5

use std::collections::HashSet;

pub fn solution_part1(input: &str) -> usize {
    let printer = Printer::from_input(input);
    let correct_updates = printer.get_correct_updates();
    correct_updates
        .into_iter()
        .map(|update| update.pages_to_produce[update.pages_to_produce.len() / 2])
        .sum()
}

pub fn solution_part2(input: &str) -> usize {
    let printer = Printer::from_input(input);
    let correct_updates = printer.get_incorrect_updates_corrected();
    correct_updates
        .into_iter()
        .map(|update| update.pages_to_produce[update.pages_to_produce.len() / 2])
        .sum()
}

struct Printer {
    page_ordering_rules: HashSet<(usize, usize)>,
    updates: Vec<Update>,
}

struct Update {
    pages_to_produce: Vec<usize>,
}

impl Printer {
    fn from_input(input: &str) -> Self {
        let mut found_separator = false;
        let mut page_ordering_rules = HashSet::new();
        let mut updates = vec![];

        input.lines().for_each(|line| {
            if line.is_empty() {
                found_separator = true;
                return;
            }

            if found_separator {
                let update = Update {
                    pages_to_produce: line
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                };
                updates.push(update);
            } else {
                page_ordering_rules.insert(
                    line.split_once('|')
                        .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
                        .unwrap(),
                );
            }
        });

        Self {
            page_ordering_rules,
            updates,
        }
    }

    fn get_correct_updates(self) -> Vec<Update> {
        self.updates
            .into_iter()
            .filter(|u| u.is_correct(&self.page_ordering_rules))
            .collect::<Vec<_>>()
    }

    fn get_incorrect_updates_corrected(self) -> Vec<Update> {
        let incorrect_updates = self
            .updates
            .into_iter()
            .filter(|u| !u.is_correct(&self.page_ordering_rules))
            .collect::<Vec<_>>();

        incorrect_updates
            .into_iter()
            .map(|u| u.correct(&self.page_ordering_rules))
            .collect::<Vec<_>>()
    }
}

impl Update {
    fn is_correct(&self, rules: &HashSet<(usize, usize)>) -> bool {
        let mut i = 0;
        for _ in 0..(self.pages_to_produce.len() - 1) {
            if !rules.contains(&(self.pages_to_produce[i], self.pages_to_produce[i + 1])) {
                return false;
            }

            i += 1;
        }

        true
    }

    fn correct(mut self, rules: &HashSet<(usize, usize)>) -> Self {
        loop {
            let mut i = 0;
            for _ in 0..(self.pages_to_produce.len() - 1) {
                if !rules.contains(&(self.pages_to_produce[i], self.pages_to_produce[i + 1])) {
                    let temp = self.pages_to_produce[i + 1];
                    self.pages_to_produce[i + 1] = self.pages_to_produce[i];
                    self.pages_to_produce[i] = temp;

                    if self.is_correct(rules) {
                        return self;
                    }
                }

                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 143);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 6949);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 123);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 4145);
    }
}
