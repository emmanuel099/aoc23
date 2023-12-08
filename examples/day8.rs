use anyhow::Result;
use aoc23::read_lines;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

type NodeId = String;

fn parse_network(lines: &[String]) -> HashMap<NodeId, (NodeId, NodeId)> {
    let re = Regex::new(r"([1-9A-Z]{3}) = \(([1-9A-Z]{3}), ([1-9A-Z]{3})\)").unwrap();

    lines
        .into_iter()
        .filter_map(|s| {
            let (_, [node, left, right]) = re.captures(s).map(|c| c.extract())?;
            Some((node.to_owned(), (left.to_owned(), right.to_owned())))
        })
        .collect()
}

fn walk_until_end(
    network: &HashMap<NodeId, (NodeId, NodeId)>,
    instructions: &[char],
    start_node: NodeId,
    end_node: NodeId,
) -> Vec<NodeId> {
    let (_, path) = instructions
        .into_iter()
        .cycle()
        .fold_while(
            (start_node.clone(), vec![start_node]),
            |(node, path), inst| {
                if node == end_node {
                    Done((node, path))
                } else {
                    let (left, right) = network.get(&node).expect("node not found");
                    let next_node = match inst {
                        'L' => left,
                        'R' => right,
                        _ => panic!("unknown instruction"),
                    };
                    let mut path = path;
                    path.push(next_node.to_owned());
                    Continue((next_node.to_owned(), path))
                }
            },
        )
        .into_inner();
    path
}

fn ghost_walk_until_end(
    network: &HashMap<NodeId, (NodeId, NodeId)>,
    instructions: &[char],
) -> usize {
    let start_nodes = network
        .keys()
        .into_iter()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .collect_vec();

    let (_, steps) = instructions
        .into_iter()
        .cycle()
        .fold_while((start_nodes, 0), |(nodes, steps), inst| {
            if nodes.iter().all(|node| node.ends_with('Z')) {
                Done((nodes, steps))
            } else {
                let next_nodes = nodes
                    .into_iter()
                    .map(|node| {
                        let (left, right) = network.get(&node).expect("node not found");
                        match inst {
                            'L' => left,
                            'R' => right,
                            _ => panic!("unknown instruction"),
                        }
                    })
                    .cloned()
                    .collect_vec();
                Continue((next_nodes, steps + 1))
            }
        })
        .into_inner();
    steps
}

/// Debugging output of ghost_walk_until_end revealed that the paths are repeating for all start node.
/// Therefore, the total number of steps in simply the least common of the individual path lengths.
fn ghost_walk_until_end_lcm(
    network: &HashMap<NodeId, (NodeId, NodeId)>,
    instructions: &[char],
) -> usize {
    let start_nodes = network
        .keys()
        .into_iter()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .collect_vec();

    let steps_per_start_node = start_nodes
        .into_iter()
        .map(|start_node| {
            let (_, path) = instructions
                .into_iter()
                .cycle()
                .fold_while((start_node, 0usize), |(node, steps), inst| {
                    if node.ends_with('Z') {
                        Done((node, steps))
                    } else {
                        let (left, right) = network.get(&node).expect("node not found");
                        let next_node = match inst {
                            'L' => left,
                            'R' => right,
                            _ => panic!("unknown instruction"),
                        };
                        Continue((next_node.to_owned(), steps + 1))
                    }
                })
                .into_inner();
            path
        })
        .collect_vec();

    steps_per_start_node
        .into_iter()
        .reduce(|x, y| num::integer::lcm(x, y))
        .unwrap()
}

fn main() -> Result<()> {
    let lines = read_lines("input/day8.txt")?;

    let instructions = lines[0].chars().collect_vec();
    let network = parse_network(&lines[1..]);

    let path = walk_until_end(&network, &instructions, "AAA".to_owned(), "ZZZ".to_owned());
    println!("Part I: {}", path.len() - 1);

    println!(
        "Part II: {}",
        ghost_walk_until_end_lcm(&network, &instructions)
    );

    Ok(())
}
