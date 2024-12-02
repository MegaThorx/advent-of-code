use regex::Regex;
use aoc::Solver;

pub struct Day6 {}

impl Solver for Day6 {
    fn solve_first(&self, lines: Vec<String>) -> String {
        let mut grid = vec![vec![false; 1000]; 1000];
        let regex = Regex::new(r"(?P<action>turn on|toggle|turn off) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)").unwrap();

        for line in lines {
            let captures = regex.captures(&line).unwrap();

            let x1 = captures["x1"].parse::<usize>().unwrap();
            let y1 = captures["y1"].parse::<usize>().unwrap();
            let x2 = captures["x2"].parse::<usize>().unwrap();
            let y2 = captures["y2"].parse::<usize>().unwrap();

            match &captures["action"] {
                "turn on" => {
                    for x in x1..x2+1 {
                        for y in y1..y2+1 {
                            grid[x][y] = true;
                        }
                    }
                },
                "turn off" => {
                    for x in x1..x2+1 {
                        for y in y1..y2+1 {
                            grid[x][y] = false;
                        }
                    }
                },
                "toggle" => {
                    for x in x1..x2+1 {
                        for y in y1..y2+1 {
                            grid[x][y] = !grid[x][y];
                        }
                    }
                },
                _ => unreachable!()
            }
        }

        let mut on_count = 0;

        grid.iter().for_each(|inner| inner.iter().for_each(|state| if *state { on_count += 1 }));

        on_count.to_string()
    }

    fn solve_second(&self, lines: Vec<String>) -> String {
        let mut grid = vec![vec![0; 1000]; 1000];
        let regex = Regex::new(r"(?P<action>turn on|toggle|turn off) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)").unwrap();

        for line in lines {
            let captures = regex.captures(&line).unwrap();

            let x1 = captures["x1"].parse::<usize>().unwrap();
            let y1 = captures["y1"].parse::<usize>().unwrap();
            let x2 = captures["x2"].parse::<usize>().unwrap();
            let y2 = captures["y2"].parse::<usize>().unwrap();

            match &captures["action"] {
                "turn on" => {
                    for x in x1..x2+1 {
                        for y in y1..y2+1 {
                            grid[x][y] += 1;
                        }
                    }
                },
                "turn off" => {
                    for x in x1..x2+1 {
                        for y in y1..y2+1 {
                            grid[x][y] -= 1;

                            if grid[x][y] < 0 {
                                grid[x][y] = 0;
                            }
                        }
                    }
                },
                "toggle" => {
                    for x in x1..x2+1 {
                        for y in y1..y2+1 {
                            grid[x][y] += 2;
                        }
                    }
                },
                _ => unreachable!()
            }
        }

        let mut total_brightness = 0;

        grid.iter().for_each(|inner| inner.iter().for_each(|state| total_brightness += state ));

        total_brightness.to_string()
    }
}