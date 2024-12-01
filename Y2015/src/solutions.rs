use aoc::{Solver, SolverFactory};
use crate::solutions::day1::Day1;

pub mod day1;

pub struct Solutions {}

impl SolverFactory for Solutions {
    fn get(&self, day: i32) -> Option<Box<(dyn Solver)>> {
        match day {
            1 => Some(Box::new(Day1 {})),
            _ => None,
        }
    }
}