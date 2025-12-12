// SPDX-FileCopyrightText: 2025 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// Limitations:
// Assumes shapes are 3x3, at most 255 shapes, and the areas are at most 255x255
// The final solution also handles the trival cases where the shapes total area is larger
// than the goal area, or where the shapes can easily fit in the goal area.
// That's enough for my input.

type SolutionType = i64;

#[derive(Debug, Clone, Default)]
pub struct InputType {
    shapes: Vec<[u8; 3]>,
    goals: Vec<(u8, u8, Vec<u8>)>,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> InputType {
    let mut shapes = Vec::new();
    let mut lines = input.lines().peekable();
    while !lines.peek().expect("line content").contains('x') {
        let line = lines.next().expect("Shape header line");
        let num = line
            .split(':')
            .next()
            .expect("Number before colon")
            .parse::<u8>()
            .expect("Valid number");
        if num as usize != shapes.len() {
            panic!("Shapes must be in order");
        }
        // Assume shapes are 3 x 3
        let mut shape = [0u8; 3];
        for shape in shape.iter_mut() {
            let shape_line = lines.next().expect("Shape line");
            *shape = shape_line
                .chars()
                .enumerate()
                .fold(0u8, |acc, (j, c)| match c {
                    '#' => acc | (1 << j),
                    '.' => acc,
                    _ => panic!("Invalid character in shape"),
                });
        }
        shapes.push(shape);
        if !lines.next().expect("line after shape").trim().is_empty() {
            panic!("Expected empty line after shape");
        }
    }
    let goals = lines
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();
            let size = parts[0]
                .split('x')
                .map(|s| {
                    s.trim_matches(|c| c == ':')
                        .parse::<u8>()
                        .expect("Valid size")
                })
                .collect::<Vec<_>>();
            let goal = parts
                .iter()
                .skip(1)
                .map(|s| s.parse::<u8>().expect("Valid number"))
                .collect::<Vec<_>>();
            (size[0], size[1], goal)
        })
        .collect();
    InputType { shapes, goals }
}

/*
fn flip_vert(shape: &[u8; 3]) -> [u8; 3] {
    [shape[2], shape[1], shape[0]]
}

fn mirror(shape: &[u8; 3]) -> [u8; 3] {
    [
        ((shape[0] & 0b100) >> 2) | ((shape[0] & 0b010) >> 0) | ((shape[0] & 0b001) << 2),
        ((shape[1] & 0b100) >> 2) | ((shape[1] & 0b010) >> 0) | ((shape[1] & 0b001) << 2),
        ((shape[2] & 0b100) >> 2) | ((shape[2] & 0b010) >> 0) | ((shape[2] & 0b001) << 2),
    ]
}

fn turn90(shape: &[u8; 3]) -> [u8; 3] {
    [
        ((shape[0] & 0b100) >> 2) | ((shape[1] & 0b100) >> 1) | ((shape[2] & 0b100) << 0),
        ((shape[0] & 0b010) >> 1) | ((shape[1] & 0b010) >> 0) | ((shape[2] & 0b010) << 1),
        ((shape[0] & 0b001) >> 0) | ((shape[1] & 0b001) << 1) | ((shape[2] & 0b001) << 2),
    ]
}

fn turn270(shape: &[u8; 3]) -> [u8; 3] {
    [
        ((shape[0] & 0b001) << 2) | ((shape[1] & 0b001) << 1) | ((shape[2] & 0b001) << 0),
        ((shape[0] & 0b010) << 1) | ((shape[1] & 0b010) >> 0) | ((shape[2] & 0b010) >> 1),
        ((shape[0] & 0b100) << 0) | ((shape[1] & 0b100) >> 1) | ((shape[2] & 0b100) >> 2),
    ]
}

fn print_shape(shape: &[u8; 3]) {
    for row in shape.iter() {
        for j in (0..3).rev() {
            if (row & (1 << j)) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
*/

fn count_bits(shape: &[u8; 3]) -> usize {
    shape.iter().map(|row| row.count_ones() as usize).sum()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &InputType) -> SolutionType {
    let max_size = input
        .goals
        .iter()
        .map(|(w, h, _)| 6 * (*w as isize - 2) * (*h as isize - 2) * input.shapes.len() as isize)
        .max()
        .unwrap();
    println!("Max size: {}", max_size);
    input
        .goals
        .iter()
        .map(|(w, h, goal)| {
            let n_full = goal
                .iter()
                .enumerate()
                .map(|(i, &x)| x as usize * count_bits(&input.shapes[i]))
                .sum::<usize>();
            if n_full > *w as usize * *h as usize {
                // println!("Impossible goal");
                return 0;
            }
            if (*w as isize / 3) * (*h as isize / 3) >= goal.iter().map(|n| *n as isize).sum() {
                // println!("Easily fits");
                return 1;
            }
            // println!("n_full: {}/{}", n_full, (*w as usize) * (*h as usize));
            panic!("Non-trivial case not implemented yet");
        })
        .sum()
}
