mod aoc2018;
mod aoc2022;
mod input;

use clap::Parser;
use input::Input;
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, Parser)]
struct Args {
    year: u16,
    day: u8,
}

fn main() {
    println!("My Advent of Code");
    println!("-----------------");
    let Args { year, day } = Args::parse();
    println!("year: {}  day: {}\n", year, day);

    let aoc = [(2018, aoc2018::DAYS), (2022, aoc2022::DAYS)]
        .into_iter()
        .map(|(y, r)| (y, (1..).zip(r).collect::<BTreeMap<_, _>>()))
        .collect::<BTreeMap<_, _>>();
    match aoc.get(&year).and_then(|r| r.get(&day)) {
        Some(run) => match Input::new(year, day) {
            Ok(input) => run(input),
            Err(err) => println!("Couldn't load input: {err}"),
        },
        None => {
            println!("Year or day not found, available:");
            aoc.iter().for_each(|(y, r)| {
                println!("{y}");
                (1..=r.len()).for_each(|d| print!(" {d}"));
                println!();
            })
        }
    }
}
