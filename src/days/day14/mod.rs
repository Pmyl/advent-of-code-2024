// https://adventofcode.com/2024/day/14

use std::usize;

use crate::{Distance, Position};

pub fn solution_part1(input: &str, width: usize, height: usize) -> usize {
    let robots = RobotsMovements::from_input(input);
    robots.move_for_seconds_and_check_quadrants(100, width, height)
}

pub fn solution_part2(input: &str, width: usize, height: usize) -> usize {
    let robots = RobotsMovements::from_input(input);
    robots.move_until_pattern(width, height)
}

struct RobotsMovements {
    robots: Vec<RobotMovements>,
}

impl RobotsMovements {
    fn from_input(input: &str) -> Self {
        Self {
            robots: input
                .lines()
                .map(RobotMovements::from_input)
                .collect::<Vec<_>>(),
        }
    }

    fn move_for_seconds_and_check_quadrants(
        &self,
        seconds: usize,
        width: usize,
        height: usize,
    ) -> usize {
        let positions = self
            .robots
            .iter()
            .map(|robot| robot.where_is_after_seconds(seconds, width, height))
            .collect::<Vec<_>>();

        self.calculate_score(positions, width, height)
    }

    fn calculate_score(&self, mut positions: Vec<Position>, width: usize, height: usize) -> usize {
        let quadrants = vec![
            (Position(0, 0), Position(width / 2 - 1, height / 2 - 1)),
            (
                Position(width / 2 + 1, 0),
                Position(width - 1, height / 2 - 1),
            ),
            (
                Position(0, height / 2 + 1),
                Position(width / 2 - 1, height - 1),
            ),
            (
                Position(width / 2 + 1, height / 2 + 1),
                Position(width - 1, height - 1),
            ),
        ];

        quadrants
            .into_iter()
            .map(|quadrant| {
                let mut how_many = 0;

                for i in (0..positions.len()).rev() {
                    if positions[i].0 >= quadrant.0 .0
                        && positions[i].0 <= quadrant.1 .0
                        && positions[i].1 >= quadrant.0 .1
                        && positions[i].1 <= quadrant.1 .1
                    {
                        positions.remove(i);
                        how_many += 1;
                    }
                }

                how_many
            })
            .fold(1, |acc, x| acc * x)
    }

    fn move_until_pattern(&self, width: usize, height: usize) -> usize {
        let mut seconds_with_min_score = 0;
        let mut min_score = usize::MAX;

        for seconds in 1..=width * height {
            let new_positions: Vec<Position> = self
                .robots
                .iter()
                .map(|robot| robot.where_is_after_seconds(seconds, width, height))
                .collect();

            let new_score = self.calculate_score(new_positions, width, height);

            if new_score < min_score {
                seconds_with_min_score = seconds;
                min_score = new_score;
            }
        }

        seconds_with_min_score
    }
}

struct RobotMovements {
    pos: Position,
    velocity: Distance,
}

impl RobotMovements {
    fn from_input(input: &str) -> Self {
        let (pos_input, vel_input) = input.trim().split_once(' ').unwrap();
        let (pos_x_input, pos_y_input) = pos_input
            .strip_prefix("p=")
            .unwrap()
            .split_once(',')
            .unwrap();
        let (vel_x_input, vel_y_input) = vel_input
            .strip_prefix("v=")
            .unwrap()
            .split_once(',')
            .unwrap();

        Self {
            pos: Position(
                pos_x_input.parse::<usize>().unwrap(),
                pos_y_input.parse::<usize>().unwrap(),
            ),
            velocity: Distance(
                vel_x_input.parse::<isize>().unwrap(),
                vel_y_input.parse::<isize>().unwrap(),
            ),
        }
    }

    fn where_is_after_seconds(&self, seconds: usize, width: usize, height: usize) -> Position {
        self.pos
            .move_by_wrapping(&self.velocity.multiply(seconds), width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE, 11, 7), 12);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT, 101, 103), 229868730);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT, 101, 103), 7861);
    }
}
