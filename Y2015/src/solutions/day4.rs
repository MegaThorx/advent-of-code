use aoc::Solver;

pub struct Day4 {}

impl Solver for Day4 {
    fn solve_first(&self, lines: Vec<String>) -> String {
        let secret = &lines[0];
        let mut number = 1;

        loop {
            if format!("{:x}", md5::compute(format!("{}{}", secret, number))).starts_with("00000") {
                break;
            }
            number += 1;
        }

        number.to_string()
    }

    fn solve_second(&self, lines: Vec<String>) -> String {
        let secret = &lines[0];
        let mut number = 1;

        loop {
            if format!("{:x}", md5::compute(format!("{}{}", secret, number))).starts_with("000000") {
                break;
            }
            number += 1;
        }

        number.to_string()
    }
}