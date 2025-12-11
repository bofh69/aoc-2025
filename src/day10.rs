// SPDX-FileCopyrightText: 2025 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;
// use std::collections::HashSet;

// use advent_of_tools::*;

type SolutionType = usize;

#[derive(Debug, Clone, Default)]
pub struct InputEntry {
    wanted_lights: u32,
    buttons: Vec<u32>,
    joltage_ratings: Vec<u8>,
}

type InputType = Vec<InputEntry>;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> InputType {
    input
        .lines()
        .map(|s| {
            let re =
                Regex::new(r"^\[([#\.]+)\] ([\(0-9,\) ]+)+ \{([0-9,]+)\}$").expect("correct regex");

            let m = re.captures(s).expect("line should match regex");
            // println!("Lights {:?}, buttons {:?} joltage {:?}", &m[1], &m[2], &m[3]);
            let wanted_lights = m[1]
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(i, _)| i as u32)
                .fold(0u32, |acc, bit| acc | (1 << bit));

            let mut buttons = Vec::new();

            for s in m[2].split(")").filter(|s| !s.is_empty()) {
                let button = s
                    .trim_matches(|c: char| c == '(' || c == ')' || c.is_whitespace())
                    .split(',')
                    .map(|s| s.parse::<u32>().expect("number"))
                    .fold(0u32, |acc, bit| acc | (1 << bit));
                buttons.push(button);
            }

            let joltage_ratings = m[3]
                .split(',')
                .map(|s| s.trim().parse::<u8>().expect("number"))
                .collect();

            InputEntry {
                wanted_lights,
                buttons,
                joltage_ratings,
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    presses: usize,
    current_lights: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.presses.cmp(&self.presses)
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.presses.cmp(&self.presses))
    }
}

fn how_many_buttons(wanted: u32, buttons: &Vec<u32>) -> usize {
    let mut frontier = std::collections::BinaryHeap::new();
    frontier.push(State {
        presses: 0,
        current_lights: 0,
    });
    while let Some(State {
        presses,
        current_lights,
    }) = frontier.pop()
    {
        if current_lights == wanted {
            return presses;
        }

        // println!("At presses {}, lights {:b}", presses, current_lights);

        for button in buttons {
            frontier.push(State {
                presses: presses + 1,
                current_lights: current_lights ^ button,
            });
        }
    }
    usize::MAX / 1024
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &InputType) -> SolutionType {
    input
        .iter()
        .map(|entry| how_many_buttons(entry.wanted_lights, &entry.buttons))
        .sum()
}

fn how_many_buttons2(joltage_ratings: &[u8], buttons: &[u32]) -> usize {
    let height = buttons.len();
    let width = joltage_ratings.len();
    let mut m = vec![0u8; width * height];
    for (i, button) in buttons.iter().enumerate() {
        for wire in 0..height {
            if (button & (1 << wire)) != 0 {
                m[wire + i * width] = 1;
            }
        }
    }
    println!("Matrix:");
    for i in 0..height {
        for j in 0..width {
            print!("{} ", m[i * width + j]);
        }
        println!();
    }
    0
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &InputType) -> SolutionType {
    input
        .iter()
        .map(|entry| how_many_buttons2(&entry.joltage_ratings, &entry.buttons))
        .sum()
}
