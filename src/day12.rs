// SPDX-FileCopyrightText: 2025 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use good_lp::*;

// Limitations:
// Assumes shapes are 3x3, at most 255 shapes, and the areas are at most 255x255

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

#[aoc(day12, part1)]
pub fn solve_part1(input: &InputType) -> SolutionType {
    for shape in input.shapes.iter() {
        print_shape(shape);
        println!("Flip:");
        print_shape(&flip_vert(shape));
        println!("Mirror:");
        print_shape(&mirror(shape));
        println!("Turn 90:");
        print_shape(&turn90(shape));
        println!("Turn 270:");
        print_shape(&turn270(shape));
        println!("Turn 180:");
        print_shape(&turn90(&turn90(shape)));
        println!();
    }
    let max_size = input
        .goals
        .iter()
        .map(|(w, h, _)| 4 * (*w as isize - 3) * (*h as isize - 3) * input.shapes.len() as isize)
        .max()
        .unwrap();
    println!("Max size: {}", max_size);
    input.goals.iter().map(|(w, h, _goal)| {
        let n_rows = 4 * (w - 3) * (h - 3) * (input.shapes.len() as u8);
        println!("n_rows: {}", n_rows);
        /*
        let mut problem = ProblemVariables::new();
        let x = problem.add_vector(
            variable().min(0i32).integer().max(1i32),
            n_rows,
        );
        let objective: Expression = x.iter().sum();
        */

        0
    }).sum()
}

/*
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
*/

#[aoc(day12, part2)]
pub fn solve_part2(input: &InputType) -> SolutionType {
    println!("{:?}", input.goals.len());
    0
}
