mod aoc2018;
mod aoc2022;
mod input;

use clap::Parser;
use input::Input;
use std::collections::BTreeMap;
use std::time::Instant;

#[derive(Debug, Copy, Clone, Parser)]
struct Args {
    year: Option<u16>,
    day: Option<u8>,
}

fn main() {
    println!("My Advent of Code");
    println!("-----------------");

    let aoc = [(2018, aoc2018::DAYS), (2022, aoc2022::DAYS)]
        .into_iter()
        .map(|(y, r)| (y, (1..).zip(r).collect::<BTreeMap<_, _>>()))
        .collect::<BTreeMap<_, _>>();
    let Args { year, day } = Args::parse();
    let (year, day) = (year.unwrap_or_default(), day.unwrap_or_default());
    match aoc.get(&year) {
        Some(runs) => match runs.get(&day) {
            Some(run) => {
                println!("year: {year}  day: {day}\n");
                time_run(run, Input::new(year, day).unwrap())
            }
            None => {
                println!("year: {year}");
                for (&day, run) in runs {
                    println!("\nday: {day}");
                    time_run(run, Input::new(year, day).unwrap())
                }
            }
        },
        None => {
            println!("\nCalling with only the year runs all challenges in there.\nAvailable:");
            aoc.iter().for_each(|(y, r)| {
                print!("* {y}\n  ");
                (1..=r.len()).for_each(|d| print!(" {d}"));
                println!();
            })
        }
    }
}

fn time_run(run: &fn(Input), input: Input) {
    let start = Instant::now();
    run(input);
    println!("-- {:?}", start.elapsed())
}
