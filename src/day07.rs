// SPDX-FileCopyrightText: 2025 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use advent_of_tools::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type SolutionType = usize;

type InputType = Map;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> InputType {
    Map::from_string(input)
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &InputType) -> SolutionType {
    let start = input.find(b'S');
    if start.len() != 1 {
        panic!("No start!");
    }
    let mut y = start[0].y + 1;
    let mut tachyons = HashSet::new();
    tachyons.insert(start[0].x);
    let mut splits = 0;
    while y < input.get_height() {
        let mut new_tachyons = HashSet::new();
        for ray in tachyons {
            let c = input.get_at_unchecked(Point { x: ray, y });
            if c == b'.' {
                new_tachyons.insert(ray);
            } else if c == b'^' {
                new_tachyons.insert(ray - 1);
                new_tachyons.insert(ray + 1);
                splits += 1;
            }
        }
        y += 1;
        tachyons = new_tachyons
    }
    splits
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &InputType) -> SolutionType {
    let start = input.find(b'S');
    if start.len() != 1 {
        panic!("No start!");
    }
    let mut y = start[0].y + 1;
    let mut tachyons = vec![0; input.get_width() as usize];
    tachyons[start[0].x as usize] = 1;
    while y < input.get_height() {
        let mut new_tachyons = vec![0; input.get_width() as usize];
        for (ray, num) in tachyons.iter().enumerate() {
            let c = input.get_at_unchecked(Point { x: ray as i32, y });
            if c == b'.' {
                new_tachyons[ray] += num;
            } else if c == b'^' {
                new_tachyons[ray - 1] += num;
                new_tachyons[ray + 1] += num;
            }
        }
        y += 1;
        tachyons = new_tachyons
    }
    tachyons.iter().sum()
}
