// https://adventofcode.com/2024/day/13

pub fn solution_part1(input: &str) -> usize {
    let claw_machine = ClawMachine::from_input(input);
    claw_machine.minimum_credits_to_win_all_prizes()
}

pub fn solution_part2(input: &str) -> usize {
    let mut claw_machine = ClawMachine::from_input(input);
    for game in claw_machine.games.iter_mut() {
        game.prize_at.0 += 10000000000000;
        game.prize_at.1 += 10000000000000;
    }
    claw_machine.minimum_credits_to_win_all_prizes()
}

struct ClawMachine {
    games: Vec<ClawMachineGame>,
}

#[derive(Debug)]
struct ClawMachineGame {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize_at: (usize, usize),
}

impl ClawMachine {
    fn from_input(input: &str) -> Self {
        Self {
            games: input
                .split("\n\n")
                .map(|game| ClawMachineGame::from_input(game))
                .collect::<Vec<_>>(),
        }
    }

    fn minimum_credits_to_win_all_prizes(&self) -> usize {
        self.games
            .iter()
            .map(|game| game.minimum_credits_to_win2())
            .sum()
    }
}

impl ClawMachineGame {
    fn from_input(input: &str) -> Self {
        let [a_str, b_str, prize_at_str] = input.splitn(3, '\n').collect::<Vec<_>>()[..] else {
            unreachable!();
        };
        let mut button_a = (0, 0);
        let mut button_b = (0, 0);
        let mut prize_at = (0, 0);
        //Button A: X+*, Y+*
        let mut c_iter = a_str.chars().skip(12);
        while let Some(c) = c_iter.next() {
            if let Some(d) = c.to_digit(10) {
                button_a.0 = button_a.0 * 10 + d as usize;
            } else {
                break;
            }
        }
        c_iter.next();
        c_iter.next();
        c_iter.next();
        while let Some(c) = c_iter.next() {
            if let Some(d) = c.to_digit(10) {
                button_a.1 = button_a.1 * 10 + d as usize;
            } else {
                break;
            }
        }
        //Button B: X+*, Y+*
        let mut c_iter = b_str.chars().skip(12);
        while let Some(c) = c_iter.next() {
            if let Some(d) = c.to_digit(10) {
                button_b.0 = button_b.0 * 10 + d as usize;
            } else {
                break;
            }
        }
        c_iter.next();
        c_iter.next();
        c_iter.next();
        while let Some(c) = c_iter.next() {
            if let Some(d) = c.to_digit(10) {
                button_b.1 = button_b.1 * 10 + d as usize;
            } else {
                break;
            }
        }
        //Prize: X=*, Y=*
        let mut c_iter = prize_at_str.chars().skip(9);
        while let Some(c) = c_iter.next() {
            if let Some(d) = c.to_digit(10) {
                prize_at.0 = prize_at.0 * 10 + d as usize;
            } else {
                break;
            }
        }
        c_iter.next();
        c_iter.next();
        c_iter.next();
        while let Some(c) = c_iter.next() {
            if let Some(d) = c.to_digit(10) {
                prize_at.1 = prize_at.1 * 10 + d as usize;
            } else {
                break;
            }
        }

        Self {
            button_a,
            button_b,
            prize_at,
        }
    }

    fn minimum_credits_to_win2(&self) -> usize {
        let ax = self.button_a.0 as isize;
        let ay = self.button_a.1 as isize;

        let bx = self.button_b.0 as isize;
        let by = self.button_b.1 as isize;

        let px = self.prize_at.0 as isize;
        let py = self.prize_at.1 as isize;

        let b_times = (ax * py - ay * px) / (by * ax - bx * ay);
        let a_times = (px - bx * b_times) / ax;

        let res1 = if a_times * ax + b_times * bx != px || a_times * ay + b_times * by != py {
            usize::MAX
        } else {
            a_times as usize * 3 + b_times as usize
        };

        let a_times = (bx * py - by * px) / (ay * bx - ax * by);
        let b_times = (px - ax * a_times) / bx;

        let res2 = if a_times * ax + b_times * bx != px || a_times * ay + b_times * by != py {
            usize::MAX
        } else {
            a_times as usize * 3 + b_times as usize
        };

        let min = res1.min(res2);

        if min == usize::MAX {
            0
        } else {
            min
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 480);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 27157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 104015411578548);
    }
}
