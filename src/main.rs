mod aoc2018;
mod aoc2022;

use clap::Parser;
use std::path::Path;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::{fmt, fs, io};

pub struct Input {
    pub data: String,
}

impl Input {
    fn new(year: u16, day: u8) -> io::Result<Self> {
        Ok(Self {
            data: fs::read_to_string(format!("src/aoc{year}/inputs/{day}"))?,
        })
    }

    pub fn as_lines(&self) -> Vec<&str> {
        self.data.lines().collect()
    }

    pub fn parse_lines<T: FromStr>(&self) -> Vec<T>
    where
        <T as FromStr>::Err: fmt::Debug,
    {
        self.as_lines().iter().map(|s| s.parse().unwrap()).collect()
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
    let args = Args::parse();
    println!("year: {}  day: {}\n", args.year, args.day);

    let aoc_years = HashMap::from([(2018, aoc2018::RUN), (2022, aoc2022::RUN)]);
    match Input::new(args) {
        Ok(input) => {
            aoc_years[&args.year][args.day as usize - 1](input);
        }
        _ => {
            println!("Year or day not found, available:");
            available();
        }
    }
}

fn available() {
    let entries = |x: &dyn AsRef<Path>| {
        let mut entries = fs::read_dir(x.as_ref())
            .unwrap()
            .filter_map(|d| d.ok())
            .filter(|d| !d.file_name().to_str().unwrap().starts_with('.'))
            .collect::<Vec<_>>();
        entries.sort_unstable_by_key(|d| d.file_name());
        entries
    };
    entries(&"inputs").iter().for_each(|d| {
        println!("{}", d.file_name().to_str().unwrap());
        entries(&d.path())
            .iter()
            .for_each(|d| print!(" {}", d.file_name().to_str().unwrap()));
        println!()
    });
}
