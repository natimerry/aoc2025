pub mod day7;
pub mod day5;
use std::time::Instant;


pub mod utils;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day6;


#[macro_use]
extern crate aoc;

pub mod register {
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
