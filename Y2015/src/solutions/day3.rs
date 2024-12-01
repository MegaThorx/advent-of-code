use aoc::Solver;

pub struct Day3 {}

impl Solver for Day3 {
    fn solve_first(&self, lines: Vec<String>) -> String {
        let actions = lines.first().unwrap().chars().collect::<Vec<char>>();
        let (mut x, mut y) = (0, 0);
        let mut visited_positions: Vec<(i64, i64)> = vec![(0, 0)];
        let mut result: i64 = 1;

        for action in actions {
            let movement = match action {
                '^' => (1, 0),
                'v' => (-1, 0),
                '>' => (0, 1),
                '<' => (0, -1),
                _ => (0, 0),
            };

            x += movement.0;
            y += movement.1;

            if !visited_positions.contains(&(x, y)) {
                visited_positions.push((x, y));
                result += 1;
            }
        }

        result.to_string()
    }

    fn solve_second(&self, lines: Vec<String>) -> String {
        let actions = lines.first().unwrap().chars().collect::<Vec<char>>();
        let (mut santa_x, mut santa_y) = (0, 0);
        let (mut robo_x, mut robo_y) = (0, 0);
        let mut visited_positions: Vec<(i64, i64)> = vec![(0, 0)];
        let mut result: i64 = 1;
        let mut santa = true;

        for action in actions {
            let movement = match action {
                '^' => (1, 0),
                'v' => (-1, 0),
                '>' => (0, 1),
                '<' => (0, -1),
                _ => (0, 0),
            };

            let x;
            let y;

            if santa {
                santa_x += movement.0;
                santa_y += movement.1;
                (x, y) = (santa_x, santa_y);
            } else {
                robo_x += movement.0;
                robo_y += movement.1;
                (x, y) = (robo_x, robo_y);
            }

            if !visited_positions.contains(&(x, y)) {
                visited_positions.push((x, y));
                result += 1;
            }

            santa = !santa;
        }

        result.to_string()
    }
}