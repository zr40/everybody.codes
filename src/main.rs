#![feature(btree_extract_if)]
#![feature(float_next_up_down)]
#![feature(iter_map_windows)]

use std::env::args;

use event::EVENTS;
use quest::Quest;

mod ec2024;
mod event;
mod puzzle_result;
mod quest;

fn main() {
    match args().nth(1).as_deref() {
        None => EVENTS.last().unwrap().run_all(),
        Some("all") => {
            for year in EVENTS {
                year.run_all();
            }
        }
        Some(year) => {
            let year: u16 = year.parse().unwrap();

            match EVENTS.iter().find(|event| event.year == year) {
                Some(year) => year.run_all(),
                None => println!("unknown event year {year}"),
            }
        }
    }
}
