use tabled::Table;
use crate::solutions::Solutions;

pub mod solutions;

fn main() {
    let results = aoc::run_all(Box::new(Solutions{ }));

    let table = Table::new(results);
    println!("{}", table);
}
