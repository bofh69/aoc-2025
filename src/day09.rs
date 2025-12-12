// SPDX-FileCopyrightText: 2025 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use advent_of_tools::*;

type SolutionType = i64;

type PositionType = i32;

type InputType = Vec<(PositionType, PositionType)>;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> InputType {
    input
        .lines()
        .map(|s| {
            let mut nums = s.split(',').map(|s| s.parse().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect()
}

fn get_area(a: &(PositionType, PositionType), b: &(PositionType, PositionType)) -> SolutionType {
    let xd: SolutionType = (a.0 - b.0).abs().into();
    let yd: SolutionType = (a.1 - b.1).abs().into();
    (xd + 1) * (yd + 1)
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &InputType) -> SolutionType {
    let mut max_area = SolutionType::MIN;
    for (i, p1) in input.iter().enumerate() {
        for p2 in input.iter().skip(i + 1) {
            let a = get_area(p1, p2);
            // println!("Area between {:?} and {:?} is {}", p1, p2, a);
            if a > max_area {
                max_area = a;
            }
        }
    }
    max_area
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &InputType) -> SolutionType {
    let mut from = input[0];
    let mut vedges = Vec::with_capacity(input.len() / 2);
    let mut hedges = Vec::with_capacity(input.len() / 2);
    for p in input.iter().skip(1) {
        if p.1 != from.1 {
            vedges.push((from, *p));
        } else {
            hedges.push((from, *p));
        }
        from = *p;
    }
    if input[0].1 != from.1 {
        vedges.push((from, input[0]));
    } else {
        hedges.push((from, input[0]));
    }
    // vedges.sort_by_key(|(a, b)| a.1.min(b.1));
    // hedges.sort_by_key(|(a, b)| a.1.min(b.1));

    let mut max_area = SolutionType::MIN;
    for (i, p1) in input.iter().enumerate() {
        'p2: for p2 in input.iter().skip(i + 1) {
            let a = get_area(p1, p2);
            let aabb = (
                p1.0.min(p2.0),
                p1.1.min(p2.1),
                p1.0.max(p2.0),
                p1.1.max(p2.1),
            );
            if a > max_area {
                for edge in &vedges {
                    let edge_max_y = edge.0.1.max(edge.1.1);
                    if aabb.1 >= edge_max_y {
                        // edge is completely above area
                        continue;
                    }
                    let edge_min_y = edge.0.1.min(edge.1.1);
                    if aabb.3 <= edge_min_y {
                        // edge is completely below area
                        continue;
                    }
                    if aabb.0 < edge.0.0 && aabb.2 > edge.0.0 {
                        // edge crosses area
                        continue 'p2;
                    }
                }
                for edge in &hedges {
                    let edge_max_x = edge.0.0.max(edge.1.0);
                    if aabb.0 >= edge_max_x {
                        // edge is completely to the left of area
                        continue;
                    }
                    let edge_min_x = edge.0.0.min(edge.1.0);
                    if aabb.2 <= edge_min_x {
                        // edge is completely to the right of area
                        continue;
                    }
                    if aabb.1 < edge.0.1 && aabb.3 > edge.0.1 {
                        // edge crosses area
                        continue 'p2;
                    }
                }
                max_area = a;
            }
        }
    }
    max_area
}
