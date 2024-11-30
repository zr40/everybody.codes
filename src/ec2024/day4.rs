use crate::Quest;
use crate::puzzle_result::Answer;

#[cfg(test)]
const EXAMPLE_A: &str = include_str!("input/4a_example");
const INPUT_A: &str = include_str!("input/4a");

const INPUT_B: &str = include_str!("input/4b");

#[cfg(test)]
const EXAMPLE_C: &str = include_str!("input/4c_example");
const INPUT_C: &str = include_str!("input/4c");

pub(super) static DAY: Quest = Quest {
    name: "Royal Smith's Puzzle",
    a: solve_a,
    b: solve_b,
    c: solve_c,
};

fn solve_a_for(input: &str) -> u32 {
    let lengths: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

    let min = *lengths.iter().min().unwrap();
    lengths.iter().map(|l| l - min).sum()
}

fn solve_c_for(input: &str) -> i32 {
    let lengths: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();

    lengths
        .iter()
        .map(|target| lengths.iter().map(|l| (l - target).abs()).sum())
        .min()
        .unwrap()
}

fn solve_a() -> Answer {
    solve_a_for(INPUT_A).into()
}

fn solve_b() -> Answer {
    solve_a_for(INPUT_B).into()
}

fn solve_c() -> Answer {
    solve_c_for(INPUT_C).into()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE_A), 10);
}
#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT_A), 75);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_a_for(INPUT_B), 929471);
}

#[test]
fn c_example() {
    assert_eq!(solve_c_for(EXAMPLE_C), 8);
}
#[test]
fn c_puzzle() {
    assert_eq!(solve_c_for(INPUT_C), 119737595);
}
