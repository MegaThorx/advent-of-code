use aoc::Solver;

pub struct Day2 {}

impl Day2 {
    fn parse(lines: &Vec<String>) -> Vec<Vec<i64>> {
        let mut rows: Vec<Vec<i64>> = Vec::new();

        for line in lines {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let mut numbers: Vec<i64> = Vec::new();

            for part in parts {
                numbers.push(part.parse::<i64>().unwrap());
            }

            rows.push(numbers);
        }

        rows
    }

    fn is_safe(numbers: &Vec<i64>) -> bool {
        let mut last_number = numbers[0];
        let increasing = numbers[0] < numbers[1];
        let mut invalid = false;

        for index in 1..numbers.len() {
            let number = numbers[index];

            if number == last_number ||
                (increasing && last_number > number) ||
                (!increasing && last_number < number) ||
                (number - last_number).abs() > 3{
                invalid = true;
                break;
            }

            last_number = number;
        }

        !invalid
    }
}

impl Solver for Day2 {
    fn solve_first(&self, lines: Vec<String>) -> String {
        let rows = Day2::parse(&lines);
        let mut result: i64 = 0;

        for row in rows {
            if Day2::is_safe(&row) {
                result += 1;
            }
        }

        result.to_string()
    }

    fn solve_second(&self, lines: Vec<String>) -> String {
        let rows = Day2::parse(&lines);
        let mut result: i64 = 0;

        for row in rows {
            if Day2::is_safe(&row) {
                result += 1;
            } else {
                if (0..row.len()).any(|index| {
                    let numbers = row
                        .clone()
                        .into_iter()
                        .enumerate()
                        .filter(|&(i, _)| index != i)
                        .map(|(_, e)| e)
                        .collect();

                    Self::is_safe(&numbers)
                }) {
                    result += 1;
                }
            }
        }

        result.to_string()
    }
}