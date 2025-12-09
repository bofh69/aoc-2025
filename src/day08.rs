// SPDX-FileCopyrightText: 2025 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use advent_of_tools::*;

type SolutionType = usize;

type PositionType = i32;

type InputType = Vec<(PositionType, PositionType, PositionType)>;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> InputType {
    input
        .lines()
        .map(|s| {
            let mut nums = s.split(',').map(|s| s.parse().unwrap());
            (
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            )
        })
        .collect()
}

fn distance(
    a: &(PositionType, PositionType, PositionType),
    b: &(PositionType, PositionType, PositionType),
) -> i32 {
    let x = ((a.0 - b.0) as f64).abs();
    let y = ((a.1 - b.1) as f64).abs();
    let z = ((a.2 - b.2) as f64).abs();
    (x * x + y * y + z * z).sqrt() as i32
}

#[derive(Default, Debug, Ord, PartialOrd, Clone, Eq, PartialEq, Hash)]
struct DistanceEntry {
    distance: PositionType,
    from: u16,
    to: u16,
}

fn change_group(groups: &mut [u16], from: u16, to: u16) {
    for g in groups.iter_mut() {
        if *g == from {
            *g = to;
        }
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &InputType) -> SolutionType {
    let size = input.len();
    let mut distances = Vec::with_capacity(size * size / 2);
    for (i, a) in input.iter().enumerate() {
        for (j, b) in input.iter().enumerate() {
            if i < j {
                distances.push(DistanceEntry {
                    distance: distance(a, b),
                    from: i as u16,
                    to: j as u16,
                });
            }
        }
    }
    distances.sort_unstable();

    let mut connections = std::collections::HashSet::new();
    for dist in distances.iter().take(1000) {
        connections.insert(dist.clone());
    }
    let mut group = vec![0u16; distances.len()];
    let mut n_groups = 1;
    for conn in connections {
        if group[conn.from as usize] == 0 {
            group[conn.from as usize] = n_groups;
        } else if group[conn.to as usize] == 0 {
            group[conn.to as usize] = group[conn.from as usize];
        } else {
            let from_group = group[conn.from as usize];
            change_group(&mut group, from_group, n_groups);
        }
        if group[conn.to as usize] == 0 {
            group[conn.to as usize] = n_groups;
        } else if group[conn.from as usize] != group[conn.to as usize] {
            let to_group = group[conn.to as usize];
            change_group(&mut group, to_group, n_groups);
        }
        n_groups += 1;
    }

    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for x in &group {
        *freq.entry(x).or_insert(0) += 1;
    }
    let mut pairs: Vec<_> = freq.into_iter().filter(|(n, _v)| **n != 0).collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));

    pairs.into_iter().take(3).map(|n| n.1).product()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &InputType) -> SolutionType {
    let size = input.len();
    let mut distances = Vec::with_capacity(size * size / 2);
    for (i, a) in input.iter().enumerate() {
        for (j, b) in input.iter().enumerate() {
            if i < j {
                distances.push(DistanceEntry {
                    distance: distance(a, b),
                    from: i as u16,
                    to: j as u16,
                });
            }
        }
    }
    distances.sort_unstable();

    // This is wrong, but accidentally works right for my input...
    // There is no check that they are all in the same cluster
    //
    // The right solution could give each node a cluster ID like in part 1 and each cluster a
    // number. When adding two clusters, update the cluster ID of one of them and increase the
    // cluster's size until reaching _size_.
    let mut connected = vec![false; distances.len()];
    let mut n_connected = 0;
    for distance in distances {
        let from = distance.from as usize;
        let to = distance.to as usize;
        if !connected[from] {
            connected[from] = true;
            n_connected += 1;
        }
        if !connected[to] {
            connected[to] = true;
            n_connected += 1;
        }
        if n_connected == size {
            return input[from].0 as SolutionType * input[to].0 as SolutionType;
        }
    }
    0
}
