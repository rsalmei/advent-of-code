use std::str::FromStr;
use std::{fmt, fs, io};

pub fn input(day: u8) -> io::Result<String> {
    fs::read_to_string(format!("inputs/input{}.txt", day))
}

pub fn input_lines(day: u8) -> io::Result<Vec<String>> {
    Ok(input(day)?.lines().map(ToOwned::to_owned).collect())
}

pub fn input_parse<T: FromStr>(day: u8) -> io::Result<Vec<T>>
where
    <T as FromStr>::Err: fmt::Debug,
{
    Ok(input(day)?.lines().map(|s| s.parse().unwrap()).collect())
}
