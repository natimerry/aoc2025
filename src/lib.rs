use std::time::Instant;

pub mod utils;
pub mod day1;
pub mod day2;
pub mod day3;


#[macro_use]
extern crate aoc;

pub mod register {
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
