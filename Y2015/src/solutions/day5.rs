use aoc::Solver;

pub struct Day5 {}

impl Solver for Day5 {
    fn solve_first(&self, lines: Vec<String>) -> String {
        let mut result: i64 = 0;
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let forbidden_strings = ["ab", "cd", "pq", "xy"];

        for line in lines {
            let mut vowels_count = 0;
            let mut double_count = 0;
            let mut forbidden_count = 0;
            let mut previous: Option<char> = None;

            for character in line.chars() {
                if vowels.contains(&character) {
                    vowels_count += 1;
                }

                if previous == Some(character) {
                    double_count += 1;
                }

                for forbidden_string in forbidden_strings {
                    let chars = forbidden_string.chars().collect::<Vec<char>>();

                    if previous == Some(chars[0]) && character == chars[1] {
                        forbidden_count += 1;
                    }
                }

                previous = Some(character);
            }

            if vowels_count >= 3 && double_count >= 1 && forbidden_count == 0 {
                result += 1;
            }
        }

        result.to_string()
    }

    fn solve_second(&self, lines: Vec<String>) -> String {
        let mut result: i64 = 0;

        for line in lines {
            let mut spaced_double_count = 0;
            let mut contains_pattern_count = 0;

            for i in 1..line.len()-1 {
                let character_previous = line.chars().nth(i-1).unwrap();
                let character = line.chars().nth(i).unwrap();
                let character_next = line.chars().nth(i+1).unwrap();

                if character_previous == character_next {
                    spaced_double_count += 1;
                }

                for j in i+1..line.len()-1 {
                    let character_future_1 = line.chars().nth(j).unwrap();
                    let character_future_2 = line.chars().nth(j+1).unwrap();

                    if character_previous == character_future_1 && character == character_future_2 {
                        contains_pattern_count += 1;
                    }
                }
            }

            if spaced_double_count >= 1 && contains_pattern_count >= 1 {
                result += 1;

            }
        }

        result.to_string()
    }
}