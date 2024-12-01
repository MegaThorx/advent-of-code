use aoc::Solver;
use tabled::Table;
use crate::solutions::day1::Day1;

pub mod solutions;

fn main() {
    let mut results: Vec<aoc::Result> = Vec::new();

    for day in 1..25 {
        let solver = get_solver(day);

        if solver.is_none() {
            continue;
        }

        let solver = solver.unwrap();

        let result = aoc::run(&solver, day, "example");
        results.push(result);
        let result = aoc::run(&solver, day, "input");
        results.push(result);
    }

    let table = Table::new(results);
    println!("{}", table);
}

fn get_solver(day: i32) -> Option<Box<dyn Solver>> {
    match day {
        1 => Some(Box::new(Day1 {})),
        _ => None,
    }
}