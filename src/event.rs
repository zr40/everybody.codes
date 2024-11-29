use std::time::Duration;

use crate::ec2024;
use crate::puzzle_result::Answer;
use crate::quest::Quest;

pub(crate) struct Event {
    pub(crate) year: u16,
    name: &'static str,
    quests: &'static [Quest],
}

pub(crate) static EVENTS: &[Event] = &[Event {
    year: 2024,
    name: "The Kingdom of Algorithmia",
    quests: ec2024::QUESTS,
}];

impl Event {
    pub(crate) fn run_all(&self) {
        println!("{} ({})", self.name, self.year);

        let mut total_duration = Duration::ZERO;

        println!(
            " Quest |     Part One     |     Part Two     |    Part Three    |                Duration                 "
        );
        println!(
            "-------+------------------+------------------+------------------+-------------+-------------+-------------"
        );
        for (index, day) in self.quests.iter().enumerate() {
            let result = day.run();

            println!(
                "    {:>2} | {:>16} | {:>16} | {:>16} | {:>8.2} ms | {:>8.2} ms | {:>8.2} ms",
                index + 1,
                result.a.answer,
                result.b.answer,
                result.c.answer,
                result.a.duration.as_micros() as f64 / 1000.0,
                result.b.duration.as_micros() as f64 / 1000.0,
                result.c.duration.as_micros() as f64 / 1000.0,
            );
            total_duration += result.a.duration;
            total_duration += result.b.duration;
            total_duration += result.c.duration;
            if let Answer::Multiline(v) = result.a.answer {
                println!("{v}");
            }
            if let Answer::Multiline(v) = result.b.answer {
                println!("{v}");
            }
            if let Answer::Multiline(v) = result.c.answer {
                println!("{v}");
            }
        }
        println!(
            "Total run time: {:.1} ms\n",
            total_duration.as_micros() as f64 / 1000.0
        );
    }
}
