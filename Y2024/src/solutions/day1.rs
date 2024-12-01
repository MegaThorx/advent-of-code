use aoc::Solver;

pub struct Day1 {}

impl Day1 {
    fn parse(lines: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
        let mut numbers_left: Vec<i64> = Vec::new();
        let mut numbers_right: Vec<i64> = Vec::new();

        for line in lines {
            let parts = line.split("   ").collect::<Vec<&str>>();

            let left = parts[0].parse::<i64>().expect("Unable to parse left number");
            let right = parts[1].parse::<i64>().expect("Unable to parse right number");

            numbers_left.push(left);
            numbers_right.push(right);
        }

        (numbers_left, numbers_right)
    }
}

impl Solver for Day1 {
    fn solve_first(&self, lines: Vec<String>) -> String {
        let (mut numbers_left, mut numbers_right) = Day1::parse(&lines);

        numbers_left.sort();
        numbers_right.sort();

        let mut result: i64 = 0;

        for index in 0..numbers_left.len()
        {
            result += (numbers_left[index] - numbers_right[index]).abs();
        }

        result.to_string()
    }

    fn solve_second(&self, lines: Vec<String>) -> String {
        let (mut numbers_left, mut numbers_right) = Day1::parse(&lines);

        numbers_left.sort();
        numbers_right.sort();

        let mut result: i64 = 0;

        for index in 0..numbers_left.len() {
            let mut count = 0;

            for index_inner in 0..numbers_left.len() {
                if numbers_left[index] == numbers_right[index_inner] {
                    count += 1;
                }
            }

            result += numbers_left[index] * count;
        }

        result.to_string()
    }
}