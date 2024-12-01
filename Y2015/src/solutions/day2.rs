use aoc::Solver;

pub struct Day2 {}

impl Day2 {
    fn parse(lines: Vec<String>) -> Vec<(i64, i64, i64)> {
        let mut numbers: Vec<(i64, i64, i64)> = Vec::new();

        for line in lines {
            let parts = line.split('x').collect::<Vec<&str>>();
            numbers.push((parts[0].parse().unwrap(), parts[1].parse().unwrap(), parts[2].parse().unwrap()));
        }

        numbers
    }
}

impl Solver for Day2 {
    fn solve_first(&self, lines: Vec<String>) -> String {
        let numbers = Day2::parse(lines);
        let mut result: i64 = 0;

        for (length, width, height) in numbers {
            let mut sides = vec![length*width, width*height, height*length];
            sides.sort();

            result += sides.iter().sum::<i64>() * 2 + sides.first().unwrap();
        }

        result.to_string()
    }

    fn solve_second(&self, lines: Vec<String>) -> String {
        let numbers = Day2::parse(lines);
        let mut result: i64 = 0;

        for (length, width, height) in numbers {
            let mut sides = vec![length, width, height];
            sides.sort();

            result += sides[0] * 2 + sides[1] * 2 + length * width * height;
        }

        result.to_string()
    }
}