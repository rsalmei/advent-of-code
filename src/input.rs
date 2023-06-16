use std::str::FromStr;
use std::{fmt, fs, io};

pub struct Input {
    data: String,
}

impl Input {
    pub fn new(year: u16, day: u8) -> io::Result<Self> {
        Ok(Self {
            data: fs::read_to_string(format!("src/aoc{year}/inputs/{day}"))?,
        })
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.data.into_bytes()
    }

    pub fn map_lines<'a, T, F: Fn(&'a str) -> T>(&'a self, f: F) -> Vec<T> {
        self.data.lines().map(f).collect()
    }

    pub fn as_lines(&self) -> Vec<&str> {
        self.map_lines(|s| s)
    }

    pub fn parse_lines<T: FromStr, U, F: Fn(Result<T, T::Err>) -> U>(&self, f: F) -> Vec<U> {
        self.map_lines(|s| f(s.parse()))
    }

    pub fn as_type<T: FromStr>(&self) -> Vec<T>
    where
        T::Err: fmt::Debug,
    {
        self.parse_lines(|x| x.unwrap())
    }

    pub fn as_optional<T: FromStr>(&self) -> Vec<Option<T>> {
        self.parse_lines(|x| x.ok())
    }
}
