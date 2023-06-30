mod aoc2018;
mod aoc2022;
mod input;

use clap::Parser;
use input::Input;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone, Parser)]
struct Args {
    /// An Advent of Code event.
    year: Option<u16>,
    /// A challenge within an event.
    day: Option<u8>,
    #[arg(short, long, default_value_t = false)]
    /// Do not print elapsed times.
    clean: bool,
}

fn main() {
    println!("My Advent of Code");
    println!("-----------------");

    let Args { year, day, clean } = Args::parse();
    let (year, day) = (year.unwrap_or_default(), day.unwrap_or_default());

    let aoc = [(2018, aoc2018::DAYS), (2022, aoc2022::DAYS)]
        .into_iter()
        .map(|(y, r)| (y, (1..).zip(r).collect::<BTreeMap<_, _>>()))
        .collect::<BTreeMap<_, _>>();
    match aoc.get(&year) {
        Some(runs) => match runs.get(&day) {
            Some(run) => {
                println!("year: {year}  day: {day}\n");
                runner(&|| run(Input::new(year, day).unwrap()), clean);
            }
            None => {
                println!("year: {year}");
                let mut total = Duration::from_secs(0);
                for (&day, run) in runs {
                    println!("\nday: {day}");
                    total += runner(&|| run(Input::new(year, day).unwrap()), clean);
                }
                println!("\ntotal -- {total:?}");
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

fn runner(run: &dyn Fn(), clean: bool) -> Duration {
    let start = Instant::now();
    run();
    let end = start.elapsed();
    if !clean {
        println!("-- {end:?}");
    }
    end
}
