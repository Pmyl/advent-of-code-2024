// https://adventofcode.com/2024/day/23

use std::collections::{BTreeSet, HashMap, HashSet};

pub fn solution_part1(input: &str) -> usize {
    let lan_party = LanParty::from_input(input);
    lan_party.count_groups_of_3_with_computer_starting_with_t()
}

pub fn solution_part2(input: &str) -> String {
    let lan_party = LanParty::from_input(input);
    lan_party.password_of_biggest_lan_connection()
}

struct LanParty<'a> {
    partial_graph: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> LanParty<'a> {
    fn from_input(input: &'a str) -> Self {
        let mut connections = input.lines().collect::<Vec<_>>();
        connections.sort();

        let mut graph = HashMap::new();

        let (left, right) = connections[0].split_once('-').unwrap();
        let mut last_computer = left;
        let mut edges = vec![right];

        for connection in connections.into_iter().skip(1) {
            let (left, right) = connection.split_once('-').unwrap();
            if left != last_computer {
                graph.insert(last_computer, edges);
                last_computer = left;
                edges = vec![];
            }

            edges.push(right);
        }

        if !edges.is_empty() {
            graph.insert(last_computer, edges);
        }

        Self {
            partial_graph: graph,
        }
    }

    fn count_groups_of_3_with_computer_starting_with_t(&self) -> usize {
        let mut all_trios = HashSet::new();

        for computer in self.partial_graph.keys() {
            for computer2 in self.partial_graph.get(computer).unwrap() {
                for computer3 in self.partial_graph.get(computer2).unwrap() {
                    let mut trio = [computer, computer2, computer3];
                    trio.sort();
                    if all_trios.contains(&trio) {
                        continue;
                    }

                    if !self
                        .partial_graph
                        .get(computer3)
                        .unwrap()
                        .iter()
                        .any(|c| c == computer)
                        && !self
                            .partial_graph
                            .get(computer)
                            .unwrap()
                            .iter()
                            .any(|c| c == computer3)
                    {
                        continue;
                    }

                    if computer.starts_with('t')
                        || computer2.starts_with('t')
                        || computer3.starts_with('t')
                    {
                        all_trios.insert(trio);
                        continue;
                    }
                }
            }
        }

        all_trios.len()
    }

    fn password_of_biggest_lan_connection(&self) -> String {
        let full_graph = self.full_graph();

        let mut connections_map = HashMap::<String, usize>::new();

        for (computer, connected_computers) in full_graph {
            let mut list = Vec::new();
            list.push(computer.to_string());
            list.push(connected_computers.first().unwrap().to_string());

            for connected_computer in connected_computers.into_iter().skip(1) {
                list.push(connected_computer.to_string());
                list.sort();

                for i in 0..list.len() {
                    let l = list.remove(i);

                    if l != computer {
                        let key = list.join("");
                        *connections_map.entry(key).or_insert(0) += 1;
                    }

                    list.insert(i, l);
                }
            }

            let key = list.join("");
            *connections_map.entry(key).or_insert(0) += 1;
        }

        connections_map
            .into_iter()
            .filter(|(key, values)| key.len() / 2 == *values)
            .max_by_key(|(_, v)| *v)
            .unwrap()
            .0
            .chars()
            .array_chunks::<2>()
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn full_graph(&self) -> HashMap<&'a str, BTreeSet<&'a str>> {
        let mut graph = HashMap::new();

        for (computer, connections) in self.partial_graph.iter() {
            graph
                .entry(*computer)
                .or_insert_with(|| BTreeSet::new())
                .extend(connections);
            for connected_computer in connections {
                graph
                    .entry(&connected_computer)
                    .or_insert_with(|| BTreeSet::new())
                    .insert(*computer);
            }
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 7);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 998);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), "co,de,ka,ta");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solution_part2(INPUT),
            "cc,ff,fh,fr,ny,oa,pl,rg,uj,wd,xn,xs,zw"
        );
    }
}
