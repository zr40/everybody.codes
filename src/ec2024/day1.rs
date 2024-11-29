use itertools::Itertools;

use crate::{Quest, puzzle_result::Answer};

#[cfg(test)]
const EXAMPLE_A: &str = "ABBAC";
const INPUT_A: &str = include_str!("input/1a");

#[cfg(test)]
const EXAMPLE_B: &str = "AxBCDDCAxD";
const INPUT_B: &str = include_str!("input/1b");

#[cfg(test)]
const EXAMPLE_C: &str = "xBxAAABCDxCC";
const INPUT_C: &str = include_str!("input/1c");

pub(super) static DAY: Quest = Quest {
    a: solve_a,
    b: solve_b,
    c: solve_c,
};

fn potions(creature: char) -> u16 {
    match creature {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        'x' => 0,
        _ => panic!("unknown creature '{creature}'"),
    }
}

fn solve_a_for(input: &str) -> u16 {
    input.chars().map(potions).sum()
}

fn solve_b_for(input: &str) -> u16 {
    input
        .chars()
        .tuples()
        .map(|(a, b)| {
            if a == 'x' {
                potions(b)
            } else if b == 'x' {
                potions(a)
            } else {
                potions(a) + potions(b) + 2
            }
        })
        .sum()
}

fn solve_c_for(input: &str) -> u16 {
    input
        .chars()
        .tuples()
        .map(|(a, b, c)| match (a, b, c) {
            ('x', 'x', a) | ('x', a, 'x') | (a, 'x', 'x') => potions(a),
            ('x', a, b) | (a, 'x', b) | (a, b, 'x') => potions(a) + potions(b) + 2,
            (a, b, c) => potions(a) + potions(b) + potions(c) + 6,
        })
        .sum()
}

fn solve_a() -> Answer {
    solve_a_for(INPUT_A).into()
}

fn solve_b() -> Answer {
    solve_b_for(INPUT_B).into()
}

fn solve_c() -> Answer {
    solve_c_for(INPUT_C).into()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE_A), 5);
}
#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT_A), 1349);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE_B), 28);
}
#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT_B), 5350);
}

#[test]
fn c_example() {
    assert_eq!(solve_c_for(EXAMPLE_C), 30);
}
#[test]
fn c_puzzle() {
    assert_eq!(solve_c_for(INPUT_C), 27958);
}