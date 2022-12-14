mod aoc2018;

use clap::Parser;
use std::collections::HashMap;
use std::num::NonZeroU8;
use std::path::Path;
use std::str::FromStr;
use std::{fmt, fs, io};

pub struct Input {
    pub data: String,
}

impl Input {
    fn new(AoC { year, day }: AoC) -> io::Result<Self> {
        Ok(Self {
            data: fs::read_to_string(format!("inputs/{year}/{day}"))?,
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

fn main() {
    println!("Advent of Code manager");
    println!("----------------------");
    let args = AoC::parse();

    let aoc_years = HashMap::from([(2018, aoc2018::RUN)]);
    match Input::new(args) {
        Ok(input) => {
            println!("  year: {}  day: {}\n", args.year, args.day);
            aoc_years[&args.year][args.day.get() as usize - 1](input);
        }
        _ => {
            println!("\nYear or day not found, available:");
            available();
        }
    }
}

#[derive(Debug, Copy, Clone, clap::Parser)]
struct AoC {
    year: u16,
    day: NonZeroU8,
}

fn available() {
    let entries = |x: &dyn AsRef<Path>| {
        let mut entries = fs::read_dir(x.as_ref())
            .unwrap()
            .filter_map(|d| d.ok())
            .filter(|d| !d.file_name().to_string_lossy().starts_with("."))
            .collect::<Vec<_>>();
        entries.sort_unstable_by_key(|d| d.file_name());
        entries
    };
    entries(&"inputs").iter().for_each(|d| {
        println!("{}", d.file_name().to_string_lossy());
        entries(&d.path())
            .iter()
            .for_each(|d| print!(" {}", d.file_name().to_string_lossy()));
        println!()
    });
}
