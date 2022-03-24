use std::str::FromStr;
use std::{fmt, fs};

pub fn input(day: u8) -> String {
    fs::read_to_string(format!("inputs/input{}.txt", day)).unwrap()
}

pub fn input_lines(day: u8) -> Vec<String> {
    input(day).lines().map(ToOwned::to_owned).collect()
}

pub fn input_parse<T: FromStr>(day: u8) -> Vec<T>
where
    <T as FromStr>::Err: fmt::Debug,
{
    input(day).lines().map(|s| s.parse().unwrap()).collect()
}
