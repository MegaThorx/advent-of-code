use aoc::{Solver, SolverFactory};
use crate::solutions::day1::Day1;
use crate::solutions::day2::Day2;
use crate::solutions::day3::Day3;
use crate::solutions::day4::Day4;
use crate::solutions::day5::Day5;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub struct Solutions {}

impl SolverFactory for Solutions {
    fn get(&self, day: i32) -> Option<Box<(dyn Solver)>> {
        match day {
            1 => Some(Box::new(Day1 {})),
            2 => Some(Box::new(Day2 {})),
            3 => Some(Box::new(Day3 {})),
            4 => Some(Box::new(Day4 {})),
            5 => Some(Box::new(Day5 {})),
            _ => None,
        }
    }
}