// SPDX-FileCopyrightText: 2025 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;
// use std::collections::HashSet;

use good_lp::*;

// use advent_of_tools::*;

type SolutionType = i64;

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
    presses: SolutionType,
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

fn how_many_buttons(wanted: u32, buttons: &Vec<u32>) -> SolutionType {
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
    SolutionType::MAX / 1024
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &InputType) -> SolutionType {
    input
        .iter()
        .map(|entry| how_many_buttons(entry.wanted_lights, &entry.buttons))
        .sum()
}

fn how_many_buttons2(joltage_ratings: &[u8], buttons: &[u32]) -> SolutionType {
    let max_joltage = *joltage_ratings.iter().max().unwrap();
    let height = buttons.len();

    let mut problem = ProblemVariables::new();
    let x = problem.add_vector(
        variable().min(0i32).integer().max(max_joltage as i32),
        height,
    );
    let objective: Expression = x.iter().sum();
    let mut model = problem.minimise(objective).using(default_solver);

    for (i, _) in joltage_ratings.iter().enumerate() {
        let mut expr = Expression::from(0);
        for (j, button) in buttons.iter().enumerate() {
            if (button & (1 << i)) != 0 {
                expr += x[j];
            }
        }
        model = model.with(expr.eq(joltage_ratings[i] as i32));
    }

    let solution = model.solve().expect("solvable LP");
    (0..height)
        .map(|button| solution.value(x[button]).round() as SolutionType)
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &InputType) -> SolutionType {
    input
        .iter()
        .map(|entry| how_many_buttons2(&entry.joltage_ratings, &entry.buttons))
        .sum()
}
