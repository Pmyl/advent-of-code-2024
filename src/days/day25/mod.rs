// https://adventofcode.com/2024/day/25

pub fn solution_part1(input: &str) -> usize {
    let tumbler_locks = TumblerLocks::from_input(input);
    tumbler_locks.fitting_keys()
}

pub fn solution_part2(_input: &str) -> usize {
    0
}

struct TumblerLocks {
    keys: Vec<[u8; 5]>,
    locks: Vec<[u8; 5]>,
}

impl TumblerLocks {
    fn from_input(input: &str) -> Self {
        let mut keys = vec![];
        let mut locks = vec![];

        for thing_str in input.split("\n\n") {
            let lines = thing_str.lines().collect::<Vec<_>>();
            let [first_line, other_lines @ .., _] = lines.as_slice() else {
                unreachable!("input is predictable");
            };

            let thing = other_lines.iter().fold([0u8; 5], |mut acc, l| {
                for (i, c) in l.chars().enumerate() {
                    if c == '#' {
                        acc[i] += 1;
                    }
                }
                acc
            });

            if first_line == &"#####" {
                locks.push(thing);
            } else {
                keys.push(thing);
            }
        }

        Self { keys, locks }
    }

    fn fitting_keys(&self) -> usize {
        println!("{:?}\n\n{:?}", self.keys, self.locks);
        self.keys
            .iter()
            .map(|key| {
                self.locks
                    .iter()
                    .filter(|lock| {
                        lock[0] + key[0] <= 5
                            && lock[1] + key[1] <= 5
                            && lock[2] + key[2] <= 5
                            && lock[3] + key[3] <= 5
                            && lock[4] + key[4] <= 5
                    })
                    .count()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 3249);
    }

    #[ignore]
    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 0);
    }

    #[ignore]
    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 0);
    }
}
