mod aoc2018;

use clap::Parser;
use std::path::Path;
use std::str::FromStr;
use std::{fmt, fs, io};

pub struct Input {
    pub data: String,
}

impl Input {
    fn new(AoC { year, day }: AoC) -> io::Result<Self> {
        Ok(Self {
            data: fs::read_to_string(format!("inputs/{year}/day{day}.txt"))?,
        })
    }

    pub fn as_lines(&self) -> Vec<String> {
        self.data.lines().map(ToOwned::to_owned).collect()
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
    match (Input::new(args), args.year) {
        (Ok(input), 2018) => {
            println!("  year: {}  day: {}\n", args.year, args.day);
            aoc2018::RUN[args.day as usize - 1](input);
        }
        _ => {
            println!("Year or day not found, available:");
            available();
        }
    }
}

#[derive(Debug, Copy, Clone, clap::Parser)]
struct AoC {
    year: u16,
    day: u8,
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
        entries(&d.path()).iter().for_each(|d| {
            let n = d.file_name().to_string_lossy().chars().nth(3).unwrap();
            print!(" {n}")
        });
    });
    println!()
}
