// https://adventofcode.com/2024/day/17

pub fn solution_part1(input: &str) -> String {
    let mut program = Program::from_input(input);
    program.execute()
}

pub fn solution_part2(input: &str) -> usize {
    let mut program = Program::from_input(input);
    program.find_a_register_to_output_copy()
}

struct Program {
    registers: [usize; 3],
    instructions: Vec<u8>,
    instruction_pointer: usize,
}

impl Program {
    fn from_input(input: &str) -> Self {
        let mut registers = [usize::MAX; 3];
        let mut lines = input.lines();
        registers[0] = parse_input_with_prefix("Register A: ", &mut lines)
            .parse::<usize>()
            .unwrap();
        registers[1] = parse_input_with_prefix("Register B: ", &mut lines)
            .parse::<usize>()
            .unwrap();
        registers[2] = parse_input_with_prefix("Register C: ", &mut lines)
            .parse::<usize>()
            .unwrap();
        lines.next().unwrap();
        let instructions = parse_input_with_prefix("Program: ", &mut lines)
            .split(',')
            .map(|o| o.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        Self {
            registers,
            instructions,
            instruction_pointer: 0,
        }
    }

    fn execute(&mut self) -> String {
        self.execute_bit_output()
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }

    fn execute_bit_output(&mut self) -> Vec<u8> {
        let mut output = Vec::<u8>::new();

        while let Some(maybe_output) = self.execute_instruction() {
            if let Some(out) = maybe_output {
                output.push(out);
            }
        }

        output
    }

    fn execute_instruction(&mut self) -> Option<Option<u8>> {
        if let (Some(op_code), operand_option) = (
            self.instructions.get(self.instruction_pointer),
            self.instructions.get(self.instruction_pointer + 1),
        ) {
            self.instruction_pointer += 2;

            match op_code {
                0 => {
                    self.registers[0] = self.registers[0]
                        / 2usize.pow(self.combo_operand_value(*operand_option.unwrap()) as u32)
                }
                1 => {
                    self.registers[1] =
                        self.registers[1] ^ self.literal_operand_value(*operand_option.unwrap())
                }
                2 => self.registers[1] = self.combo_operand_value(*operand_option.unwrap()) % 8,
                3 => {
                    if self.registers[0] != 0 {
                        self.instruction_pointer =
                            self.literal_operand_value(*operand_option.unwrap());
                    }
                }
                4 => self.registers[1] = self.registers[1] ^ self.registers[2],
                5 => {
                    return Some(Some(
                        (self.combo_operand_value(*operand_option.unwrap()) % 8) as u8,
                    ))
                }
                6 => {
                    self.registers[1] = self.registers[0]
                        / 2usize.pow(self.combo_operand_value(*operand_option.unwrap()) as u32)
                }
                7 => {
                    self.registers[2] = self.registers[0]
                        / 2usize.pow(self.combo_operand_value(*operand_option.unwrap()) as u32)
                }
                _ => panic!("Invalid op code {}", op_code),
            }

            Some(None)
        } else {
            None
        }
    }

    fn find_a_register_to_output_copy(&mut self) -> usize {
        let current_bit = self.instructions.len() - 1;

        fn recursive(program: &mut Program, mut a: usize, current_bit: usize) -> Option<usize> {
            for _ in 0..8 {
                program.registers = [a, 0, 0];
                program.instruction_pointer = 0;

                let output = program.execute_bit_output();

                if output.len() >= current_bit
                    && output[current_bit] == program.instructions[current_bit]
                {
                    if current_bit == 0 {
                        return Some(a);
                    } else if let Some(register_a) = recursive(program, a, current_bit - 1) {
                        return Some(register_a);
                    }
                }

                a += 8usize.pow(current_bit as u32);
            }

            None
        }

        recursive(self, 0, current_bit).expect("a solution to exists")
    }

    fn combo_operand_value(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("Invalid operand value: {}", operand),
        }
    }

    fn literal_operand_value(&self, operand: u8) -> usize {
        match operand {
            0..=7 => operand as usize,
            _ => panic!("Invalid operand value: {}", operand),
        }
    }
}

fn parse_input_with_prefix<'a, I: Iterator<Item = &'a str>>(
    prefix: &str,
    lines: &mut I,
) -> &'a str {
    lines.next().unwrap().strip_prefix(prefix).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRE_EXAMPLE1: &str = "Register A: 0
Register B: 0
Register C: 9

Program: 2,6";

    const PRE_EXAMPLE2: &str = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";

    const PRE_EXAMPLE3: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const PRE_EXAMPLE4: &str = "Register A: 0
Register B: 29
Register C: 0

Program: 1,7";

    const PRE_EXAMPLE5: &str = "Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0";

    const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EXAMPLE2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    const REVERSE_EXAMPLE2: &str = "Register A: 117441
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_pre_example1() {
        let mut program = Program::from_input(PRE_EXAMPLE1);
        program.execute();

        assert_eq!(program.registers[1], 1);
    }

    #[test]
    fn test_part1_pre_example2() {
        assert_eq!(solution_part1(PRE_EXAMPLE2), "0,1,2");
    }

    #[test]
    fn test_part1_pre_example3() {
        assert_eq!(solution_part1(PRE_EXAMPLE3), "4,2,5,6,7,7,7,7,3,1,0");
    }

    #[test]
    fn test_part1_pre_example4() {
        let mut program = Program::from_input(PRE_EXAMPLE4);
        program.execute();

        assert_eq!(program.registers[1], 26);
    }

    #[test]
    fn test_part1_pre_example5() {
        let mut program = Program::from_input(PRE_EXAMPLE5);
        program.execute();

        assert_eq!(program.registers[1], 44354);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), "1,7,6,5,1,0,5,0,7");
    }

    #[test]
    fn test_part2_reverse_example() {
        assert_eq!(solution_part1(REVERSE_EXAMPLE2), "0,3,5,4,3,0");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE2), 117440);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(solution_part2(INPUT), 236555995274861);
    }
}
