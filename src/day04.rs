// SPDX-FileCopyrightText: 2025 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use advent_of_tools::*;

type SolutionType = usize;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string(input)
}

#[aoc(day4, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    map.find(b'@')
        .iter()
        .filter(|p| map.neighbors(**p).filter(|(_, _, c)| *c == b'@').count() < 4)
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let mut map = map.clone();
    let mut count = 0;
    while map.transform(|map, p, c| {
        if c == b'@' {
            if map.neighbors(p).filter(|(_, _, c)| *c == b'@').count() < 4 {
                count += 1;
                b'x'
            } else {
                c
            }
        } else {
            c
        }
    }) {}
    count
}
