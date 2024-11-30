use std::collections::BTreeMap;

use crate::Quest;
use crate::puzzle_result::Answer;

#[cfg(test)]
const EXAMPLE_A: &str = include_str!("input/5a_example");
const INPUT_A: &str = include_str!("input/5a");

#[cfg(test)]
const EXAMPLE_B: &str = include_str!("input/5b_example");
const INPUT_B: &str = include_str!("input/5b");

const INPUT_C: &str = include_str!("input/5c");

pub(super) static DAY: Quest = Quest {
    name: "Pseudo-Random Clap Dance",
    a: solve_a,
    b: solve_b,
    c: solve_c,
};

fn solve_a_for(input: &str) -> usize {
    let mut columns: Vec<Vec<usize>> = vec![vec![], vec![], vec![], vec![]];

    for line in input.lines() {
        let mut number = line.split(' ');
        columns[0].push(number.next().unwrap().parse().unwrap());
        columns[1].push(number.next().unwrap().parse().unwrap());
        columns[2].push(number.next().unwrap().parse().unwrap());
        columns[3].push(number.next().unwrap().parse().unwrap());
    }

    for round in 0..10 {
        let column = round % 4;
        let target_column = (round + 1) % 4;

        let clapper = columns[column].remove(0);
        let new_pos = (clapper - 1) % (columns[target_column].len() * 2);
        let new_pos = if new_pos > columns[target_column].len() {
            (columns[target_column].len() * 2) - new_pos
        } else {
            new_pos
        };

        columns[target_column].insert(new_pos, clapper);
    }

    // XXX: would break for multi-digit numbers
    columns[0][0] * 1000 + columns[1][0] * 100 + columns[2][0] * 10 + columns[3][0]
}

fn solve_b_for(input: &str) -> usize {
    let mut columns: Vec<Vec<usize>> = vec![vec![], vec![], vec![], vec![]];

    for line in input.lines() {
        let mut number = line.split(' ');
        columns[0].push(number.next().unwrap().parse().unwrap());
        columns[1].push(number.next().unwrap().parse().unwrap());
        columns[2].push(number.next().unwrap().parse().unwrap());
        columns[3].push(number.next().unwrap().parse().unwrap());
    }

    let mut numbers_shouted = BTreeMap::new();

    let mut round = 0;
    loop {
        let column = round % 4;
        let target_column = (round + 1) % 4;

        let clapper = columns[column].remove(0);
        let new_pos = (clapper - 1) % (columns[target_column].len() * 2);
        let new_pos = if new_pos > columns[target_column].len() {
            (columns[target_column].len() * 2) - new_pos
        } else {
            new_pos
        };

        columns[target_column].insert(new_pos, clapper);

        let number: usize = format!(
            "{}{}{}{}",
            columns[0][0], columns[1][0], columns[2][0], columns[3][0]
        )
        .parse()
        .unwrap();

        round += 1;

        if let Some(amount) = numbers_shouted.get(&number) {
            let amount = amount + 1;

            if amount == 2024 {
                return round * number;
            }

            numbers_shouted.insert(number, amount);
        } else {
            numbers_shouted.insert(number, 1);
        }
    }
}

fn solve_c_for(input: &str) -> usize {
    let mut columns: Vec<Vec<usize>> = vec![vec![], vec![], vec![], vec![]];

    for line in input.lines() {
        let mut number = line.split(' ');
        columns[0].push(number.next().unwrap().parse().unwrap());
        columns[1].push(number.next().unwrap().parse().unwrap());
        columns[2].push(number.next().unwrap().parse().unwrap());
        columns[3].push(number.next().unwrap().parse().unwrap());
    }

    let mut numbers_shouted = BTreeMap::new();

    let mut round = 0;
    loop {
        let column = round % 4;
        let target_column = (round + 1) % 4;

        let clapper = columns[column].remove(0);
        let new_pos = (clapper - 1) % (columns[target_column].len() * 2);
        let new_pos = if new_pos > columns[target_column].len() {
            (columns[target_column].len() * 2) - new_pos
        } else {
            new_pos
        };

        columns[target_column].insert(new_pos, clapper);

        let number: usize = format!(
            "{}{}{}{}",
            columns[0][0], columns[1][0], columns[2][0], columns[3][0]
        )
        .parse()
        .unwrap();

        round += 1;

        if let Some(amount) = numbers_shouted.get(&number) {
            let amount = amount + 1;

            // XXX: not proper loop detection
            if amount == 3 {
                return *numbers_shouted.keys().max().unwrap();
            }

            numbers_shouted.insert(number, amount);
        } else {
            numbers_shouted.insert(number, 1);
        }
    }
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
    assert_eq!(solve_a_for(EXAMPLE_A), 2323);
}
#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT_A), 2223);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE_B), 50877075);
}
#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT_B), 16460102085624);
}

#[test]
fn c_example() {
    assert_eq!(solve_c_for(EXAMPLE_B), 6584);
}
#[test]
fn c_puzzle() {
    assert_eq!(solve_c_for(INPUT_C), 5766100410051003);
}
