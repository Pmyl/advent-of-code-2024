// https://adventofcode.com/2024/day/24

use std::collections::HashMap;

pub fn solution_part1(input: &str) -> usize {
    let device = MonitoringDevice::from_input(input);
    device.z_output()
}

pub fn solution_part2(
    input: &str,
    swapped: usize,
    simulated_operation: SimulatedOperation,
) -> String {
    let device = MonitoringDevice::from_input(input);
    device.swapped_to_sum_x_y_equals_z(swapped, simulated_operation)
}

struct MonitoringDevice<'a> {
    initial_values: HashMap<&'a str, u32>,
    connections: Vec<Connection<'a>>,
}

#[derive(Clone)]
struct Connection<'a> {
    left: &'a str,
    right: &'a str,
    gate: Gate,
    result: &'a str,
}

#[derive(PartialEq, Debug, Clone)]
enum Gate {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
pub enum SimulatedOperation {
    Sum,
    And,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Pair(usize, usize);

impl Pair {
    const fn new() -> Self {
        Pair(0, 1)
    }

    fn next(&mut self, max: usize) -> bool {
        if self.1 + 1 == max {
            if self.0 + 2 == max {
                return false;
            }

            self.0 += 1;
            self.1 = self.0 + 1;
        } else {
            self.1 += 1;
        }

        true
    }

    fn reset(&mut self) {
        self.0 = 0;
        self.1 = 1;
    }
}

impl<'a> MonitoringDevice<'a> {
    fn from_input(input: &'a str) -> Self {
        let lines = input.lines().collect::<Vec<&'a str>>();

        let (initial_values_lines, connections_lines) = lines.split_once(|l| *l == "").unwrap();

        let initial_values = initial_values_lines
            .iter()
            .map(|l| {
                let (name, value) = l.split_once(": ").unwrap();
                (name, value.parse::<u32>().unwrap())
            })
            .collect::<HashMap<_, _>>();

        let connections = connections_lines
            .iter()
            .map(|l| {
                let (operation, result) = l.split_once(" -> ").unwrap();
                let mut operation_parts = operation.splitn(3, " ");
                let (left, gate, right) = (
                    operation_parts.next().unwrap(),
                    operation_parts.next().unwrap(),
                    operation_parts.next().unwrap(),
                );

                Connection {
                    left,
                    right,
                    result,
                    gate: match gate {
                        "AND" => Gate::And,
                        "OR" => Gate::Or,
                        "XOR" => Gate::Xor,
                        _ => panic!("invalid"),
                    },
                }
            })
            .collect::<Vec<_>>();

        Self {
            initial_values,
            connections,
        }
    }

    fn z_output(&self) -> usize {
        self.z_output_internal(self.initial_values.clone(), &self.connections)
    }

    fn swapped_to_sum_x_y_equals_z(
        &self,
        swapped: usize,
        simulated_operation: SimulatedOperation,
    ) -> String {
        let x: usize = calculate_value_of_wires(&self.initial_values, "x");
        let y: usize = calculate_value_of_wires(&self.initial_values, "y");
        let expected_z = simulated_operation.apply(x, y);

        let mut connections = self.connections.clone();
        let len = self.connections.len();

        let mut pairs = vec![Pair::new(); swapped];
        let mut maybe_pairs: HashMap<Vec<Pair>, bool> = HashMap::new();

        loop {
            let mut i_to_move = 0;
            while i_to_move != swapped {
                if !pairs[i_to_move].next(len) {
                    pairs[i_to_move].reset();
                    i_to_move += 1;
                } else {
                    break;
                }
            }

            let mut pairs_to_save = pairs.clone();
            pairs_to_save.sort();

            if maybe_pairs.contains_key(&pairs_to_save) {
                continue;
            }

            if i_to_move == swapped {
                break;
            }

            for pair in pairs.iter() {
                let temp = connections[pair.0].result;
                connections[pair.0].result = connections[pair.1].result;
                connections[pair.1].result = temp;
            }

            let z = self.z_output_internal(self.initial_values.clone(), &connections);

            for pair in pairs.iter().rev() {
                let temp = connections[pair.0].result;
                connections[pair.0].result = connections[pair.1].result;
                connections[pair.1].result = temp;
            }

            if expected_z == z {
                maybe_pairs.insert(pairs_to_save, true);
            } else {
                maybe_pairs.insert(pairs_to_save, false);
            }
        }

        let input_bits = self
            .initial_values
            .iter()
            .filter(|(k, _)| k.starts_with("x"))
            .count();

        let maybe_pairs = maybe_pairs.into_iter().map(|(k, _)| k).collect::<Vec<_>>();

        self.find_pairs_to_swap(simulated_operation, connections, input_bits, maybe_pairs)
    }

