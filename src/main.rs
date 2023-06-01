mod aoc2018;
mod aoc2022;

use clap::Parser;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::{fmt, fs, io};

pub struct Input {
    data: String,
}

impl Input {
    fn new(year: u16, day: u8) -> io::Result<Self> {
        Ok(Self {
            data: fs::read_to_string(format!("src/aoc{year}/inputs/{day}"))?,
        })
    }

    pub fn get_as<'a, T, F: Fn(&'a str) -> T>(&'a self, f: F) -> Vec<T> {
        self.data.lines().map(f).collect()
    }

    pub fn as_lines(&self) -> Vec<&str> {
        self.get_as(|s| s)
    }

    pub fn as_parse<T: FromStr, U, F: Fn(Result<T, T::Err>) -> U>(&self, f: F) -> Vec<U> {
        self.get_as(|s| f(s.parse()))
    }

    pub fn as_parse_type<T: FromStr>(&self) -> Vec<T>
    where
        T::Err: fmt::Debug,
    {
        self.as_parse(|x| x.unwrap())
    }

    pub fn as_parse_optional<T: FromStr>(&self) -> Vec<Option<T>> {
        self.as_parse(|x| x.ok())
    }
}

#[derive(Debug, Copy, Clone, clap::Parser)]
struct Args {
    #[arg(default_value_t = 0)]
    year: u16,
    #[arg(default_value_t = 0)]
    day: u8,
}

fn main() {
    println!("My Advent of Code manager");
    println!("-------------------------");
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
