use std::collections::BTreeSet;

use crate::Quest;
use crate::puzzle_result::Answer;

#[cfg(test)]
const EXAMPLE_A1: &str = include_str!("input/2a_example1");
#[cfg(test)]
const EXAMPLE_A2: &str = include_str!("input/2a_example2");
#[cfg(test)]
const EXAMPLE_A3: &str = include_str!("input/2a_example3");
#[cfg(test)]
const EXAMPLE_A4: &str = include_str!("input/2a_example4");
const INPUT_A: &str = include_str!("input/2a");

#[cfg(test)]
const EXAMPLE_B: &str = include_str!("input/2b_example");
const INPUT_B: &str = include_str!("input/2b");

#[cfg(test)]
const EXAMPLE_C: &str = include_str!("input/2c_example");
const INPUT_C: &str = include_str!("input/2c");

pub(super) static DAY: Quest = Quest {
    name: "The Runes of Power",
    a: solve_a,
    b: solve_b,
    c: solve_c,
};

fn solve_a_for(input: &str) -> usize {
    let mut lines = input.lines();
    let words: Vec<&str> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split(',')
        .collect();
    lines.next().unwrap();
    let sentence = lines.next().unwrap();

    (0..sentence.len())
        .filter(|i| words.iter().any(|word| sentence[*i..].starts_with(word)))
        .count()
}

fn solve_b_for(input: &str) -> usize {
    let mut lines = input.lines();
    let mut words: BTreeSet<String> = BTreeSet::new();

    for word in lines.next().unwrap().split_once(':').unwrap().1.split(',') {
        words.insert(word.to_owned());
        words.insert(word.chars().rev().collect());
    }

    let mut count = 0;
    for sentence in lines {
        let mut symbols = BTreeSet::new();
        for i in 0..sentence.len() {
            for word in &words {
                if sentence[i..].starts_with(word) {
                    for j in i..(i + word.len()) {
                        symbols.insert(j);
                    }
                }
            }
        }
        count += symbols.len();
    }
    count
}

fn solve_c_for(input: &str) -> usize {
    let mut lines = input.lines();
    let mut words: BTreeSet<String> = BTreeSet::new();

    for word in lines.next().unwrap().split_once(':').unwrap().1.split(',') {
        words.insert(word.to_owned());
        words.insert(word.chars().rev().collect());
    }

    lines.next().unwrap();

    let mut grid: Vec<Vec<char>> = vec![];
    for line in lines {
        grid.push(line.chars().collect());
    }

    let mut found = BTreeSet::new();

    for y in 0..grid.len() {
        for x in 0..(grid[0].len()) {
            for word in &words {
                if word
                    .chars()
                    .enumerate()
                    .all(|(i, ch)| grid[y][(x + i) % grid[0].len()] == ch)
                {
                    for x in x..x + word.len() {
                        found.insert((x % grid[0].len(), y));
                    }
                }

                if word
                    .chars()
                    .enumerate()
                    .all(|(i, ch)| y + i < grid.len() && grid[y + i][x] == ch)
                {
                    for y in y..y + word.len() {
                        found.insert((x, y));
                    }
                }
            }
        }
    }

    found.len()
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
    assert_eq!(solve_a_for(EXAMPLE_A1), 4);
    assert_eq!(solve_a_for(EXAMPLE_A2), 3);
    assert_eq!(solve_a_for(EXAMPLE_A3), 2);
    assert_eq!(solve_a_for(EXAMPLE_A4), 3);
}
#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT_A), 35);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE_B), 42);
}
#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT_B), 5243);
}

#[test]
fn c_example() {
    assert_eq!(solve_c_for(EXAMPLE_C), 10);
}
#[test]
fn c_puzzle() {
    assert_eq!(solve_c_for(INPUT_C), 11958);
}
