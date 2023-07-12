use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// The years I've already started.
const YEARS: &[u16] = &[2018, 2022];

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("aoc-auto-detect.rs");
    let mut aoc_runners = File::create(dest_path)?;
    writeln!(&mut aoc_runners, "[",)?;
    YEARS
        .iter()
        .for_each(|&year| collect_year(year, &mut aoc_runners).unwrap());
    writeln!(&mut aoc_runners, "]",)?;
    Ok(())
}

fn collect_year(year: u16, file: &mut File) -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/aoc{year}/mod.rs");
    let contents = fs::read_to_string(&format!("src/aoc{year}/mod.rs"))?;
    writeln!(file, r"({year}, &[",)?;
    let days = contents.lines().map(|s| {
        s.trim_start_matches("pub mod day")
            .trim_end_matches(';')
            .parse::<u8>()
            .unwrap()
    });
    for day in 1..=days.max().unwrap() {
        writeln!(file, r"aoc{year}::day{day}::run,",)?;
    }
    writeln!(file, r"]),",)?;
    Ok(())
}
