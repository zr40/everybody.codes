use std::collections::{BTreeSet, VecDeque};

use itertools::Itertools;

use crate::Quest;
use crate::puzzle_result::Answer;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/3_example");
const INPUT_A: &str = include_str!("input/3a");

const INPUT_B: &str = include_str!("input/3b");

const INPUT_C: &str = include_str!("input/3c");

pub(super) static DAY: Quest = Quest {
    name: "Mining Maestro",
    a: solve_a,
    b: solve_b,
    c: solve_c,
};

enum Mode {
    Normal,
    RoyalLands,
}

#[derive(Default)]
struct VisitQueue {
    visited: BTreeSet<(usize, usize)>,
    queue: VecDeque<(usize, usize, usize)>,
}

impl VisitQueue {
    fn push_back(&mut self, x: usize, y: usize, depth: usize) {
        if self.visited.insert((x, y)) {
            self.queue.push_back((x, y, depth));
        }
    }

    fn pop_front(&mut self) -> Option<(usize, usize, usize)> {
        self.queue.pop_front()
    }
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let mut map = input
        .lines()
        .map(|line| line.chars().map(|ch| ch == '#').collect_vec())
        .collect_vec();

    if let Mode::RoyalLands = mode {
        for line in &mut map {
            line.insert(0, false);
            line.push(false);
        }
        let len = map[0].len();
        map.insert(0, vec![false; len]);
        map.push(vec![false; len]);
    }

    let mut sum = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let mut queue = VisitQueue::default();
            queue.push_back(x, y, 0);

            while let Some((px, py, depth)) = queue.pop_front() {
                if map[py][px] {
                    if px > 0 {
                        queue.push_back(px - 1, py, depth + 1);
                    }
                    if px < map[0].len() - 1 {
                        queue.push_back(px + 1, py, depth + 1);
                    }
                    if py > 0 {
                        queue.push_back(px, py - 1, depth + 1);
                    }
                    if py < map.len() - 1 {
                        queue.push_back(px, py + 1, depth + 1);
                    }

                    if let Mode::RoyalLands = mode {
                        if px > 0 {
                            if py > 0 {
                                queue.push_back(px - 1, py - 1, depth + 1);
                            }
                            if py < map.len() - 1 {
                                queue.push_back(px - 1, py + 1, depth + 1);
                            }
                        }
                        if px < map[0].len() - 1 {
                            if py > 0 {
                                queue.push_back(px + 1, py - 1, depth + 1);
                            }
                            if py < map.len() - 1 {
                                queue.push_back(px + 1, py + 1, depth + 1);
                            }
                        }
                    }
                } else {
                    sum += depth;
                    break;
                }
            }
        }
    }

    sum
}

fn solve_a() -> Answer {
    solve_for(INPUT_A, Mode::Normal).into()
}

fn solve_b() -> Answer {
    solve_for(INPUT_B, Mode::Normal).into()
}

fn solve_c() -> Answer {
    solve_for(INPUT_C, Mode::RoyalLands).into()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Normal), 35);
}
#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT_A, Mode::Normal), 131);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT_B, Mode::Normal), 2660);
}

#[test]
fn c_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::RoyalLands), 29);
}
#[test]
fn c_puzzle() {
    assert_eq!(solve_for(INPUT_C, Mode::RoyalLands), 10138);
}
