// SPDX-FileCopyrightText: 2025 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use advent_of_tools::*;

type SolutionType = usize;

type InputType = (Vec<(SolutionType, SolutionType)>, Vec<SolutionType>);

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> InputType {
    let mut input = input.lines();
    let mut result1 = Vec::new();
    while let Some(range) = input.next()
        && !range.is_empty()
    {
        let mut range = range.split('-');
        let from = range.next().unwrap().parse::<SolutionType>().unwrap();
        let to = range.next().unwrap().parse::<SolutionType>().unwrap();
        result1.push((from, to));
    }
    let result2 = input.map(|s| s.parse().unwrap()).collect();
    (result1, result2)
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &InputType) -> SolutionType {
    input
        .1
        .iter()
        .filter(|&&n| input.0.iter().any(|&(from, to)| n >= from && n <= to))
        .count()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &InputType) -> SolutionType {
    let mut ranges = input
        .0
        .iter()
        .map(|&(from, to)| (from, to, false))
        .collect::<Vec<_>>();
    loop {
        let mut new_ranges: Vec<(SolutionType, SolutionType, bool)> = Vec::new();
        let mut updates = false;
        'range: for i in 0..ranges.len() {
            if ranges[i].2 {
                continue;
            }
            let from1 = ranges[i].0;
            let to1 = ranges[i].1;
            for j in i + 1..ranges.len() {
                if ranges[j].2 {
                    continue;
                }
                let from2 = ranges[j].0;
                let to2 = ranges[j].1;
                if !(to1 < from2 || from1 > to2) {
                    let new_from = from1.min(from2);
                    let new_to = to1.max(to2);
                    new_ranges.push((new_from, new_to, false));
                    ranges[i].2 = true;
                    ranges[j].2 = true;
                    updates = true;
                    continue 'range;
                }
            }
            new_ranges.push((from1, to1, false));
        }
        ranges = new_ranges;
        if !updates {
            break;
        }
    }
    ranges
        .iter()
        .map(|&(from, to, _)| to - from + 1)
        .sum::<SolutionType>()
}
