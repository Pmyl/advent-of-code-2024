// https://adventofcode.com/2024/day/17

use std::time::Duration;

pub fn solution_part1(input: &str) -> String {
    let mut program = Program::from_input(input);
    program.execute()
}

pub fn solution_part2(input: &str) -> usize {
    let mut program = Program::from_input(input);
    program.find_a_register_to_output_copy_using_reverse2()
}

struct Program {
    registers: [usize; 3],
    start_registers: [usize; 3],
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
            start_registers: registers.clone(),
            registers,
            instructions,
            instruction_pointer: 0,
        }
    }

    fn execute(&mut self) -> String {
        let mut output = Vec::<u8>::new();

        while let Some(maybe_output) = self.execute_instruction() {
            if let Some(out) = maybe_output {
                output.push(out);
            }
        }

        output
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",")
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
        let mut index: usize = 0;
        let mut register_a: usize = 2310851772;

        loop {
            println!("Testing register A {}", register_a);

            loop {
                match self.execute_instruction() {
                    Some(Some(output)) if self.instructions[index] == output => {
                        if self.instructions.len() != index + 1 {
                            index += 1;
                        } else {
                            return register_a;
                        }
                    }
                    Some(None) => {}
                    _ => break,
                }
            }

            register_a += 1;
            index = 0;
            self.restart();
            self.registers[0] = register_a;
        }
    }

    fn find_a_register_to_output_copy_using_reverse2(&mut self) -> usize {
        let mut index: usize = self.instructions.len() - 1;
        let mut register_a: Vec<usize> = vec![0];
        let mut register_c: Vec<usize> = vec![0];
        let mut instruction_pointer = self.instructions.len() - 2;
        let mut complete = false;
        let mut register_b_initial_value = 0;

        loop {
            let mut register_b: Vec<usize> = vec![register_b_initial_value];
            register_b_initial_value += 8;
            println!("\n+++++++Testing for register B {:?}", register_b);

            loop {
                let (Some(op_code), Some(operand)) = (
                    self.instructions.get(instruction_pointer),
                    self.instructions.get(instruction_pointer + 1),
                ) else {
                    panic!("There is no lowest positive initial value for register A that cause the program to output a copy of itself");
                };

                println!("Operation [{}, {}]", op_code, operand);
                // println!("Register A {}", register_a.len());
                println!("Register B {:?}", register_b);

                if instruction_pointer == 0 {
                    instruction_pointer = self.instructions.len() - 2;
                } else {
                    instruction_pointer -= 2;
                }

                match (op_code, operand) {
                    // 3,0 => Ok
                    (3, 0) => {}
                    // 5,5 => Forces B to be a multiple of 8 because the last output is 0
                    (5, 5) => {
                        println!("Checking output {}", self.instructions[index]);
                        register_b = register_b
                            .into_iter()
                            .filter(|b| *b % 8 == self.instructions[index] as usize)
                            .collect::<Vec<_>>();
                        // println!("Register B is now {:?}", register_b);

                        if register_b.is_empty() {
                            println!("XXX Register B does not have possible values");
                            break;
                        }

                        if index == 0 {
                            println!("COMPLETE");
                            complete = true;
                        } else {
                            index -= 1;
                        }
                    }
                    // 1,5 => B = B XOR 5
                    (1, 5) => {
                        register_b = register_b.into_iter().map(|v| v ^ 5).collect::<Vec<_>>();
                        // println!("Register B is now {:?}", register_b);
                    }
                    // 0,3 => A is 0 the first time, now can be 0..7
                    (0, 3) => {
                        let power = 3;
                        let denominator = 2usize.pow(power as u32);

                        register_a = register_a
                            .into_iter()
                            .flat_map(|v| {
                                (0..denominator)
                                    .map(|remainder| v * denominator + remainder)
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>();
                        // println!("Register A is now {:?}", register_a);
                    }
                    // 4,2 => C = B XOR (0,1,2,3,4,5,6,7) and B = B XOR C
                    (4, 2) => {
                        if register_c.is_empty() {
                            let prev_register_b_possibilities = [0, 1, 2, 3, 4, 5, 6, 7];
                            register_c = register_b
                                .iter()
                                .flat_map(|v| {
                                    prev_register_b_possibilities.into_iter().map(|pb| *v ^ pb)
                                })
                                .collect::<Vec<_>>();
                        }

                        register_b = register_b
                            .into_iter()
                            .flat_map(|b| register_c.iter().map(|c| b ^ c).collect::<Vec<_>>())
                            .collect::<Vec<_>>();
                        // println!("Register B is now {:?}", register_b);
                    }
                    // 7,5 => C can be many more values
                    (7, 5) => {
                        // C = A / 2^B
                        register_a = register_c
                            .iter()
                            .flat_map(|v| {
                                register_b
                                    .iter()
                                    .flat_map(|power| {
                                        let denominator = 2usize.pow(*power as u32);
                                        (0..denominator)
                                            .map(|remainder| v * denominator + remainder)
                                            .collect::<Vec<_>>()
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>();
                    }
                    // 1,3 => B = B XOR 3
                    (1, 3) => {
                        register_b = register_b.into_iter().map(|v| v ^ 3).collect::<Vec<_>>();
                        // println!("Register B is now {:?}", register_b);
                    }
                    // 2,4 => Reverse modulo based on B and A, A's filtered values are the ones matching 8n + B
                    (2, 4) => {
                        if register_b.iter().any(|b| *b > 8) {
                            // println!("XXX Should not be greater than 8");
                            break;
                        }

                        // println!("Register A was {:?}", register_a);
                        register_a = register_a
                            .into_iter()
                            .filter(|a| register_b.iter().any(|b| *a == (a / 8) * 8 + *b))
                            .collect::<Vec<_>>();
                        // println!("Register A is {:?}", register_a);

                        if register_a.is_empty() {
                            // println!("No possible values in Register A");
                            break;
                        }
                    }
                    _ => unreachable!(),
                }

                // std::thread::sleep(Duration::from_millis(1000));

                if instruction_pointer == self.instructions.len() - 2 && complete {
                    return register_a.into_iter().min().unwrap();
                }
            }
        }
    }

    fn find_a_register_to_output_copy_using_reverse(&mut self) -> usize {
        let mut index: usize = self.instructions.len() - 1;
        let mut register_a: Vec<usize> = vec![0];
        let mut register_b: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut register_c: Vec<usize> = vec![0];
        let mut instruction_pointer = self.instructions.len() - 2;
        let mut complete = false;

        loop {
            let (Some(op_code), Some(operand)) = (
                self.instructions.get(instruction_pointer),
                self.instructions.get(instruction_pointer + 1),
            ) else {
                panic!("There is no lowest positive initial value for register A that cause the program to output a copy of itself");
            };

            println!("Operation [{}, {}]", op_code, operand);

            if instruction_pointer == 0 {
                instruction_pointer = self.instructions.len() - 2;
            } else {
                instruction_pointer -= 2;
            }

            match op_code {
                0 => {
                    let power = self.combo_operand_value(*operand);
                    let denominator = 2usize.pow(power as u32);
                    println!("Register A has {} possibilities", register_a.len());
                    println!(
                        "A = A' / 2^{} => A = A' / {} => A' = A * {} ... A * {} + {}",
                        power, denominator, denominator, denominator, denominator,
                    );

                    register_a = register_a
                        .into_iter()
                        .flat_map(|v| {
                            (0..denominator)
                                .map(|remainder| v * denominator + remainder)
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();

                    println!("Register A has now {} possibilities", register_a.len());
                }
                1 => {
                    println!("B = B XOR operand");

                    if register_b.is_empty() {
                        panic!("Register B empty");
                    } else {
                        register_b = register_b
                            .into_iter()
                            .map(|v| v ^ self.literal_operand_value(*operand))
                            .collect::<Vec<_>>();
                    }
                }
                2 => {
                    println!("Register B was something, do not know what, blanking");
                    register_b = vec![];
                }
                3 if *operand == 0 => println!("Jump skipped in reverse"),
                3 if *operand != 0 => {
                    panic!(
                        "Don't know how to handle this, it shouldn't be in the input {}, {}",
                        op_code, operand
                    )
                }
                4 => {
                    println!("B = B XOR C");

                    if register_b.is_empty() || register_c.is_empty() {
                        panic!(
                            "Register {} empty",
                            match (register_b.is_empty(), register_c.is_empty()) {
                                (true, true) => "B & C",
                                (true, false) => "B",
                                (false, true) => "C",
                                _ => unreachable!(),
                            }
                        );
                    } else {
                        register_b = register_b
                            .into_iter()
                            .flat_map(|v| register_c.iter().map(|c| v ^ c).collect::<Vec<_>>())
                            .collect::<Vec<_>>();
                    }
                }
                5 => {
                    if complete {
                        panic!("There is no lowest positive initial value for register A that cause the program to output a copy of itself");
                    }

                    match operand {
                        0..=3 => {
                            println!(
                                "Output of {} % 8 should be {}",
                                operand, self.instructions[index]
                            );

                            if *operand != self.instructions[index] {
                                panic!("Can't do it");
                            }

                            println!("It's {}!", operand);
                        }
                        4 | 6 => {
                            panic!("Unhandled, shouldn't be in the input")
                        }
                        5 => {
                            let register_name = match operand {
                                4 => "Register A",
                                5 => "Register B",
                                6 => "Register C",
                                _ => unreachable!(),
                            };
                            let values = match operand {
                                4 => &mut register_a,
                                5 => &mut register_b,
                                6 => &mut register_c,
                                _ => unreachable!(),
                            };
                            println!(
                                "Output of {} % 8 should be {}",
                                register_name, self.instructions[index]
                            );

                            if values.is_empty() {
                                println!(
                                    "Don't know the possible values, since the output is only of B and B is modulo'd 8 then we can just get the value we need I guess?"
                                );
                                values.clear();
                                values.push(self.instructions[index] as usize);
                            } else {
                                let mut new_values = values
                                    .iter()
                                    .map(|v| *v)
                                    .filter(|v| (v % 8) as u8 == self.instructions[index])
                                    .collect::<Vec<_>>();
                                values.clear();
                                values.append(&mut new_values);
                            }

                            if values.is_empty() {
                                panic!("There is no lowest positive initial value for register A that cause the program to output a copy of itself");
                            } else {
                                println!("It can be one of {} values!", values.len());
                                if index == 0 {
                                    complete = true;
                                } else {
                                    index -= 1;
                                }
                            }
                        }
                        _ => panic!("Invalid operand {}", operand),
                    };
                }
                6 => {
                    let power = self.combo_operand_value(*operand);
                    let denominator = 2usize.pow(power as u32);
                    println!("Register B has {} possibilities", register_a.len());
                    println!(
                        "B = A' / 2^{} => B = A' / {} => A' = B * {} ... B * {} + {}",
                        power, denominator, denominator, denominator, denominator,
                    );

                    register_b = register_a
                        .iter()
                        .flat_map(|v| {
                            (0..denominator)
                                .map(|remainder| v * denominator + remainder)
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();

                    println!("Register B has now {} possibilities", register_a.len());
                }
                7 => {
                    // TODO: this gets the wrong registers, it should get the local ones that can have more possibilities
                    let power = self.combo_operand_value(*operand);
                    let denominator = 2usize.pow(power as u32);
                    println!("Register C has {} possibilities", register_a.len());
                    println!(
                        "C = A' / 2^{} => C = A' / {} => A' = C * {} ... C * {} + {}",
                        power, denominator, denominator, denominator, denominator,
                    );

                    register_c = register_a
                        .iter()
                        .flat_map(|v| {
                            (0..denominator)
                                .map(|remainder| v * denominator + remainder)
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();

                    println!("Register C has now {} possibilities", register_a.len());
                }
                _ => panic!("Invalid op code {}", op_code),
            }

            if instruction_pointer == self.instructions.len() - 2 && complete {
                break;
            }
        }

        register_a.into_iter().min().unwrap()
    }

    fn restart(&mut self) {
        self.registers = self.start_registers.clone();
        self.instruction_pointer = 0;
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

    // #[test]
    // fn test_part2_reverse_example() {
    //     assert_eq!(solution_part1(REVERSE_EXAMPLE2), "0,3,5,4,3,0");
    // }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE2), 117440);
    }

    #[test]
    fn test_part2_input() {
        // Program:
        // 2,4 => B = A % 8 can only be one of (0,1,2,3,4,5,6,7)
        // 1,3 => B = B XOR 3 one of (0,1,2,3,4,5,6,7)
        // 7,5 => C = A / 2^B can be len(A) * len(B) values
        // 4,2 => B = B XOR C => can be len(C) * len(B) values
        // 0,3 => A = A / 2^8 can be len(A)^2 values
        // 1,5 => B = B XOR 5 still len(B) values
        // 5,5 => Filter B values
        // 3,0 => Jump
        //
        // Last
        // 2,4 => B = A % 8 can only be one of (0,1,2,3,4,5,6,7)
        // 1,3 => B = B XOR 3 one of (0,1,2,3,4,5,6,7)
        // 7,5 => C = A / 2^B can be len(A) * len(B) values
        // 4,2 => B = B XOR C => can be len(C) * len(B) values
        // 0,3 => A = A / 2^8 can be len(A)^2 values
        // 1,5 => B = B XOR 5 still len(B) values
        // 5,5 => Filter B values
        // 3,0 => Jump
        //
        //
        // First reverse
        // 3,0 => Ok
        // 5,5 => Forces B to be a multiple of 8 because the last output is 0
        // 1,5 => B = B XOR 5
        // 0,3 => A is 0 the first time, now can be 0..7
        // 4,2 => C = B XOR (0,1,2,3,4,5,6,7)
        // 7,5 => C can be many more values
        // 1,3 => B = B XOR 3
        // 2,4 => Reverse modulo based on B and A, A's filtered values are the ones matching 8n + B
        // Tested to 3855407954
        assert_eq!(solution_part2(INPUT), 0);
    }
}
