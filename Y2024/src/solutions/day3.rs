use aoc::Solver;
use regex::{Match, Regex};

pub struct Day3 {}

impl Solver for Day3 {
    fn solve_first(&self, lines: Vec<String>) -> String {
        let input = lines.join("");
        let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut result: i64 = 0;

        for capture in regex.captures_iter(&input) {
            result += capture.get(1).unwrap().as_str().parse::<i64>().unwrap() *
                capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
        }

        result.to_string()
    }

    fn solve_second(&self, lines: Vec<String>) -> String {
        let input = lines.join("");
        let regex = Regex::new(r"((mul)\((\d+),(\d+)\))|((don't)\(\))|((do)\(\))").unwrap();
        let mut mul_enabled = true;
        let mut result: i64 = 0;

        for capture in regex.captures_iter(&input) {
            let mut action = "";

            if capture.get(2).is_some() {
                action = capture.get(2).unwrap().as_str();
            } else if capture.get(6).is_some() {
                action = capture.get(6).unwrap().as_str();
            } else if capture.get(8).is_some() {
                action = capture.get(8).unwrap().as_str();
            }

            match action {
                "mul" => {
                    if mul_enabled {
                        result += capture.get(3).unwrap().as_str().parse::<i64>().unwrap() *
                            capture.get(4).unwrap().as_str().parse::<i64>().unwrap();
                    }
                },
                "don't" => {
                    mul_enabled = false;
                },
                "do" => {
                    mul_enabled = true;
                },
                _ => {
                    unreachable!()
                }
            }
        }

        result.to_string()
    }
}