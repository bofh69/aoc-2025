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

fn area(a: &(PositionType, PositionType), b: &(PositionType, PositionType)) -> SolutionType {
    let xd: SolutionType = (a.0 - b.0).abs().into();
    let yd: SolutionType = (a.1 - b.1).abs().into();
    (xd + 1) * (yd + 1)
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &InputType) -> SolutionType {
    let mut max_area = SolutionType::MIN;
    for (i, p1) in input.iter().enumerate() {
        for p2 in input.iter().skip(i + 1) {
            let a = area(p1, p2);
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
    let mut edges = Vec::with_capacity(input.len() / 2);
    for p in input.iter().skip(1) {
        if p.1 != from.1 {
            edges.push((from, *p));
        }
        from = *p;
    }
    if input[0].1 != from.1 {
        edges.push((from, input[0]));
    }
    edges.sort_by_key(|(a, b)| a.1.min(b.1));

    // Run shoelacer algorithm
    // see what areas that fit
    for (i, edge) in edges.iter().enumerate() {
        let cy_min = edge.0.1.min(edge.1.1);
        let cy_max = edge.0.1.max(edge.1.1);
        println!("Edge {}: rows {:?} to {:?}", i, cy_min, cy_max);
        let current_edges: Vec<_> = edges
            .iter()
            .filter(|(a, b)| {
                let y_min = a.1.min(b.1);
                let y_max = a.1.max(b.1);
                !(y_max < cy_min || y_min > cy_max)
            })
            .collect();
        println!("  Current edges: {:?}", current_edges);
    }

    edges.len() as SolutionType
}
