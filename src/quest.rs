use std::time::{Duration, Instant};

use crate::puzzle_result::Answer;

type Part = fn() -> Answer;

#[derive(Clone, Copy)]
pub(crate) struct Quest {
    pub name: &'static str,
    pub a: Part,
    pub b: Part,
    pub c: Part,
}

pub(crate) struct PartResult {
    pub(crate) answer: Answer,
    pub(crate) duration: Duration,
}

fn run_part(part: Part) -> PartResult {
    let start = Instant::now();
    let answer = part();
    let duration = start.elapsed();

    PartResult { answer, duration }
}

pub(crate) struct QuestResult {
    pub(crate) a: PartResult,
    pub(crate) b: PartResult,
    pub(crate) c: PartResult,
}

impl Quest {
    pub(crate) fn run(&self) -> QuestResult {
        let a = run_part(self.a);
        let b = run_part(self.b);
        let c = run_part(self.c);

        QuestResult { a, b, c }
    }
}