    fn find_pairs_to_swap(
        &self,
        simulated_operation: SimulatedOperation,
        mut connections: Vec<Connection<'a>>,
        input_bits: usize,
        mut maybe_pairs: Vec<Vec<Pair>>,
    ) -> String {
        for x in 0..2usize.pow(input_bits as u32) {
            for y in 0..2usize.pow(input_bits as u32) {
                for i in (0..maybe_pairs.len()).rev() {
                    for pair in maybe_pairs[i].iter() {
                        let temp = connections[pair.0].result;
                        connections[pair.0].result = connections[pair.1].result;
                        connections[pair.1].result = temp;
                    }

                    let mut values = self.initial_values.clone();
                    for xi in 0..input_bits {
                        *values.get_mut(format!("x{:0>2}", xi).as_str()).unwrap() =
                            if x & (1 << xi) != 0 { 1 } else { 0 };
                    }
                    for yi in 0..input_bits {
                        *values.get_mut(format!("y{:0>2}", yi).as_str()).unwrap() =
                            if y & (1 << yi) != 0 { 1 } else { 0 };
                    }

                    let z = self.z_output_internal(values, &connections);

                    for pair in maybe_pairs[i].iter().rev() {
                        let temp = connections[pair.0].result;
                        connections[pair.0].result = connections[pair.1].result;
                        connections[pair.1].result = temp;
                    }

                    if simulated_operation.apply(x, y) != z {
                        maybe_pairs.remove(i);
                    }
                }

                if maybe_pairs.len() == 1 {
                    let mut connections_to_swap = maybe_pairs[0]
                        .iter()
                        .flat_map(|pair| {
                            vec![connections[pair.0].result, connections[pair.1].result]
                        })
                        .collect::<Vec<_>>();
                    connections_to_swap.sort();
                    return connections_to_swap.join(",");
                } else if maybe_pairs.is_empty() {
                    unreachable!("Removed everything");
                }
            }
        }

        unreachable!("Not found, remaining {:?}", maybe_pairs);
    }

    fn z_output_internal(
        &self,
        mut values: HashMap<&'a str, u32>,
        connections: &Vec<Connection<'a>>,
    ) -> usize {
        let mut connections_to_try = vec![];

        for connection in connections {
            let (Some(left), Some(right)) =
                (values.get(connection.left), values.get(connection.right))
            else {
                connections_to_try.push(connection);
                continue;
            };

            values.insert(connection.result, connection.gate.apply(*left, *right));
        }

        let mut a_change_happened = true;

        while !connections_to_try.is_empty() && a_change_happened {
            a_change_happened = false;

            let connections = connections_to_try;
            connections_to_try = vec![];

            for connection in connections {
                let (Some(left), Some(right)) =
                    (values.get(connection.left), values.get(connection.right))
                else {
                    connections_to_try.push(connection);
                    continue;
                };

                values.insert(connection.result, connection.gate.apply(*left, *right));
                a_change_happened = true;
            }
        }

        calculate_value_of_wires(&values, "z")
    }
}

fn calculate_value_of_wires(values: &HashMap<&str, u32>, starts_with: &str) -> usize {
    let mut value = values
        .into_iter()
        .filter(|(k, _)| k.starts_with(starts_with))
        .collect::<Vec<_>>();
    value.sort_by_key(|v| v.0);

    value
        .into_iter()
        .rev()
        .fold(0usize, |output, (_, v)| (output << 1) + *v as usize)
}

impl Gate {
    fn apply(&self, left: u32, right: u32) -> u32 {
        match self {
            Gate::And => left & right,
            Gate::Or => left | right,
            Gate::Xor => left ^ right,
        }
    }
}

impl SimulatedOperation {
    fn apply(&self, left: usize, right: usize) -> usize {
        match self {
            SimulatedOperation::Sum => left + right,
            SimulatedOperation::And => left & right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const EXAMPLE2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    const EXAMPLE3: &str = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 4);
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(solution_part1(EXAMPLE2), 2024);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 49520947122770);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            solution_part2(EXAMPLE3, 2, SimulatedOperation::And),
            "z00,z01,z02,z05"
        );
    }

    #[ignore]
    #[test]
    fn test_part2() {
        // solution is gjc,gvm,qjj,qsb,wmp,z17,z26,z39
        // manually found by manually looking at the graph, didn't bother implementing it
    }

    #[test]
    fn test_pairs() {
        let mut pair = Pair::new();
        let len = 4;
        assert_eq!(pair, Pair(0, 1));
        assert_eq!(pair.next(len), true);
        assert_eq!(pair, Pair(0, 2));
        assert_eq!(pair.next(len), true);
        assert_eq!(pair, Pair(0, 3));
        assert_eq!(pair.next(len), true);
        assert_eq!(pair, Pair(1, 2));
        assert_eq!(pair.next(len), true);
        assert_eq!(pair, Pair(1, 3));
        assert_eq!(pair.next(len), true);
        assert_eq!(pair, Pair(2, 3));
        assert_eq!(pair.next(len), false);
    }
}
