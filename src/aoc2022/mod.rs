mod day1;
mod day2;
mod day3;

use crate::Input;

pub const RUN: &[fn(Input)] = &[day1::run, day2::run, day3::run];
