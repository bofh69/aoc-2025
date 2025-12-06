// SPDX-FileCopyrightText: 2025 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::aoc;

use advent_of_tools::*;

type SolutionType = usize;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> SolutionType {
    let input = input.lines();
    let input: Vec<Vec<&str>> = input.map(|l| l.split_whitespace().collect()).collect();
    let rows: Vec<Vec<SolutionType>> = input[0..input.len() - 1]
        .iter()
        .map(|row| row.iter().map(|s| s.parse().unwrap()).collect())
        .collect();
    let ops: Vec<char> = input[input.len() - 1]
        .iter()
        .map(|column| column.chars().next().unwrap())
        .collect();
    let mut sum = 0;
    for (column, op) in ops.iter().enumerate() {
        sum += match op {
            '+' => rows.iter().map(|row| row[column]).sum(),
            '*' => rows.iter().map(|row| row[column]).product(),
            _ => 0,
        };
    }
    sum
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> SolutionType {
    let input = Map::<i16>::from_string(input);
    let mut columns = Vec::new();
    let mut ops = Vec::new();
    for column in 0..input.get_width() {
        let mut count = 0;
        for row in 0..input.get_height() - 1 {
            let c = input.get_at_unchecked(Point { x: column, y: row });
            if c == b' ' {
                // Ignore
            } else if c.is_ascii_digit() {
                count = count * 10 + (c - b'0') as SolutionType;
            } else {
                panic!("Unknown char: {c}");
            }
        }
        columns.push(count);
        ops.push(input.get_at_unchecked(Point {
            x: column,
            y: input.get_height() - 1,
        }));
    }

    let mut sum = 0;
    let mut current_op = b' ';
    let mut partial = 0;
    for (idx, value) in columns.iter().enumerate() {
        if idx + 1 < columns.len() && ops[idx + 1] != b' ' {
            // Skip column before op
            continue;
        }
        if ops[idx] != b' ' {
            current_op = ops[idx];
            // println!("Adding {partial} as column {idx}");
            sum += partial;
            if current_op == b'+' {
                partial = 0;
            } else {
                partial = 1;
            }
        }
        if current_op == b'+' {
            partial += value;
        } else if current_op == b'*' {
            partial *= value;
        } else {
            panic!("Unknown op {current_op}");
        }
    }
    sum += partial;
    sum
}
