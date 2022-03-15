use std::str::FromStr;
use std::{fmt, fs};

pub fn input(day: u8) -> String {
    fs::read_to_string(format!("inputs/input{}.txt", day)).unwrap()
}

pub fn input_parse<T: FromStr>(day: u8) -> Vec<T>
where
    <T as FromStr>::Err: fmt::Debug,
{
    input(day)
        .lines()
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}
