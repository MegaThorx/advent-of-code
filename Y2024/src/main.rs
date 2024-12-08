use tabled::Table;
use std::env;
use crate::solutions::Solutions;

pub mod solutions;

fn main() {
    let mut arguments = env::args();
    let path = arguments.nth(1).unwrap();

    let results = aoc::run_all(path, 2024, Box::new(Solutions{ }));

    let table = Table::new(results);
    println!("{}", table);
}
