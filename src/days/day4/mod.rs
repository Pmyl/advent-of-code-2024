// https://adventofcode.com/2024/day/4

use input_iterators::{DiagonalBlTr, DiagonalBrTl, Horizontal, Vertical, WordSearchItem};

mod input_iterators;

pub fn solution_part1(input: &str) -> usize {
    let word_finder = WordFinder::new(input);
    word_finder.count_word("xmas")
}

pub fn solution_part2(input: &str) -> usize {
    let word_finder = WordFinder::new(input);
    word_finder.count_x_word("mas")
}

struct WordFinder {
    lines: Vec<Vec<char>>,
}

impl WordFinder {
    fn new(text: &str) -> Self {
        Self {
            lines: text
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>(),
        }
    }

    fn count_word(&self, word: &str) -> usize {
        let letters: Vec<char> = word.chars().collect();
        let mut found: usize = 0;

        // horizontal left to right
        found += find_matches(Horizontal::new(&self.lines), &letters, false).len();
        // horizontal right to left
        found += find_matches(Horizontal::new(&self.lines), &reverse(&letters), false).len();
        // vertical top to bottom
        found += find_matches(Vertical::new(&self.lines), &letters, false).len();
        // vertical bottom to top
        found += find_matches(Vertical::new(&self.lines), &reverse(&letters), false).len();
        // diagonal bottom-right to top-left
        found += find_matches(DiagonalBrTl::new(&self.lines), &letters, false).len();
        // diagonal top-left to bottom-right
        found += find_matches(DiagonalBrTl::new(&self.lines), &reverse(&letters), false).len();
        // diagonal bottom-left to top-right
        found += find_matches(DiagonalBlTr::new(&self.lines), &letters, false).len();
        // diagonal top-right to bottom-left
        found += find_matches(DiagonalBlTr::new(&self.lines), &reverse(&letters), false).len();

        found
    }

    fn count_x_word(&self, word: &str) -> usize {
        if word.len() % 2 != 1 {
            panic!("This only works with even number of letters");
        }

        let letters: Vec<char> = word.chars().collect();

        // diagonal bottom-right to top-left
        let br_tl = find_matches(DiagonalBrTl::new(&self.lines), &letters, false);
        // diagonal top-left to bottom-right
        let tl_br = find_matches(DiagonalBrTl::new(&self.lines), &reverse(&letters), false);
        // diagonal bottom-left to top-right
        let bl_tr = find_matches(DiagonalBlTr::new(&self.lines), &letters, false);
        // diagonal top-right to bottom-left
        let tr_bl = find_matches(DiagonalBlTr::new(&self.lines), &reverse(&letters), false);

        let centers_br_tl = br_tl
            .into_iter()
            .map(|m| m[m.len() / 2])
            .chain(tl_br.into_iter().map(|m| m[m.len() / 2]));

        let centers_bl_tr = bl_tr
            .into_iter()
            .map(|m| m[m.len() / 2])
            .chain(tr_bl.into_iter().map(|m| m[m.len() / 2]))
            .collect::<Vec<_>>();

        centers_br_tl
            .into_iter()
            .filter(|i| centers_bl_tr.contains(&i))
            .count()
    }
}

fn find_matches<'a>(
    iter: impl Iterator<Item = WordSearchItem>,
    letters: &Vec<char>,
    print: bool,
) -> Vec<Vec<(usize, usize)>> {
    let mut current_letter_i = 0;
    let word_length = letters.len();
    let mut matches: Vec<Vec<(usize, usize)>> = vec![];
    let mut current_match = vec![];

    if print {
        println!("Finding matches for {:?}", letters);
    }

    for item in iter {
        if let WordSearchItem::Letter(c, pos) = item {
            if print {
                print!("{}", c);
            }

            if c.eq_ignore_ascii_case(&letters[current_letter_i]) {
                if word_length == current_letter_i + 1 {
                    if print {
                        print!("!");
                    }
                    current_letter_i = 0;
                    current_match.push(pos);
                    matches.push(current_match);
                    current_match = vec![];
                } else {
                    current_letter_i += 1;
                    current_match.push(pos);
                }
            } else {
                current_match.clear();
                current_letter_i = if c.eq_ignore_ascii_case(&letters[0]) {
                    current_match.push(pos);
                    1
                } else {
                    0
                };
            }
        } else {
            current_match.clear();
            current_letter_i = 0;
            if print {
                println!();
            }
        }
    }

    if print {
        println!();
    }

    matches
}

fn reverse(letters: &Vec<char>) -> Vec<char> {
    letters.iter().rev().map(|l| *l).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 18);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 2521);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 9);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 1912);
    }
}
