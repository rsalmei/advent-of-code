mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use crate::Input;

pub const RUN: &[fn(Input)] = &[day1::run, day2::run, day3::run, day4::run, day5::run];
