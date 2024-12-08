use aoc::{Solver, SolverFactory};
use crate::solutions::day1::Day1;
use crate::solutions::day2::Day2;
use crate::solutions::day3::Day3;

pub mod day1;
pub mod day2;
pub mod day3;
pub struct Solutions {}

impl SolverFactory for Solutions {
    fn get(&self, day: i32) -> Option<Box<(dyn Solver)>> {
        match day {
            1 => Some(Box::new(Day1 {})),
            2 => Some(Box::new(Day2 {})),
            3 => Some(Box::new(Day3 {})),
            _ => None,
        }
    }
}
