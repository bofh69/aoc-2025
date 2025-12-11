// SPDX-FileCopyrightText: 2025 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use advent_of_tools::*;

use std::collections::HashMap;

type SolutionType = usize;

type IndexType = u16;

type InputType = (
    HashMap<String, IndexType>,
    HashMap<IndexType, Vec<IndexType>>,
);

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> InputType {
    let mut node_numbers: HashMap<String, IndexType> = input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            (
                s.split(':').next().expect("nodename:").to_owned(),
                i.try_into().expect("Fits in IndexType"),
            )
        })
        .collect();
    node_numbers.insert("out".to_owned(), IndexType::MAX);
    (
        node_numbers.clone(),
        input
            .lines()
            .enumerate()
            .map(|(i, s)| {
                (
                    i.try_into().expect("Fits in IndexType"),
                    s.split(' ')
                        .skip(1)
                        .map(|s| {
                            *node_numbers
                                .get(s)
                                .unwrap_or_else(|| panic!("{s} should be a known node"))
                        })
                        .collect(),
                )
            })
            .collect(),
    )
}

fn count_paths(
    graph: &HashMap<IndexType, Vec<IndexType>>,
    current_node: IndexType,
    goal_node: IndexType,
    visited: &mut HashMap<IndexType, SolutionType>,
) -> SolutionType {
    if current_node == goal_node {
        return 1;
    }
    if let Some(&cached) = visited.get(&current_node) {
        return cached;
    }
    let mut sum = 0;
    for &next_node in &graph[&current_node] {
        if let Some(cached) = visited.get(&next_node) {
            sum += cached;
            continue;
        }
        let paths = count_paths(graph, next_node, goal_node, visited);
        visited.insert(next_node, paths);
        sum += paths;
    }
    sum
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &InputType) -> SolutionType {
    count_paths(
        &input.1,
        *input.0.get("you").expect("you to exist"),
        *input.0.get("out").expect("out to exist"),
        &mut HashMap::new(),
    )
}

fn count_paths2(
    graph: &HashMap<IndexType, Vec<IndexType>>,
    current_node: IndexType,
    has_visited: (bool, bool),
    goal_node: IndexType,
    must_visit1_node: IndexType,
    must_visit2_node: IndexType,
    visited: &mut HashMap<(IndexType, (bool, bool)), SolutionType>,
) -> SolutionType {
    if current_node == goal_node {
        if has_visited.0 && has_visited.1 {
            return 1;
        } else {
            return 0;
        }
    }
    if let Some(&cached) = visited.get(&(current_node, has_visited)) {
        return cached;
    }
    let mut has_visited = has_visited;
    if current_node == must_visit1_node {
        has_visited.0 = true;
    }
    if current_node == must_visit2_node {
        has_visited.1 = true;
    }
    let mut sum = 0;
    for &next_node in &graph[&current_node] {
        if let Some(cached) = visited.get(&(next_node, has_visited)) {
            sum += cached;
            continue;
        }
        let paths = count_paths2(
            graph,
            next_node,
            has_visited,
            goal_node,
            must_visit1_node,
            must_visit2_node,
            visited,
        );
        visited.insert((next_node, has_visited), paths);
        sum += paths;
    }
    sum
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &InputType) -> SolutionType {
    count_paths2(
        &input.1,
        *input.0.get("svr").expect("svr to exist"),
        (false, false),
        *input.0.get("out").expect("fft to exist"),
        *input.0.get("fft").expect("fft to exist"),
        *input.0.get("dac").expect("dac to exist"),
        &mut HashMap::new(),
    )
}
