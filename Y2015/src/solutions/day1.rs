use aoc::Solver;

pub struct Day1 {}

impl Solver for Day1 {
    fn solve_first(&self, lines: Vec<String>) -> String {
        let mut floor: i64 = 0;

        for character in lines[0].chars() {
            floor += match character {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            }
        }

        floor.to_string()
    }

    fn solve_second(&self, lines: Vec<String>) -> String {
        let mut floor: i64 = 0;
        let mut index: i64 = 1;
        let mut solution: i64 = -1;


        for character in lines[0].chars() {
            floor += match character {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            };

            if floor == -1 {
                solution = index;
                break;
            }

            index += 1;
        }

        solution.to_string()
    }
}